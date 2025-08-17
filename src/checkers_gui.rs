//! # Графический интерфейс для игры "Шашки"
//! 
//! Этот модуль содержит GUI версию игры с использованием egui.
//! Предоставляет современный интерфейс с красивыми цветами и анимациями.

use eframe::egui;
use rust_tic_tac_toe::checkers::{Checkers, CheckersPlayer};
use rust_tic_tac_toe::{Localization, Language};

/// Основная структура графического интерфейса игры в шашки
pub struct CheckersGUI {
    /// Игровая логика
    game: Checkers,
    /// Размер одной клетки игрового поля в пикселях
    cell_size: f32,
    /// Цветовая схема интерфейса
    colors: CheckersColors,
    /// Локализация
    localization: Localization,
    /// Выбранная шашка для хода
    selected_checker: Option<(usize, usize)>,
    /// Возможные ходы для выбранной шашки
    possible_moves: Vec<(usize, usize)>,
}

/// Цветовая схема для графического интерфейса шашек
struct CheckersColors {
    /// Цвет белых клеток доски
    white_cell: egui::Color32,
    /// Цвет черных клеток доски
    black_cell: egui::Color32,
    /// Цвет белых шашек
    white_checker: egui::Color32,
    /// Цвет черных шашек
    black_checker: egui::Color32,
    /// Цвет выделения
    highlight: egui::Color32,
    /// Цвет возможных ходов
    possible_move: egui::Color32,
    /// Цвет выбранной шашки
    selected: egui::Color32,
    /// Цвет текста
    text: egui::Color32,
}

impl Default for CheckersColors {
    fn default() -> Self {
        Self {
            white_cell: egui::Color32::from_rgb(240, 217, 181),    // Светло-бежевый
            black_cell: egui::Color32::from_rgb(181, 136, 99),     // Темно-коричневый
            white_checker: egui::Color32::from_rgb(255, 255, 255), // Белый
            black_checker: egui::Color32::from_rgb(50, 50, 50),    // Темно-серый
            highlight: egui::Color32::from_rgb(255, 184, 108),     // Оранжевый
            possible_move: egui::Color32::from_rgb(144, 238, 144), // Светло-зеленый
            selected: egui::Color32::from_rgb(255, 215, 0),        // Золотой
            text: egui::Color32::from_rgb(0, 0, 0),               // Черный
        }
    }
}

impl CheckersGUI {
    /// Создает новый графический интерфейс для шашек
    pub fn new() -> Self {
        Self {
            game: Checkers::new(),
            cell_size: 60.0,
            colors: CheckersColors::default(),
            localization: Localization::new(Language::English),
            selected_checker: None,
            possible_moves: Vec::new(),
        }
    }

    /// Запускает графический интерфейс
    pub fn run(&mut self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([600.0, 700.0])
                .with_min_inner_size([500.0, 600.0])
                .with_title("Шашки"),
            ..Default::default()
        };
        
