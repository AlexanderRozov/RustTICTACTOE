//! # Сетевой модуль для P2P игры
//! 
//! Этот модуль обеспечивает возможность игры в шашки по сети
//! с использованием P2P соединений через WebRTC.

use crate::checkers::{Checkers, CheckersMove, CheckersPlayer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// === СЕТЕВЫЕ СООБЩЕНИЯ ===

/// Типы сетевых сообщений
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// Приветствие и обмен информацией о соединении
    Hello {
        player_name: String,
        game_id: String,
        player_id: String,
    },
    /// Ход в игре
    GameMove {
        from: (usize, usize),
        to: (usize, usize),
        captures: Vec<(usize, usize)>,
        player: CheckersPlayer,
    },
    /// Обновление состояния игры
    GameState {
        board_state: String,
        current_player: CheckersPlayer,
        game_over: bool,
    },
    /// Запрос на начало новой игры
    NewGameRequest,
    /// Подтверждение новой игры
    NewGameAccepted,
    /// Отклонение новой игры
    NewGameDeclined,
    /// Сообщение чата
    ChatMessage {
        player_name: String,
        message: String,
        timestamp: u64,
    },
    /// Разрыв соединения
    Disconnect,
    /// Ping для проверки соединения
    Ping,
    /// Pong в ответ на ping
    Pong,
}

impl NetworkMessage {
    /// Сериализует сообщение в JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    /// Десериализует сообщение из JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// === СОСТОЯНИЕ СОЕДИНЕНИЯ ===

/// Статус сетевого соединения
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    /// Не подключен
    Disconnected,
    /// Подключение устанавливается
    Connecting,
    /// Подключен
    Connected,
    /// Ошибка подключения
    Error(String),
}

/// Информация о подключенном игроке
#[derive(Debug, Clone)]
pub struct ConnectedPlayer {
    pub id: String,
    pub name: String,
    pub is_host: bool,
    pub latency: Option<u64>, // в миллисекундах
}

// === P2P СОЕДИНЕНИЕ ===

/// Трейт для P2P соединения
pub trait P2PConnection: Send + Sync {
    /// Устанавливает соединение
    fn connect(&mut self) -> Result<(), String>;
    
    /// Отправляет сообщение
    fn send_message(&mut self, message: &NetworkMessage) -> Result<(), String>;
    
    /// Получает сообщение (блокирующий)
    fn receive_message(&mut self) -> Result<Option<NetworkMessage>, String>;
    
    /// Получает статус соединения
    fn get_status(&self) -> ConnectionStatus;
    
    /// Закрывает соединение
    fn disconnect(&mut self) -> Result<(), String>;
    
    /// Проверяет, активно ли соединение
    fn is_connected(&self) -> bool;
}

// === МЕНЕДЖЕР СЕТЕВОЙ ИГРЫ ===

/// Менеджер сетевой игры
pub struct NetworkGameManager {
    /// ID игры
    game_id: String,
    /// ID локального игрока
    local_player_id: String,
    /// Имя локального игрока
    local_player_name: String,
    /// Подключенные игроки
    connected_players: HashMap<String, ConnectedPlayer>,
    /// Статус соединения
    connection_status: ConnectionStatus,
    /// P2P соединение
    connection: Option<Box<dyn P2PConnection>>,
    /// Очередь исходящих сообщений
    outgoing_messages: Vec<NetworkMessage>,
    /// Очередь входящих сообщений
    incoming_messages: Vec<NetworkMessage>,
    /// Флаг хоста
    is_host: bool,
}

impl NetworkGameManager {
    /// Создает новый менеджер сетевой игры
    pub fn new(player_name: String) -> Self {
        Self {
            game_id: Uuid::new_v4().to_string(),
            local_player_id: Uuid::new_v4().to_string(),
            local_player_name: player_name,
            connected_players: HashMap::new(),
            connection_status: ConnectionStatus::Disconnected,
            connection: None,
            outgoing_messages: Vec::new(),
            incoming_messages: Vec::new(),
            is_host: false,
        }
    }
    
