//! # Игры - Игровая логика
//! 
//! Этот модуль содержит основную логику игр:
//! - Крестики-нолики
//! - Шашки
//! 
//! Этот модуль содержит основную логику игры "Крестики-нолики".
//! Включает в себя структуры данных, игровую механику и тесты.
//! 
//! ## Использование
//! ```rust
//! use rust_tic_tac_toe::TicTacToe;
//! 
//! let mut game = TicTacToe::new();
//! game.make_move(4); // X делает ход в центр
//! game.make_move(0); // O делает ход в левый верхний угол
//! ```
//! 
//! ## Веб-версия
//! Для запуска в браузере используйте:
//! ```bash
//! wasm-pack build --target web
//! ```

/// Поддерживаемые языки
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Language {
    /// Русский язык
    Russian,
    /// Английский язык
    English,
}

/// Структура для локализации
#[derive(Clone)]
pub struct Localization {
    /// Текущий язык
    pub language: Language,
}

impl Localization {
    /// Создает новую локализацию с указанным языком
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// Возвращает локализованный текст по ключу
    pub fn get_text(&self, key: &str) -> &'static str {
        match self.language {
            Language::Russian => match key {
                "game_title" => "Крестики-нолики",
                "current_player_turn" => "Ход игрока: {}",
                "winner" => "Победитель: {}!",
                "draw" => "Ничья!",
                "new_game" => "Новая игра",
                "exit" => "Выход",
                "loading" => "Загрузка игры...",
                "game_loaded" => "Игра загружена!",
                "loading_error" => "Ошибка загрузки игры: {}",
                "performance" => "Производительность",
                "performance_desc" => "Нативная скорость Rust в браузере",
                "design" => "Дизайн",
                "design_desc" => "Современный и красивый интерфейс",
                "compatibility" => "Совместимость",
                "compatibility_desc" => "Работает на всех устройствах",
                "download_source" => "Скачать исходный код",
                "play_again" => "Играть снова",
                "created_with" => "Создано с ❤️ на Rust + WebAssembly",
                "language_switch" => "Язык",
                "russian" => "Русский",
                "english" => "English",
                "checkers_title" => "Шашки",
                "tic_tac_toe_title" => "Крестики-нолики",
                "game_select" => "Выберите игру",
                "white_turn" => "Ход белых",
                "black_turn" => "Ход черных",
                "white_wins" => "Белые победили!",
                "black_wins" => "Черные победили!",
                _ => "Unknown",
            },
            Language::English => match key {
                "game_title" => "Tic-Tac-Toe",
                "current_player_turn" => "Player's turn: {}",
                "winner" => "Winner: {}!",
                "draw" => "Draw!",
                "new_game" => "New Game",
                "exit" => "Exit",
                "loading" => "Loading game...",
                "game_loaded" => "Game loaded!",
                "loading_error" => "Loading error: {}",
                "performance" => "Performance",
                "performance_desc" => "Native Rust speed in browser",
                "design" => "Design",
                "design_desc" => "Modern and beautiful interface",
                "compatibility" => "Compatibility",
                "compatibility_desc" => "Works on all devices",
                "download_source" => "Download source code",
                "play_again" => "Play again",
                "created_with" => "Created with ❤️ using Rust + WebAssembly",
                "language_switch" => "Language",
                "russian" => "Русский",
                "english" => "English",
                "checkers_title" => "Checkers",
                "tic_tac_toe_title" => "Tic-Tac-Toe",
                "game_select" => "Select Game",
                "white_turn" => "White's turn",
                "black_turn" => "Black's turn",
                "white_wins" => "White wins!",
                "black_wins" => "Black wins!",
                _ => "Unknown",
            },
        }
    }
}

/// Игрок в игре "Крестики-нолики"
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    /// Игрок X (ходит первым)
    X,
    /// Игрок O (ходит вторым)
    O,
}

