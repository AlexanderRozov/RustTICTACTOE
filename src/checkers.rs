//! # Шашки - Игровая логика
//! 
//! Этот модуль содержит основную логику игры "Шашки".
//! Включает в себя структуры данных, игровую механику и правила игры.
//! 
//! ## Использование
//! ```rust
//! use rust_tic_tac_toe::checkers::Checkers;
//! 
//! let mut game = Checkers::new();
//! game.make_move(2, 1, 3, 2); // Ход белой шашкой
//! ```

// === ОСНОВНЫЕ ТИПЫ ===

/// Игрок в игре "Шашки"
#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum CheckersPlayer {
    /// Белые шашки (ходят первыми)
    White,
    /// Черные шашки (ходят вторыми)
    Black,
}

impl CheckersPlayer {
    /// Возвращает символ игрока для отображения
    pub fn symbol(&self) -> &'static str {
        match self {
            CheckersPlayer::White => "⚪",
            CheckersPlayer::Black => "⚫",
        }
    }
    
    /// Возвращает символ дамки для отображения
    pub fn king_symbol(&self) -> &'static str {
        match self {
            CheckersPlayer::White => "👑",
            CheckersPlayer::Black => "👑",
        }
    }
    
    /// Возвращает направление движения для обычных шашек
    pub fn direction(&self) -> i32 {
        match self {
            CheckersPlayer::White => 1,  // Вниз
            CheckersPlayer::Black => -1, // Вверх
        }
    }
}

/// Тип шашки
#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum CheckerType {
    /// Обычная шашка
    Regular,
    /// Дамка
    King,
}

/// Шашка на доске
#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Checker {
    /// Игрок, которому принадлежит шашка
    pub player: CheckersPlayer,
    /// Тип шашки
    pub checker_type: CheckerType,
}

impl Checker {
    /// Создает новую обычную шашку
    pub fn new(player: CheckersPlayer) -> Self {
        Self {
            player,
            checker_type: CheckerType::Regular,
        }
    }
    
    /// Создает новую дамку
    pub fn new_king(player: CheckersPlayer) -> Self {
        Self {
            player,
            checker_type: CheckerType::King,
        }
    }
    
    /// Повышает шашку до дамки
    pub fn promote_to_king(&mut self) {
        self.checker_type = CheckerType::King;
    }
    
    /// Проверяет, является ли шашка дамкой
    pub fn is_king(&self) -> bool {
        matches!(self.checker_type, CheckerType::King)
    }
}

/// Ход в игре
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CheckersMove {
    /// Начальная позиция (строка, столбец)
    pub from: (usize, usize),
    /// Конечная позиция (строка, столбец)
    pub to: (usize, usize),
    /// Позиции захваченных шашек
    pub captures: Vec<(usize, usize)>,
}

impl CheckersMove {
    /// Создает новый ход
    pub fn new(from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> Self {
        Self {
            from: (from_row, from_col),
            to: (to_row, to_col),
            captures: Vec::new(),
        }
    }
    
    /// Создает ход с захватом
    pub fn with_captures(from: (usize, usize), to: (usize, usize), captures: Vec<(usize, usize)>) -> Self {
        Self { from, to, captures }
    }
}

/// Структура для логгирования хода
#[derive(Debug, Clone)]
pub struct MoveLog {
    /// Номер хода
    pub move_number: usize,
    /// Игрок, сделавший ход
    pub player: CheckersPlayer,
    /// Откуда
    pub from: (usize, usize),
    /// Куда
    pub to: (usize, usize),
    /// Тип хода
    pub move_type: MoveType,
    /// Захваченные шашки
    pub captured: Vec<(usize, usize)>,
    /// Позиция доски после хода
    pub board_after: String,
}

/// Тип хода
#[derive(Debug, Clone)]
pub enum MoveType {
    /// Обычный ход
    Regular,
    /// Взятие
    Capture,
    /// Превращение в дамку
    Promotion,
    /// Взятие с превращением
    CaptureWithPromotion,
}

// === ОСНОВНАЯ ИГРОВАЯ ЛОГИКА ===

/// Основная структура игры в шашки
#[derive(Clone, Debug)]
pub struct Checkers {
    /// Игровое поле 8x8
    board: [[Option<Checker>; 8]; 8],
    /// Текущий игрок
    pub current_player: CheckersPlayer,
    /// Флаг окончания игры
    game_over: bool,
    /// Последний сделанный ход
    last_move: Option<CheckersMove>,
    /// История ходов
    move_history: Vec<MoveLog>,
}

impl Checkers {
    // === КОНСТРУКТОРЫ И ИНИЦИАЛИЗАЦИЯ ===
    
