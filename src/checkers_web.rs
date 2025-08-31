//! # Веб-версия шашек с сетевым функционалом
//! 
//! Этот модуль обеспечивает интеграцию шашек с веб-интерфейсом
//! и сетевым функционалом через WebRTC.

use crate::checkers::{Checkers, CheckersMove, CheckersPlayer};
use crate::network::{NetworkGameManager, NetworkMessage, ConnectionStatus, ConnectedPlayer};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// === WASM ЭКСПОРТ ===

/// Экспорт менеджера сетевой игры для JavaScript
#[wasm_bindgen]
pub struct NetworkGameManagerWasm {
    manager: Rc<RefCell<NetworkGameManager>>,
}

#[wasm_bindgen]
impl NetworkGameManagerWasm {
    /// Создает новый менеджер сетевой игры
    #[wasm_bindgen(constructor)]
    pub fn new(player_name: String) -> Self {
        Self {
            manager: Rc::new(RefCell::new(NetworkGameManager::new(player_name))),
        }
    }
    
    /// Создает новую игру (становится хостом)
    pub fn create_game(&self) -> Result<(), JsValue> {
        self.manager
            .borrow_mut()
            .create_game()
            .map_err(|e| JsValue::from_str(&e))
    }
    
    /// Присоединяется к существующей игре
    pub fn join_game(&self, game_id: &str) -> Result<(), JsValue> {
        self.manager
            .borrow_mut()
            .join_game(game_id)
            .map_err(|e| JsValue::from_str(&e))
    }
    
    /// Отправляет ход в игре
    pub fn send_move(&self, from_row: usize, from_col: usize, to_row: usize, to_col: usize, player: u8) -> Result<(), JsValue> {
        let player_enum = match player {
            0 => CheckersPlayer::White,
            1 => CheckersPlayer::Black,
            _ => return Err(JsValue::from_str("Неверный тип игрока")),
        };
        
        let game_move = CheckersMove::new(from_row, from_col, to_row, to_col);
        
        self.manager
            .borrow_mut()
            .send_move(game_move, player_enum)
            .map_err(|e| JsValue::from_str(&e))
    }
    
    /// Отправляет сообщение чата
    pub fn send_chat_message(&self, message: &str) -> Result<(), JsValue> {
        self.manager
            .borrow_mut()
            .send_chat_message(message)
            .map_err(|e| JsValue::from_str(&e))
    }
    
    /// Обновляет состояние игры
    pub fn update_game_state(&self, game: &Checkers) -> Result<(), JsValue> {
        self.manager
            .borrow_mut()
            .update_game_state(game)
            .map_err(|e| JsValue::from_str(&e))
    }
    
    /// Получает статус соединения
    pub fn get_connection_status(&self) -> String {
        match self.manager.borrow().get_connection_status() {
            ConnectionStatus::Connected => "connected".to_string(),
            ConnectionStatus::Connecting => "connecting".to_string(),
            ConnectionStatus::Disconnected => "disconnected".to_string(),
            ConnectionStatus::Error(msg) => format!("error: {}", msg),
        }
    }
    
    /// Проверяет, является ли локальный игрок хостом
    pub fn is_host(&self) -> bool {
        self.manager.borrow().is_host()
    }
    
    /// Получает ID игры
    pub fn get_game_id(&self) -> String {
        self.manager.borrow().get_game_id().to_string()
    }
    
    /// Получает список подключенных игроков
    pub fn get_connected_players(&self) -> JsValue {
        let players = self.manager.borrow().get_connected_players();
        let players_array = js_sys::Array::new();
        
        for player in players {
            let player_obj = js_sys::Object::new();
            js_sys::Reflect::set(&player_obj, &"id".into(), &player.id.into()).unwrap();
            js_sys::Reflect::set(&player_obj, &"name".into(), &player.name.into()).unwrap();
            js_sys::Reflect::set(&player_obj, &"isHost".into(), &player.is_host.into()).unwrap();
            if let Some(latency) = player.latency {
                js_sys::Reflect::set(&player_obj, &"latency".into(), &latency.into()).unwrap();
            }
            players_array.push(&player_obj);
        }
        
        players_array.into()
    }
    
    /// Обрабатывает входящие сообщения
    pub fn process_incoming_messages(&self) -> JsValue {
        let messages = self.manager.borrow_mut().process_incoming_messages();
        let messages_array = js_sys::Array::new();
        
        for message in messages {
            let message_obj = js_sys::Object::new();
            match message {
                NetworkMessage::Hello { player_name, game_id, player_id } => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"hello".into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"playerName".into(), &player_name.into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"gameId".into(), &game_id.into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"playerId".into(), &player_id.into()).unwrap();
                }
                NetworkMessage::GameMove { from, to, captures, player } => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"gameMove".into()).unwrap();
                    let from_array = js_sys::Array::new();
                    from_array.push(&from.0.into());
                    from_array.push(&from.1.into());
                    js_sys::Reflect::set(&message_obj, &"from".into(), &from_array.into()).unwrap();
                    
                    let to_array = js_sys::Array::new();
                    to_array.push(&to.0.into());
                    to_array.push(&to.1.into());
                    js_sys::Reflect::set(&message_obj, &"to".into(), &to_array.into()).unwrap();
                    