    /// Создает новую игру (становится хостом)
    pub fn create_game(&mut self) -> Result<(), String> {
        self.is_host = true;
        self.connection_status = ConnectionStatus::Connecting;
        
        // Добавляем себя в список игроков
        self.connected_players.insert(
            self.local_player_id.clone(),
            ConnectedPlayer {
                id: self.local_player_id.clone(),
                name: self.local_player_name.clone(),
                is_host: true,
                latency: None,
            },
        );
        
        Ok(())
    }
    
    /// Присоединяется к существующей игре
    pub fn join_game(&mut self, game_id: &str) -> Result<(), String> {
        self.game_id = game_id.to_string();
        self.is_host = false;
        self.connection_status = ConnectionStatus::Connecting;
        
        // Отправляем приветственное сообщение
        let hello = NetworkMessage::Hello {
            player_name: self.local_player_name.clone(),
            game_id: self.game_id.clone(),
            player_id: self.local_player_id.clone(),
        };
        self.outgoing_messages.push(hello);
        
        Ok(())
    }
    
    /// Отправляет ход в игре
    pub fn send_move(&mut self, game_move: CheckersMove, player: CheckersPlayer) -> Result<(), String> {
        let message = NetworkMessage::GameMove {
            from: game_move.from,
            to: game_move.to,
            captures: game_move.captures,
            player,
        };
        self.outgoing_messages.push(message);
        Ok(())
    }
    