    /// Создает новую игру
    /// 
    /// Расставляет шашки в начальные позиции:
    /// - Белые шашки в верхних 3 рядах
    /// - Черные шашки в нижних 3 рядах
    /// - Только на черных клетках
    pub fn new() -> Self {
        let mut board = [[None; 8]; 8];
        
        // Расставляем белые шашки (верхние 3 ряда)
        for row in 0..3 {
            for col in 0..8 {
                if Self::is_black_cell(row, col) {
                    board[row][col] = Some(Checker::new(CheckersPlayer::White));
                }
            }
        }
        
        // Расставляем черные шашки (нижние 3 ряда)
        for row in 5..8 {
            for col in 0..8 {
                if Self::is_black_cell(row, col) {
                    board[row][col] = Some(Checker::new(CheckersPlayer::Black));
                }
            }
        }
        
        Self {
            board,
            current_player: CheckersPlayer::White,
            game_over: false,
            last_move: None,
            move_history: Vec::new(),
        }
    }
    
    /// Сбрасывает игру в начальное состояние
    pub fn reset(&mut self) {
        *self = Self::new();
        println!("Игра сброшена в начальное состояние");
    }
    
    // === ГЕТТЕРЫ И СВОЙСТВА ===
    
    /// Возвращает символ текущего игрока
    pub fn current_player_symbol(&self) -> &'static str {
        self.current_player.symbol()
    }
    
    /// Возвращает текущее состояние доски
    pub fn get_board(&self) -> [[Option<Checker>; 8]; 8] {
        self.board
    }
    
    /// Получает шашку в указанной позиции
    pub fn get_checker(&self, row: usize, col: usize) -> Option<Checker> {
        if Self::is_valid_position(row, col) {
            self.board[row][col]
        } else {
            None
        }
    }
    
    /// Возвращает последний сделанный ход
    pub fn get_last_move(&self) -> Option<CheckersMove> {
        self.last_move.clone()
    }
    
    /// Возвращает историю ходов
    pub fn get_move_history(&self) -> &[MoveLog] {
        &self.move_history
    }
    
    /// Возвращает последний ход из истории
    pub fn get_last_move_log(&self) -> Option<&MoveLog> {
        self.move_history.last()
    }
    
    /// Проверяет, закончена ли игра
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
    
    /// Возвращает победителя игры, если он есть
    pub fn get_winner(&self) -> Option<CheckersPlayer> {
        if !self.game_over {
            return None;
        }
        
        let mut white_count = 0;
        let mut black_count = 0;
        
        for row in &self.board {
            for cell in row {
                if let Some(checker) = cell {
                    match checker.player {
                        CheckersPlayer::White => white_count += 1,
                        CheckersPlayer::Black => black_count += 1,
                    }
                }
            }
        }
        
        if white_count == 0 {
            Some(CheckersPlayer::Black)
        } else if black_count == 0 {
            Some(CheckersPlayer::White)
        } else {
            None
        }
    }
    
    // === УТИЛИТЫ ===
    
    /// Проверяет, является ли клетка черной (игровой)
    pub fn is_black_cell(row: usize, col: usize) -> bool {
        (row + col) % 2 == 1
    }
    
    /// Проверяет, находится ли позиция в пределах доски
    pub fn is_valid_position(row: usize, col: usize) -> bool {
        row < 8 && col < 8
    }
    
    // === ОСНОВНАЯ ИГРОВАЯ МЕХАНИКА ===
    