impl Player {
    /// Возвращает символ игрока для отображения
    pub fn symbol(&self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

/// Основная структура игры "Крестики-нолики"
/// 
/// Игровое поле представляет собой массив 3x3, где:
/// - Индексы 0-2: первая строка (сверху)
/// - Индексы 3-5: вторая строка (по центру)  
/// - Индексы 6-8: третья строка (снизу)
/// 
/// ## Пример использования
/// ```rust
/// let mut game = TicTacToe::new();
/// 
/// // X делает ход в центр (позиция 4)
/// game.make_move(4);
/// 
/// // O делает ход в левый верхний угол (позиция 0)
/// game.make_move(0);
/// 
/// // Проверяем, закончилась ли игра
/// if game.is_game_over() {
///     match game.get_winner() {
///         Some(player) => println!("Победитель: {}!", player.symbol()),
///         None => println!("Ничья!"),
///     }
/// }
/// ```
#[derive(Clone)]
pub struct TicTacToe {
    /// Игровое поле: массив 9 элементов, где None = пустая клетка
    board: [Option<Player>; 9],
    /// Текущий игрок, который должен сделать ход
    current_player: Player,
    /// Флаг, указывающий, что игра закончена
    game_over: bool,
}

impl TicTacToe {
    /// Создает новую игру
    /// 
    /// Игра начинается с игрока X, поле пустое
    pub fn new() -> Self {
        Self {
            board: [None; 9],
            current_player: Player::X,
            game_over: false,
        }
    }
    
    /// Сбрасывает игру в начальное состояние
    pub fn reset(&mut self) {
        self.board = [None; 9];
        self.current_player = Player::X;
        self.game_over = false;
    }
    
    /// Возвращает символ текущего игрока
    pub fn current_player_symbol(&self) -> &'static str {
        self.current_player.symbol()
    }
    
    /// Делает ход в указанную позицию
    /// 
    /// ## Параметры
    /// - `position`: позиция для хода (0-8)
    /// 
    /// ## Возвращает
    /// - `true` если ход успешно сделан
    /// - `false` если ход некорректен (позиция занята или не существует)
    /// 
    /// ## Пример
    /// ```rust
    /// let mut game = TicTacToe::new();
    /// assert!(game.make_move(4));  // X в центр - успешно
    /// assert!(!game.make_move(4)); // X в центр снова - неуспешно (занято)
    /// assert!(!game.make_move(9)); // X в несуществующую позицию - неуспешно
    /// ```
    pub fn make_move(&mut self, position: usize) -> bool {
        // Проверяем корректность позиции
        if position >= 9 || self.board[position].is_some() {
            return false;
        }
        
        // Делаем ход
        self.board[position] = Some(self.current_player);
        
        // Проверяем, есть ли победитель
        if self.check_winner(position) {
            self.game_over = true;
        } else if self.is_board_full() {
            // Если доска заполнена без победителя - ничья
            self.game_over = true;
        } else {
            // Переключаем игрока
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
        
        true
    }
    
    /// Проверяет, есть ли победитель после хода в указанную позицию
    /// 
    /// Проверяет все возможные выигрышные комбинации:
    /// - Горизонтали: [0,1,2], [3,4,5], [6,7,8]
    /// - Вертикали: [0,3,6], [1,4,7], [2,5,8]  
    /// - Диагонали: [0,4,8], [2,4,6]
    fn check_winner(&self, position: usize) -> bool {
        let player = self.board[position].unwrap();
        
        // Все возможные выигрышные комбинации
        let win_patterns = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // горизонтали
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // вертикали
            [0, 4, 8], [2, 4, 6], // диагонали
        ];
        
        // Проверяем каждую комбинацию
        for pattern in &win_patterns {
            if pattern.iter().all(|&pos| self.board[pos] == Some(player)) {
                return true;
            }
        }
        
        false
    }
    
    /// Проверяет, заполнена ли доска полностью
    fn is_board_full(&self) -> bool {
        self.board.iter().all(|cell| cell.is_some())
    }
    
    /// Проверяет, закончена ли игра
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
    
    /// Возвращает победителя игры, если он есть
    /// 
    /// ## Возвращает
    /// - `Some(player)` если есть победитель
    /// - `None` если игра в ничью или еще не закончена
    pub fn get_winner(&self) -> Option<Player> {
        if !self.game_over {
            return None;
        }
        
        // Проверяем все возможные выигрышные комбинации
        let win_patterns = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6],
        ];
        
        for pattern in &win_patterns {
            if let Some(player) = self.board[pattern[0]] {
                if pattern.iter().all(|&pos| self.board[pos] == Some(player)) {
                    return Some(player);
                }
            }
        }
        