    /// Отправляет сообщение чата
    pub fn send_chat_message(&mut self, message: &str) -> Result<(), String> {
        let chat_message = NetworkMessage::ChatMessage {
            player_name: self.local_player_name.clone(),
            message: message.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.outgoing_messages.push(chat_message);
        Ok(())
    }
    
    /// Обновляет состояние игры
    pub fn update_game_state(&mut self, game: &Checkers) -> Result<(), String> {
        let board_state = self.serialize_board(&game);
        let message = NetworkMessage::GameState {
            board_state,
            current_player: game.current_player,
            game_over: game.is_game_over(),
        };
        self.outgoing_messages.push(message);
        Ok(())
    }
    
    /// Обрабатывает входящие сообщения
    pub fn process_incoming_messages(&mut self) -> Vec<NetworkMessage> {
        let messages = self.incoming_messages.clone();
        self.incoming_messages.clear();
        messages
    }
    
    /// Получает исходящие сообщения
    pub fn get_outgoing_messages(&mut self) -> Vec<NetworkMessage> {
        let messages = self.outgoing_messages.clone();
        self.outgoing_messages.clear();
        messages
    }
    
    /// Добавляет исходящее сообщение
    pub fn add_outgoing_message(&mut self, message: NetworkMessage) {
        self.outgoing_messages.push(message);
    }
    
    /// Устанавливает статус соединения
    pub fn set_connection_status(&mut self, status: ConnectionStatus) {
        self.connection_status = status;
    }
    
    /// Получает статус соединения
    pub fn get_connection_status(&self) -> ConnectionStatus {
        self.connection_status.clone()
    }
    
    /// Проверяет, является ли локальный игрок хостом
    pub fn is_host(&self) -> bool {
        self.is_host
    }
    
    /// Получает ID игры
    pub fn get_game_id(&self) -> &str {
        &self.game_id
    }
    
    /// Получает список подключенных игроков
    pub fn get_connected_players(&self) -> Vec<ConnectedPlayer> {
        self.connected_players.values().cloned().collect()
    }
    
    /// Сериализует состояние доски
    fn serialize_board(&self, game: &Checkers) -> String {
        // Простая сериализация доски в строку
        // В реальной реализации здесь будет более эффективная сериализация
        format!("{:?}", game)
    }
    
    /// Обрабатывает входящее сообщение
    fn handle_message(&mut self, message: NetworkMessage) {
        match message {
            NetworkMessage::Hello { player_name, player_id, .. } => {
                // Добавляем нового игрока
                self.connected_players.insert(
                    player_id.clone(),
                    ConnectedPlayer {
                        id: player_id,
                        name: player_name,
                        is_host: false,
                        latency: None,
                    },
                );
            }
            NetworkMessage::Disconnect => {
                self.connection_status = ConnectionStatus::Disconnected;
            }
            _ => {
                // Добавляем сообщение в очередь входящих
                self.incoming_messages.push(message);
            }
        }
    }
}

// === ВЕБ-РЕАЛИЗАЦИЯ P2P СОЕДИНЕНИЯ ===

#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{
        RtcPeerConnection, RtcDataChannel, RtcSessionDescription, RtcIceCandidate,
        RtcDataChannelInit, RtcConfiguration, RtcOfferOptions,
    };
    use std::rc::Rc;
    use std::cell::RefCell;
    
    /// WebRTC P2P соединение для веб-версии
    pub struct WebRTCConnection {
        peer_connection: Option<RtcPeerConnection>,
        data_channel: Option<RtcDataChannel>,
        on_message: Option<Box<dyn Fn(NetworkMessage) + Send + Sync>>,
        status: ConnectionStatus,
    }
    
    impl WebRTCConnection {
        pub fn new() -> Self {
            Self {
                peer_connection: None,
                data_channel: None,
                on_message: None,
                status: ConnectionStatus::Disconnected,
            }
        }
        
        /// Устанавливает обработчик входящих сообщений
        pub fn set_message_handler(&mut self, handler: Box<dyn Fn(NetworkMessage) + Send + Sync>) {
            self.on_message = Some(handler);
        }
    }
    
    impl P2PConnection for WebRTCConnection {
        fn connect(&mut self) -> Result<(), String> {
            // Создаем RTCPeerConnection
            let config = RtcConfiguration::new();
            let peer_connection = RtcPeerConnection::new_with_configuration(&config)
                .map_err(|e| format!("Failed to create RTCPeerConnection: {:?}", e))?;
            
            // Создаем DataChannel
            let data_channel_init = RtcDataChannelInit::new();
            let data_channel = peer_connection
                .create_data_channel_with_data_channel_dict("game", &data_channel_init)
                .map_err(|e| format!("Failed to create DataChannel: {:?}", e))?;
            
            // Настраиваем обработчики событий
            let on_message = self.on_message.clone();
            let on_message_callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                if let Some(data) = event.data().as_string() {
                    if let Ok(message) = NetworkMessage::from_json(&data) {
                        if let Some(handler) = &on_message {
                            handler(message);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            data_channel.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
            on_message_callback.forget(); // Предотвращаем очистку памяти
            
            self.peer_connection = Some(peer_connection);
            self.data_channel = Some(data_channel);
            self.status = ConnectionStatus::Connected;
            
            Ok(())
        }
        
        fn send_message(&mut self, message: &NetworkMessage) -> Result<(), String> {
            if let Some(data_channel) = &self.data_channel {
                let json = message.to_json()
                    .map_err(|e| format!("Failed to serialize message: {}", e))?;
                data_channel.send_with_str(&json)
                    .map_err(|e| format!("Failed to send message: {:?}", e))?;
                Ok(())
            } else {
                Err("DataChannel not initialized".to_string())
            }
        }
        
        fn receive_message(&mut self) -> Result<Option<NetworkMessage>, String> {
            // В WebRTC сообщения обрабатываются асинхронно через callback
            // Этот метод возвращает None, так как сообщения уже обработаны
            Ok(None)
        }
        
        fn get_status(&self) -> ConnectionStatus {
            self.status.clone()
        }
        
        fn disconnect(&mut self) -> Result<(), String> {
            if let Some(peer_connection) = &self.peer_connection {
                peer_connection.close();
            }
            self.status = ConnectionStatus::Disconnected;
            Ok(())
        }
        
        fn is_connected(&self) -> bool {
            matches!(self.status, ConnectionStatus::Connected)
        }
    }
}

// === НАТИВНАЯ РЕАЛИЗАЦИЯ P2P СОЕДИНЕНИЯ ===

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::net::SocketAddr;
    
    /// TCP P2P соединение для нативной версии
    pub struct TCPConnection {
        listener: Option<TcpListener>,
        stream: Option<TcpStream>,
        status: ConnectionStatus,
        local_addr: Option<SocketAddr>,
    }
    
    impl TCPConnection {
        pub fn new() -> Self {
            Self {
                listener: None,
                stream: None,
                status: ConnectionStatus::Disconnected,
                local_addr: None,
            }
        }
        
        /// Слушает входящие соединения
        pub async fn listen(&mut self, addr: &str) -> Result<(), String> {
            let listener = TcpListener::bind(addr)
                .await
                .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;
            
            self.local_addr = Some(listener.local_addr()
                .map_err(|e| format!("Failed to get local address: {}", e))?);
            
            self.listener = Some(listener);
            self.status = ConnectionStatus::Connecting;
            
            Ok(())
        }
        
        /// Принимает входящее соединение
        pub async fn accept(&mut self) -> Result<(), String> {
            if let Some(listener) = &mut self.listener {
                let (stream, _) = listener.accept()
                    .await
                    .map_err(|e| format!("Failed to accept connection: {}", e))?;
                
                self.stream = Some(stream);
                self.status = ConnectionStatus::Connected;
                Ok(())
            } else {
                Err("Listener not initialized".to_string())
            }
        }
        
        /// Подключается к удаленному адресу
        pub async fn connect_to(&mut self, addr: &str) -> Result<(), String> {
            let stream = TcpStream::connect(addr)
                .await
                .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;
            
            self.stream = Some(stream);
            self.status = ConnectionStatus::Connected;
            Ok(())
        }
    }
    
    impl P2PConnection for TCPConnection {
        fn connect(&mut self) -> Result<(), String> {
            // Для TCP соединения нужно сначала вызвать listen() или connect_to()
            Err("Use listen() or connect_to() for TCP connections".to_string())
        }
        
        fn send_message(&mut self, message: &NetworkMessage) -> Result<(), String> {
            if let Some(stream) = &mut self.stream {
                let json = message.to_json()
                    .map_err(|e| format!("Failed to serialize message: {}", e))?;
                let data = format!("{}\n", json);
                
                // В реальной реализации здесь будет асинхронная отправка
                // Для простоты используем синхронную версию
                let data_bytes = data.as_bytes().to_vec();
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async {
                                        let mut stream = stream.try_clone()
                    .map_err(|e| format!("Failed to clone stream: {}", e))?;
                        
                        stream.write_all(&data_bytes)
                            .await
                            .map_err(|e| format!("Failed to write to stream: {}", e))?;
                        
                        Ok(())
                    })
            } else {
                Err("Stream not initialized".to_string())
            }
        }
        
        fn receive_message(&mut self) -> Result<Option<NetworkMessage>, String> {
            if let Some(stream) = &mut self.stream {
                // В реальной реализации здесь будет асинхронное чтение
                // Для простоты возвращаем None
                Ok(None)
            } else {
                Err("Stream not initialized".to_string())
            }
        }
        
        fn get_status(&self) -> ConnectionStatus {
            self.status.clone()
        }
        
        fn disconnect(&mut self) -> Result<(), String> {
            if let Some(stream) = &mut self.stream {
                let _ = stream.shutdown();
            }
            self.status = ConnectionStatus::Disconnected;
            Ok(())
        }
        
        fn is_connected(&self) -> bool {
            matches!(self.status, ConnectionStatus::Connected)
        }
    }
}

// === ЭКСПОРТ МОДУЛЯ ===

#[cfg(target_arch = "wasm32")]
pub use web::WebRTCConnection as P2PConnectionImpl;

#[cfg(not(target_arch = "wasm32"))]
pub use native::TCPConnection as P2PConnectionImpl;