                    let captures_array = js_sys::Array::new();
                    for capture in captures {
                        let capture_array = js_sys::Array::new();
                        capture_array.push(&capture.0.into());
                        capture_array.push(&capture.1.into());
                        captures_array.push(&capture_array);
                    }
                    js_sys::Reflect::set(&message_obj, &"captures".into(), &captures_array.into()).unwrap();
                    
                    let player_value = match player {
                        CheckersPlayer::White => 0,
                        CheckersPlayer::Black => 1,
                    };
                    js_sys::Reflect::set(&message_obj, &"player".into(), &player_value.into()).unwrap();
                }
                NetworkMessage::GameState { board_state, current_player, game_over } => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"gameState".into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"boardState".into(), &board_state.into()).unwrap();
                    let player_value = match current_player {
                        CheckersPlayer::White => 0,
                        CheckersPlayer::Black => 1,
                    };
                    js_sys::Reflect::set(&message_obj, &"currentPlayer".into(), &player_value.into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"gameOver".into(), &game_over.into()).unwrap();
                }
                NetworkMessage::ChatMessage { player_name, message, timestamp } => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"chatMessage".into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"playerName".into(), &player_name.into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"message".into(), &message.into()).unwrap();
                    js_sys::Reflect::set(&message_obj, &"timestamp".into(), &timestamp.into()).unwrap();
                }
                NetworkMessage::NewGameRequest => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"newGameRequest".into()).unwrap();
                }
                NetworkMessage::NewGameAccepted => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"newGameAccepted".into()).unwrap();
                }
                NetworkMessage::NewGameDeclined => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"newGameDeclined".into()).unwrap();
                }
                NetworkMessage::Disconnect => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"disconnect".into()).unwrap();
                }
                NetworkMessage::Ping => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"ping".into()).unwrap();
                }
                NetworkMessage::Pong => {
                    js_sys::Reflect::set(&message_obj, &"type".into(), &"pong".into()).unwrap();
                }
            }
            messages_array.push(&message_obj);
        }
        
        messages_array.into()
    }
    
    /// Устанавливает P2P соединение
    pub fn connect(&mut self) -> Result<(), JsValue> {
        // В веб-версии соединение устанавливается через JavaScript
        // Здесь мы просто обновляем статус
        Ok(())
    }
    
    /// Отключается от игры
    pub fn disconnect(&mut self) -> Result<(), JsValue> {
        // Отправляем сообщение об отключении
        let disconnect_msg = NetworkMessage::Disconnect;
        self.manager
            .borrow_mut()
            .add_outgoing_message(disconnect_msg);
        
        // Обновляем статус
        self.manager
            .borrow_mut()
            .set_connection_status(ConnectionStatus::Disconnected);
        
        Ok(())
    }
    
    /// Запрашивает новую игру
    pub fn request_new_game(&self) -> Result<(), JsValue> {
        let new_game_msg = NetworkMessage::NewGameRequest;
        self.manager
            .borrow_mut()
            .add_outgoing_message(new_game_msg);
        Ok(())
    }
    
    /// Устанавливает имя игрока
    pub fn set_player_name(&mut self, name: String) {
        self.manager.borrow_mut().local_player_name = name;
    }
    
    /// Проверяет, подключен ли игрок
    pub fn is_connected(&self) -> bool {
        matches!(self.manager.borrow().get_connection_status(), ConnectionStatus::Connected)
    }
}

// === ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ===

/// Создает новую игру в шашки
#[wasm_bindgen]
pub fn create_checkers_game() -> Checkers {
    Checkers::new()
}

/// Проверяет, является ли ход валидным
#[wasm_bindgen]
pub fn is_valid_move(game: &Checkers, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> bool {
    let game_move = CheckersMove::new(from_row, from_col, to_row, to_col);
    game.is_valid_move(&game_move)
}

/// Выполняет ход в игре
#[wasm_bindgen]
pub fn make_move(game: &mut Checkers, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> bool {
    let game_move = CheckersMove::new(from_row, from_col, to_row, to_col);
    game.make_move(&game_move)
}

/// Получает текущего игрока
#[wasm_bindgen]
pub fn get_current_player(game: &Checkers) -> u8 {
    match game.current_player {
        CheckersPlayer::White => 0,
        CheckersPlayer::Black => 1,
    }
}

/// Проверяет, закончена ли игра
#[wasm_bindgen]
pub fn is_game_over(game: &Checkers) -> bool {
    game.is_game_over()
}

/// Получает победителя игры
#[wasm_bindgen]
pub fn get_winner(game: &Checkers) -> Option<u8> {
    game.get_winner().map(|player| match player {
        CheckersPlayer::White => 0,
        CheckersPlayer::Black => 1,
    })
}

/// Сбрасывает игру в начальное состояние
#[wasm_bindgen]
pub fn reset_game(game: &mut Checkers) {
    *game = Checkers::new();
}

/// Получает состояние доски в виде строки
#[wasm_bindgen]
pub fn get_board_state(game: &Checkers) -> String {
    format!("{:?}", game)
}