        let app = self.clone();
        eframe::run_native(
            "Шашки",
            options,
            Box::new(|_cc| Box::new(app)),
        )
    }

    /// Отрисовывает игровое поле
    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let board_size = self.cell_size * 8.0;
        
        let (response, painter) = ui.allocate_painter(
            egui::vec2(board_size, board_size),
            egui::Sense::click()
        );

        // Рисуем фон доски
        painter.rect_filled(
            egui::Rect::from_min_size(
                response.rect.min,
                egui::vec2(board_size, board_size)
            ),
            0.0,
            self.colors.white_cell,
        );

        // Рисуем клетки доски
        self.draw_cells(&painter, response.rect);
        
        // Рисуем шашки
        self.draw_checkers(&painter, response.rect);
        
        // Рисуем выделения
        self.draw_highlights(&painter, response.rect);

        // Обрабатываем клики мыши
        if response.clicked() {
            self.handle_click(&response);
        }
    }

    /// Отрисовывает клетки доски
    fn draw_cells(&self, painter: &egui::Painter, rect: egui::Rect) {
        for row in 0..8 {
            for col in 0..8 {
                if Checkers::is_black_cell(row, col) {
                    let cell_rect = egui::Rect::from_min_size(
                        egui::pos2(
                            rect.min.x + col as f32 * self.cell_size,
                            rect.min.y + row as f32 * self.cell_size,
                        ),
                        egui::vec2(self.cell_size, self.cell_size),
                    );
                    
                    painter.rect_filled(cell_rect, 0.0, self.colors.black_cell);
                }
            }
        }
    }

    /// Отрисовывает шашки на доске
    fn draw_checkers(&self, painter: &egui::Painter, rect: egui::Rect) {
        let board = self.game.get_board();
        
        for (row, row_data) in board.iter().enumerate() {
            for (col, cell) in row_data.iter().enumerate() {
                if let Some(checker) = cell {
                    let center = egui::pos2(
                        rect.min.x + col as f32 * self.cell_size + self.cell_size / 2.0,
                        rect.min.y + row as f32 * self.cell_size + self.cell_size / 2.0,
                    );
                    
                    let color = match checker.player {
                        CheckersPlayer::White => self.colors.white_checker,
                        CheckersPlayer::Black => self.colors.black_checker,
                    };
                    
                    // Рисуем основу шашки
                    let radius = self.cell_size * 0.35;
                    painter.circle_filled(center, radius, color);
                    
                    // Рисуем обводку
                    painter.circle_stroke(center, radius, egui::Stroke::new(2.0, self.colors.text));
                    
                    // Если это дамка, рисуем корону
                    if checker.is_king() {
                        self.draw_crown(painter, center, radius);
                    }
                }
            }
        }
    }

    /// Отрисовывает корону для дамки
    fn draw_crown(&self, painter: &egui::Painter, center: egui::Pos2, radius: f32) {
        let crown_color = self.colors.highlight;
        let crown_size = radius * 0.4;
        
        // Рисуем простую корону (треугольник) используя линии
        let top = egui::pos2(center.x, center.y - crown_size);
        let left = egui::pos2(center.x - crown_size, center.y + crown_size);
        let right = egui::pos2(center.x + crown_size, center.y + crown_size);
        
        // Рисуем линии короны
        let stroke = egui::Stroke::new(2.0, crown_color);
        painter.line_segment([top, left], stroke);
        painter.line_segment([top, right], stroke);
        painter.line_segment([left, right], stroke);
    }

    /// Отрисовывает выделения (выбранная шашка, возможные ходы)
    fn draw_highlights(&self, painter: &egui::Painter, rect: egui::Rect) {
        // Выделяем выбранную шашку
        if let Some((row, col)) = self.selected_checker {
            let center = egui::pos2(
                rect.min.x + col as f32 * self.cell_size + self.cell_size / 2.0,
                rect.min.y + row as f32 * self.cell_size + self.cell_size / 2.0,
            );
            
            let radius = self.cell_size * 0.4;
            painter.circle_stroke(center, radius, egui::Stroke::new(3.0, self.colors.selected));
        }
        
        // Показываем возможные ходы
        for (row, col) in &self.possible_moves {
            let center = egui::pos2(
                rect.min.x + *col as f32 * self.cell_size + self.cell_size / 2.0,
                rect.min.y + *row as f32 * self.cell_size + self.cell_size / 2.0,
            );
            
            let radius = self.cell_size * 0.2;
            painter.circle_filled(center, radius, self.colors.possible_move);
            painter.circle_stroke(center, radius, egui::Stroke::new(2.0, self.colors.text));
        }
    }

    /// Обрабатывает клики мыши по игровому полю
    fn handle_click(&mut self, response: &egui::Response) {
        if self.game.is_game_over() {
            return;
        }
        
        let click_pos = response.hover_pos().unwrap();
        let board_pos = click_pos - response.rect.min;
        
        let col = (board_pos.x / self.cell_size) as usize;
        let row = (board_pos.y / self.cell_size) as usize;
        
        if row < 8 && col < 8 {
            self.handle_cell_click(row, col);
        }
    }

    /// Обрабатывает клик по конкретной клетке
    fn handle_cell_click(&mut self, row: usize, col: usize) {
        if !Checkers::is_black_cell(row, col) {
            return; // Клик по белой клетке игнорируем
        }
        
        let board = self.game.get_board();
        
        // Если кликнули по шашке текущего игрока
        if let Some(checker) = board[row][col] {
            if checker.player == self.game.current_player {
                self.selected_checker = Some((row, col));
                self.possible_moves = self.game.get_possible_moves(row, col)
                    .into_iter()
                    .map(|mv| mv.to)
                    .collect();
                return;
            }
        }
        
        // Если кликнули по пустой клетке и есть выбранная шашка
        if let Some((from_row, from_col)) = self.selected_checker {
            if board[row][col].is_none() {
                // Пытаемся сделать ход
                if self.game.make_move(from_row, from_col, row, col) {
                    // Ход успешен, очищаем выделения
                    self.selected_checker = None;
                    self.possible_moves.clear();
                }
            }
        }
    }

    /// Отрисовывает статус игры
    fn draw_status(&self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        
        if self.game.is_game_over() {
            match self.game.get_winner() {
                Some(player) => {
                    let text = match player {
                        CheckersPlayer::White => self.localization.get_text("white_wins"),
                        CheckersPlayer::Black => self.localization.get_text("black_wins"),
                    };
                    ui.heading(egui::RichText::new(text)
                        .color(self.colors.highlight)
                        .size(24.0));
                }
                None => {
                    ui.heading(egui::RichText::new(self.localization.get_text("draw"))
                        .color(self.colors.highlight)
                        .size(24.0));
                }
            }
        } else {
            let text = match self.game.current_player {
                CheckersPlayer::White => self.localization.get_text("white_turn"),
                CheckersPlayer::Black => self.localization.get_text("black_turn"),
            };
            ui.heading(egui::RichText::new(text)
                .color(self.colors.text)
                .size(20.0));
        }
    }

    /// Отрисовывает элементы управления
    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        
        // Переключатель языка
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(self.localization.get_text("language_switch"))
                .color(self.colors.text)
                .size(16.0));
            
            if ui.button(egui::RichText::new(self.localization.get_text("russian"))
                .color(if self.localization.language == Language::Russian { self.colors.highlight } else { self.colors.text })
                .size(14.0))
                .clicked() {
                self.localization.language = Language::Russian;
            }
            
            if ui.button(egui::RichText::new(self.localization.get_text("english"))
                .color(if self.localization.language == Language::English { self.colors.highlight } else { self.colors.text })
                .size(14.0))
                .clicked() {
                self.localization.language = Language::English;
            }
        });
        
        ui.add_space(10.0);
        
        // Кнопка "Новая игра"
        if ui.button(egui::RichText::new(self.localization.get_text("new_game"))
            .color(self.colors.text)
            .size(16.0))
            .clicked() {
            self.game.reset();
            self.selected_checker = None;
            self.possible_moves.clear();
        }
        
        // Кнопка "Выход"
        if ui.button(egui::RichText::new(self.localization.get_text("exit"))
            .color(self.colors.text)
            .size(16.0))
            .clicked() {
            std::process::exit(0);
        }
    }
}

/// Реализация главного цикла приложения
impl eframe::App for CheckersGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Заголовок игры
                ui.heading(egui::RichText::new(self.localization.get_text("checkers_title"))
                    .color(self.colors.highlight)
                    .size(28.0));
                
                ui.add_space(20.0);
                
                // Игровое поле
                self.draw_board(ui);
                
                // Статус игры
                self.draw_status(ui);
                
                // Элементы управления
                self.draw_controls(ui);
            });
        });
    }
}

// === РЕАЛИЗАЦИЯ КЛОНИРОВАНИЯ ===

impl Clone for CheckersGUI {
    fn clone(&self) -> Self {
        Self {
            game: self.game.clone(),
            cell_size: self.cell_size,
            colors: self.colors.clone(),
            localization: self.localization.clone(),
            selected_checker: self.selected_checker,
            possible_moves: self.possible_moves.clone(),
        }
    }
}

impl Clone for CheckersColors {
    fn clone(&self) -> Self {
        Self {
            white_cell: self.white_cell,
            black_cell: self.black_cell,
            white_checker: self.white_checker,
            black_checker: self.black_checker,
            highlight: self.highlight,
            possible_move: self.possible_move,
            selected: self.selected,
            text: self.text,
        }
    }
}