        None // Ничья
    }
    
    /// Отображает игровое поле в консоли
    /// 
    /// ## Пример вывода
    /// ```
    ///  X | O | X
    /// ---------
    ///  O | X | O
    /// ---------
    ///  O | X | X
    /// ```
    pub fn display_board(&self) {
        println!();
        for row in 0..3 {
            let start = row * 3;
            let row_values: Vec<String> = (start..start + 3)
                .map(|i| match self.board[i] {
                    Some(player) => player.symbol().to_string(),
                    None => " ".to_string(),
                })
                .collect();
            
            println!(" {} | {} | {} ", row_values[0], row_values[1], row_values[2]);
            
            if row < 2 {
                println!("---------");
            }
        }
        println!();
    }
    
    // === МЕТОДЫ ДЛЯ ТЕСТИРОВАНИЯ ===
    
    /// Возвращает текущее состояние доски (для тестов)
    pub fn get_board(&self) -> [Option<Player>; 9] {
        self.board
    }
    
    /// Устанавливает состояние доски (для тестов)
    pub fn set_board(&mut self, board: [Option<Player>; 9]) {
        self.board = board;
    }
    
    /// Устанавливает текущего игрока (для тестов)
    pub fn set_current_player(&mut self, player: Player) {
        self.current_player = player;
    }
}

// === ШАШКИ ===

pub mod checkers;

// === ВЕБ-ВЕРСИЯ ===

#[cfg(target_arch = "wasm32")]
pub mod web;
#[cfg(target_arch = "wasm32")]
pub mod checkers_web;

// === ТЕСТЫ ===

#[cfg(test)]
mod tests {
    use super::*;

    /// Тест: новая игра создается корректно
    #[test]
    fn test_new_game() {
        let game = TicTacToe::new();
        assert!(!game.is_game_over());
        assert_eq!(game.current_player_symbol(), "X");
        assert_eq!(game.get_winner(), None);
    }

    /// Тест: выигрыш по горизонтали
    #[test]
    fn test_horizontal_win() {
        let mut game = TicTacToe::new();
        // X | X | X
        // ---------
        //   |   |  
        // ---------
        //   |   |  
        game.make_move(0); // X в позицию 0
        game.make_move(3); // O в позицию 3
        game.make_move(1); // X в позицию 1
        game.make_move(4); // O в позицию 4
        game.make_move(2); // X в позицию 2
        
        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), Some(Player::X));
    }

    /// Тест: выигрыш по вертикали
    #[test]
    fn test_vertical_win() {
        let mut game = TicTacToe::new();
        // X |   |  
        // ---------
        // X |   |  
        // ---------
        // X |   |  
        game.make_move(0); // X в позицию 0
        game.make_move(1); // O в позицию 1
        game.make_move(3); // X в позицию 3
        game.make_move(2); // O в позицию 2
        game.make_move(6); // X в позицию 6
        
        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), Some(Player::X));
    }

    /// Тест: выигрыш по диагонали
    #[test]
    fn test_diagonal_win() {
        let mut game = TicTacToe::new();
        // X |   |  
        // ---------
        //   | X |  
        // ---------
        //   |   | X
        game.make_move(0); // X в позицию 0
        game.make_move(1); // O в позицию 1
        game.make_move(4); // X в позицию 4
        game.make_move(2); // O в позицию 2
        game.make_move(8); // X в позицию 8
        
        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), Some(Player::X));
    }

    /// Тест: ничья
    #[test]
    fn test_draw() {
        let mut game = TicTacToe::new();
        // X | O | X
        // ---------
        // O | X | O
        // ---------
        // O | X | X
        game.make_move(0); // X
        game.make_move(1); // O
        game.make_move(2); // X
        game.make_move(3); // O
        game.make_move(4); // X
        game.make_move(5); // O
        game.make_move(6); // X
        game.make_move(7); // O
        game.make_move(8); // X
        
        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), None); // Ничья
    }

    /// Тест: некорректные ходы
    #[test]
    fn test_invalid_move() {
        let mut game = TicTacToe::new();
        game.make_move(0); // X в позицию 0
        
        // Попытка сделать ход в уже занятую позицию
        assert!(!game.make_move(0));
        
        // Попытка сделать ход в несуществующую позицию
        assert!(!game.make_move(9));
    }

    /// Тест: переключение игроков
    #[test]
    fn test_player_switching() {
        let mut game = TicTacToe::new();
        assert_eq!(game.current_player_symbol(), "X");
        
        game.make_move(0); // X делает ход
        assert_eq!(game.current_player_symbol(), "O");
        
        game.make_move(1); // O делает ход
        assert_eq!(game.current_player_symbol(), "X");
    }

    /// Тест: сброс игры
    #[test]
    fn test_reset() {
        let mut game = TicTacToe::new();
        game.make_move(0);
        game.make_move(1);
        
        game.reset();
        
        assert!(!game.is_game_over());
        assert_eq!(game.current_player_symbol(), "X");
        assert_eq!(game.get_winner(), None);
        
        // Проверяем, что доска очищена
        for cell in game.get_board().iter() {
            assert!(cell.is_none());
        }
    }
}