    /// Делает ход
    /// 
    /// ## Параметры
    /// - `from_row`, `from_col`: начальная позиция
    /// - `to_row`, `to_col`: конечная позиция
    /// 
    /// ## Возвращает
    /// - `true` если ход успешно сделан
    /// - `false` если ход некорректен
    pub fn make_move(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> bool {
        // Проверяем корректность позиций
        if !Self::is_valid_position(from_row, from_col) || !Self::is_valid_position(to_row, to_col) {
            return false;
        }
        
        // Проверяем, что начальная клетка содержит шашку текущего игрока
        let checker = match self.board[from_row][from_col] {
            Some(c) if c.player == self.current_player => c,
            _ => return false,
        };
        
        // Проверяем, что конечная клетка пуста и черная
        if self.board[to_row][to_col].is_some() || !Self::is_black_cell(to_row, to_col) {
            return false;
        }
        
        // Проверяем, есть ли обязательные взятия
        let has_captures = self.has_forced_captures();
        let is_capture_move = self.is_capture_move(from_row, from_col, to_row, to_col);
        
        // Если есть обязательные взятия, то можно делать только взятия
        if has_captures && !is_capture_move {
            return false;
        }
        
        // Проверяем корректность хода
        if !self.is_valid_move(from_row, from_col, to_row, to_col, &checker) {
            return false;
        }
        
        // Выполняем ход
        self.board[to_row][to_col] = Some(checker);
        self.board[from_row][from_col] = None;
        
        // Определяем тип хода для логгирования
        let mut move_type = if is_capture_move { MoveType::Capture } else { MoveType::Regular };
        let mut captured = Vec::new();
        
        // Если это взятие, удаляем захваченную шашку
        if is_capture_move {
            let captured_row = (from_row + to_row) / 2;
            let captured_col = (from_col + to_col) / 2;
            self.board[captured_row][captured_col] = None;
            captured.push((captured_row, captured_col));
            
            // Сохраняем последний ход с информацией о взятии
            self.last_move = Some(CheckersMove::with_captures(
                (from_row, from_col),
                (to_row, to_col),
                captured.clone()
            ));
        } else {
            // Сохраняем последний ход
            self.last_move = Some(CheckersMove::new(from_row, from_col, to_row, to_col));
        }
        
        // Проверяем, нужно ли повысить шашку до дамки
        let should_promote = self.should_promote_to_king(to_row, to_col, &checker);
        if should_promote {
            if let Some(checker) = &mut self.board[to_row][to_col] {
                checker.promote_to_king();
                // Обновляем тип хода
                move_type = if is_capture_move { MoveType::CaptureWithPromotion } else { MoveType::Promotion };
            }
        }
        
        // Логгируем ход
        self.log_move((from_row, from_col), (to_row, to_col), move_type, captured);
        
        // Проверяем, есть ли победитель
        if self.check_winner() {
            self.game_over = true;
        } else {
            // Переключаем игрока только если нет возможности продолжить взятие
            if is_capture_move {
                // Проверяем, можно ли продолжить взятие той же шашкой
                let possible_captures = self.get_possible_captures(to_row, to_col);
                
                if !possible_captures.is_empty() {
                    // Не переключаем игрока - он может продолжить взятие
                    return true;
                }
            }
            
            // Переключаем игрока
            self.current_player = match self.current_player {
                CheckersPlayer::White => CheckersPlayer::Black,
                CheckersPlayer::Black => CheckersPlayer::White,
            };
        }
        
        true
    }
    
    // === ВАЛИДАЦИЯ ХОДОВ ===
    
    /// Проверяет корректность хода
    fn is_valid_move(&self, from_row: usize, from_col: usize, to_row: usize, to_col: usize, checker: &Checker) -> bool {
        let row_diff = to_row as i32 - from_row as i32;
        let col_diff = to_col as i32 - from_col as i32;
        
        // Проверяем, что ход по диагонали
        if row_diff.abs() != col_diff.abs() {
            return false;
        }
        
        let is_capture = row_diff.abs() == 2;
        
        match checker.checker_type {
            CheckerType::Regular => {
                if is_capture {
                    // Взятие: ход на 2 клетки по диагонали ТОЛЬКО ВПЕРЕД
                    let direction = checker.player.direction();
                    if row_diff != direction * 2 || col_diff.abs() != 2 {
                        return false;
                    }
                } else {
                    // Обычный ход: на 1 клетку вперед по диагонали
                    let direction = checker.player.direction();
                    if row_diff != direction || col_diff.abs() != 1 {
                        return false;
                    }
                }
            }
            CheckerType::King => {
                // Дамки могут ходить на любое расстояние по диагонали в любом направлении
                if row_diff.abs() < 1 || col_diff.abs() < 1 {
                    return false;
                }
            }
        }
        
        if is_capture {
            // Для взятия проверяем, что между позициями есть шашка противника
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            
            if let Some(mid_checker) = self.board[mid_row][mid_col] {
                if mid_checker.player == checker.player {
                    return false; // Нельзя брать свою шашку
                }
            } else {
                return false; // Между позициями должна быть шашка
            }
        } else {
            // Для обычного хода проверяем, что путь свободен
            if !self.is_path_clear(from_row, from_col, to_row, to_col) {
                return false;
            }
        }
        
        true
    }
    
    /// Проверяет, свободен ли путь между двумя позициями
    fn is_path_clear(&self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> bool {
        let row_step = if to_row > from_row { 1 } else { -1 };
        let col_step = if to_col > from_col { 1 } else { -1 };
        
        let mut current_row = from_row as i32 + row_step;
        let mut current_col = from_col as i32 + col_step;
        
        while current_row != to_row as i32 && current_col != to_col as i32 {
            if self.board[current_row as usize][current_col as usize].is_some() {
                return false;
            }
            current_row += row_step;
            current_col += col_step;
        }
        
        true
    }
    
    /// Проверяет, нужно ли повысить шашку до дамки
    fn should_promote_to_king(&self, row: usize, _col: usize, checker: &Checker) -> bool {
        if checker.is_king() {
            return false;
        }
        
        match checker.player {
            CheckersPlayer::White => row == 7, // Белые шашки достигают нижнего края
            CheckersPlayer::Black => row == 0, // Черные шашки достигают верхнего края
        }
    }
    
    /// Проверяет, есть ли победитель
    fn check_winner(&self) -> bool {
        let mut white_count = 0;
        let mut black_count = 0;
        
        for row in &self.board {
            for cell in row {
                if let Some(checker) = cell {
                    match checker.player {
                        CheckersPlayer::White => white_count += 1,
                        CheckersPlayer::Black => black_count += 1,
                    }
                }
            }
        }
        
        white_count == 0 || black_count == 0
    }
    
    // === ПОИСК ВОЗМОЖНЫХ ХОДОВ ===
    
    /// Получает все возможные ходы для указанной шашки
    pub fn get_possible_moves(&self, row: usize, col: usize) -> Vec<CheckersMove> {
        let mut moves = Vec::new();
        
        if let Some(checker) = self.board[row][col] {
            if checker.player != self.current_player {
                return moves;
            }
            
            // Сначала проверяем, есть ли обязательные взятия
            let captures = self.get_possible_captures(row, col);
            if !captures.is_empty() {
                return captures;
            }
            
            // Если взятий нет, возвращаем обычные ходы
            let directions = match checker.checker_type {
                CheckerType::Regular => vec![checker.player.direction()],
                CheckerType::King => vec![-1, 1],
            };
            
            for &row_dir in &directions {
                for &col_dir in &[-1, 1] {
                    let mut new_row = row as i32 + row_dir;
                    let mut new_col = col as i32 + col_dir;
                    
                    // Проверяем, что начальная позиция для движения валидна
                    if !Self::is_valid_position(new_row as usize, new_col as usize) {
                        continue;
                    }
                    
                    while Self::is_valid_position(new_row as usize, new_col as usize) {
                        if !Self::is_black_cell(new_row as usize, new_col as usize) {
                            break;
                        }
                        
                        if self.board[new_row as usize][new_col as usize].is_none() {
                            moves.push(CheckersMove::new(row, col, new_row as usize, new_col as usize));
                        } else {
                            break;
                        }
                        
                        // Обычные шашки могут ходить только на 1 клетку
                        if matches!(checker.checker_type, CheckerType::Regular) {
                            break;
                        }
                        
                        new_row += row_dir;
                        new_col += col_dir;
                    }
                }
            }
        }
        
        moves
    }
    
    /// Получает все возможные взятия для указанной шашки
    pub fn get_possible_captures(&self, row: usize, col: usize) -> Vec<CheckersMove> {
        let mut captures = Vec::new();
        
        if let Some(checker) = self.board[row][col] {
            if checker.player != self.current_player {
                return captures;
            }
            
            // Ищем все возможные взятия, включая множественные
            self.find_all_captures(row, col, &checker, &mut captures, &mut Vec::new());
        }
        
        captures
    }
    
    /// Рекурсивно ищет все возможные взятия для шашки
    fn find_all_captures(&self, row: usize, col: usize, checker: &Checker, 
                         captures: &mut Vec<CheckersMove>, path: &mut Vec<(usize, usize)>) {
        let directions = match checker.checker_type {
            CheckerType::Regular => vec![checker.player.direction()], // Только вперед для обычных шашек
            CheckerType::King => vec![-1, 1], // В любом направлении для дамок
        };
        
        for &row_dir in &directions {
            for &col_dir in &[-1, 1] {
                // Ищем шашку противника для взятия
                let jump_row = row as i32 + row_dir;
                let jump_col = col as i32 + col_dir;
                
                if !Self::is_valid_position(jump_row as usize, jump_col as usize) {
                    continue;
                }
                
                // Проверяем, есть ли шашка противника для взятия
                if let Some(jump_checker) = self.board[jump_row as usize][jump_col as usize] {
                    if jump_checker.player != checker.player {
                        // Проверяем, что эта шашка еще не была взята в текущем пути
                        if !path.contains(&(jump_row as usize, jump_col as usize)) {
                            // Ищем позиции приземления
                            let mut land_positions = Vec::new();
                            
                            if checker.is_king() {
                                // Дамки могут приземлиться на любое свободное поле за шашкой противника
                                let mut land_row = jump_row + row_dir;
                                let mut land_col = jump_col + col_dir;
                                
                                while Self::is_valid_position(land_row as usize, land_col as usize) &&
                                       Self::is_black_cell(land_row as usize, land_col as usize) {
                                    
                                    if self.board[land_row as usize][land_col as usize].is_none() {
                                        land_positions.push((land_row as usize, land_col as usize));
                                    } else {
                                        break; // Встретили препятствие
                                    }
                                    
                                    land_row += row_dir;
                                    land_col += col_dir;
                                }
                            } else {
                                // Обычные шашки могут приземлиться только на следующую клетку
                                let land_row = jump_row + row_dir;
                                let land_col = jump_col + col_dir;
                                
                                if Self::is_valid_position(land_row as usize, land_col as usize) &&
                                   Self::is_black_cell(land_row as usize, land_col as usize) &&
                                   self.board[land_row as usize][land_col as usize].is_none() {
                                    
                                    land_positions.push((land_row as usize, land_col as usize));
                                }
                            }
                            
                            // Добавляем все найденные позиции приземления
                            for land_pos in land_positions {
                                let mut new_path = path.clone();
                                new_path.push((jump_row as usize, jump_col as usize));
                                
                                // Создаем ход с захватом
                                let capture_move = CheckersMove::with_captures(
                                    (row, col),
                                    land_pos,
                                    new_path.clone()
                                );
                                
                                captures.push(capture_move);
                                
                                // Рекурсивно ищем дополнительные взятия с этой позиции
                                if checker.is_king() {
                                    // Для дамок проверяем возможность продолжения взятия
                                    self.find_all_captures(land_pos.0, land_pos.1, checker, 
                                                         captures, &mut new_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Проверяет, есть ли обязательные взятия для текущего игрока
    pub fn has_forced_captures(&self) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(checker) = self.board[row][col] {
                    if checker.player == self.current_player {
                        if !self.get_possible_captures(row, col).is_empty() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    /// Находит лучшее взятие (с максимальным количеством захваченных шашек)
    pub fn find_best_capture(&self) -> Option<CheckersMove> {
        let mut best_capture: Option<CheckersMove> = None;
        let mut max_captures = 0;
        
        for row in 0..8 {
            for col in 0..8 {
                if let Some(checker) = self.board[row][col] {
                    if checker.player == self.current_player {
                        let captures = self.get_possible_captures(row, col);
                        for capture in captures {
                            if capture.captures.len() > max_captures {
                                max_captures = capture.captures.len();
                                best_capture = Some(capture);
                            }
                        }
                    }
                }
            }
        }
        
        best_capture
    }
    
    /// Получает все возможные ходы для текущего игрока
    pub fn get_all_possible_moves(&self) -> Vec<CheckersMove> {
        let mut all_moves = Vec::new();
        let has_captures = self.has_forced_captures();
        
        for row in 0..8 {
            for col in 0..8 {
                if let Some(checker) = self.board[row][col] {
                    if checker.player == self.current_player {
                        if has_captures {
                            // Если есть обязательные взятия, добавляем только взятия
                            all_moves.extend(self.get_possible_captures(row, col));
                        } else {
                            // Если взятий нет, добавляем обычные ходы
                            all_moves.extend(self.get_possible_moves(row, col));
                        }
                    }
                }
            }
        }
        
        all_moves
    }

    /// Проверяет, является ли ход взятием
    pub fn is_capture_move(&self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> bool {
        let row_diff = to_row as i32 - from_row as i32;
        let col_diff = to_col as i32 - from_col as i32;
        
        // Взятие - это ход на расстояние 2 по диагонали
        row_diff.abs() == 2 && col_diff.abs() == 2
    }
    
    // === ЛОГГИРОВАНИЕ ===
    
    /// Логгирует ход
    fn log_move(&mut self, from: (usize, usize), to: (usize, usize), 
                move_type: MoveType, captured: Vec<(usize, usize)>) {
        let move_number = self.move_history.len() + 1;
        let player = self.current_player;
        
        // Создаем строковое представление доски
        let board_after = self.board_to_string();
        
        let move_log = MoveLog {
            move_number,
            player,
            from,
            to,
            move_type: move_type.clone(),
            captured: captured.clone(),
            board_after,
        };
        
        self.move_history.push(move_log);
        
        // Выводим информацию о ходе в консоль
        println!("=== ХОД {} ===", move_number);
        println!("Игрок: {}", match player { CheckersPlayer::White => "Белые", CheckersPlayer::Black => "Черные" });
        println!("От: ({}, {}) -> К: ({}, {})", from.0, from.1, to.0, to.1);
        println!("Тип: {}", match &move_type {
            MoveType::Regular => "Обычный ход",
            MoveType::Capture => "Взятие",
            MoveType::Promotion => "Превращение в дамку",
            MoveType::CaptureWithPromotion => "Взятие с превращением",
        });
        if !captured.is_empty() {
            println!("Захвачено: {:?}", captured);
        }
        println!("Доска после хода:");
        self.display_board();
        println!("==================");
    }
    
    /// Преобразует доску в строку для логгирования
    fn board_to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(checker) => {
                        if checker.is_king() {
                            result.push(checker.player.king_symbol().chars().next().unwrap_or('K'));
                        } else {
                            result.push(checker.player.symbol().chars().next().unwrap_or('O'));
                        }
                    }
                    None => result.push(' '),
                }
            }
            result.push('\n');
        }
        result
    }
    
    // === ОТОБРАЖЕНИЕ ===
    
    /// Отображает игровое поле в консоли
    pub fn display_board(&self) {
        println!();
        println!("   A B C D E F G H");
        println!("  ┌─┬─┬─┬─┬─┬─┬─┬─┐");
        
        for (row_idx, row) in self.board.iter().enumerate() {
            print!("{} │", 8 - row_idx);
            
            for (col_idx, cell) in row.iter().enumerate() {
                if Self::is_black_cell(row_idx, col_idx) {
                    match cell {
                        Some(checker) => {
                            if checker.is_king() {
                                print!("{}", checker.player.king_symbol());
                            } else {
                                print!("{}", checker.player.symbol());
                            }
                        }
                        None => print!(" "),
                    }
                } else {
                    print!("█");
                }
                print!("│");
            }
            println!();
            
            if row_idx < 7 {
                println!("  ├─┼─┼─┼─┼─┼─┼─┼─┤");
            }
        }
        
        println!("  └─┴─┴─┴─┴─┴─┴─┴─┘");
        println!("   A B C D E F G H");
        println!();
    }
    
    /// Отображает историю ходов
    pub fn display_move_history(&self) {
        println!("\n=== ИСТОРИЯ ХОДОВ ===");
        if self.move_history.is_empty() {
            println!("История пуста - игра еще не началась");
        } else {
            for move_log in &self.move_history {
                println!("Ход {}: {} от ({},{}) к ({},{}) - {}",
                    move_log.move_number,
                    match move_log.player { CheckersPlayer::White => "Белые", CheckersPlayer::Black => "Черные" },
                    move_log.from.0, move_log.from.1,
                    move_log.to.0, move_log.to.1,
                    match move_log.move_type {
                        MoveType::Regular => "Обычный ход",
                        MoveType::Capture => "Взятие",
                        MoveType::Promotion => "Превращение в дамку",
                        MoveType::CaptureWithPromotion => "Взятие с превращением",
                    }
                );
                if !move_log.captured.is_empty() {
                    println!("  Захвачено: {:?}", move_log.captured);
                }
            }
        }
        println!("=====================\n");
    }
    
    /// Отображает текущее состояние игры с подробной информацией
    pub fn display_game_state(&self) {
        println!("\n=== СОСТОЯНИЕ ИГРЫ ===");
        println!("Текущий игрок: {}", match self.current_player {
            CheckersPlayer::White => "Белые",
            CheckersPlayer::Black => "Черные"
        });
        println!("Игра окончена: {}", if self.game_over { "Да" } else { "Нет" });
        
        if let Some(winner) = self.get_winner() {
            println!("Победитель: {}", match winner {
                CheckersPlayer::White => "Белые",
                CheckersPlayer::Black => "Черные"
            });
        }
        
        if let Some(last_move) = &self.last_move {
            println!("Последний ход: от ({},{}) к ({},{})", 
                last_move.from.0, last_move.from.1,
                last_move.to.0, last_move.to.1);
            if !last_move.captures.is_empty() {
                println!("Захвачено: {:?}", last_move.captures);
            }
        }
        
        // Проверяем обязательные взятия
        if self.has_forced_captures() {
            println!("⚠️  Есть обязательные взятия!");
        }
        
        println!("=====================\n");
    }
    
    // === МЕТОДЫ ДЛЯ ТЕСТИРОВАНИЯ ===
    
    #[cfg(test)]
    /// Устанавливает шашку в указанную позицию (только для тестов)
    pub fn set_checker(&mut self, row: usize, col: usize, checker: Option<Checker>) {
        if Self::is_valid_position(row, col) {
            self.board[row][col] = checker;
        }
    }
    
    #[cfg(test)]
    /// Получает шашку из указанной позиции (только для тестов)
    pub fn get_checker_at(&self, row: usize, col: usize) -> Option<Checker> {
        if Self::is_valid_position(row, col) {
            self.board[row][col]
        } else {
            None
        }
    }
    
    #[cfg(test)]
    /// Очищает всю доску (только для тестов)
    pub fn clear_board(&mut self) {
        for row in 0..8 {
            for col in 0..8 {
                self.board[row][col] = None;
            }
        }
    }
    
    #[cfg(test)]
    /// Возвращает длину истории ходов (только для тестов)
    pub fn move_history_len(&self) -> usize {
        self.move_history.len()
    }
    
    #[cfg(test)]
    /// Получает ход из истории по индексу (только для тестов)
    pub fn get_move_at(&self, index: usize) -> Option<&MoveLog> {
        self.move_history.get(index)
    }
}
