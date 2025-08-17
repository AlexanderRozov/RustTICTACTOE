//! # Веб-версия графического интерфейса для игры "Шашки"
//! 
//! Этот модуль содержит веб-версию GUI для игры в шашки.
//! Работает в браузере через WebAssembly.

use wasm_bindgen::prelude::*;
use eframe::{egui, WebRunner};
use crate::checkers::{Checkers, CheckersPlayer, CheckerType};
use crate::{Localization, Language};

/// Веб-версия GUI для игры в шашки
#[wasm_bindgen]
pub struct CheckersWeb {
    runner: WebRunner,
}

#[wasm_bindgen]
impl CheckersWeb {
    /// Создает новый веб-интерфейс для шашек
    #[wasm_bindgen(constructor)]
    pub fn new() -> CheckersWeb {
        let runner = WebRunner::new();
        Self { runner }
    }

    /// Запускает веб-интерфейс для шашек
    pub async fn start(&mut self) -> Result<(), JsValue> {
        let options = eframe::WebOptions {
            follow_system_theme: false,
            default_theme: eframe::Theme::Dark,
            ..Default::default()
        };

        self.runner.start(
            "checkers-canvas",
            options,
            Box::new(|_cc| Box::new(CheckersWebApp::new())),
        ).await?;

        Ok(())
    }

    /// Останавливает веб-интерфейс
    pub fn destroy(&mut self) {
        self.runner.destroy();
    }
}

/// Внутреннее веб-приложение для шашек
struct CheckersWebApp {
    game: Checkers,
    cell_size: f32,
    colors: WebCheckersColors,
    localization: Localization,
    selected_checker: Option<(usize, usize)>,
    possible_moves: Vec<(usize, usize)>,
    /// Сообщение об ошибке для отображения
    error_message: Option<String>,
    /// Флаг, указывающий, что есть обязательные взятия
    has_forced_captures: bool,
}

struct WebCheckersColors {
    white_cell: egui::Color32,
    black_cell: egui::Color32,
    white_checker: egui::Color32,
    black_checker: egui::Color32,
    highlight: egui::Color32,
    possible_move: egui::Color32,
    /// Цвет обязательных взятий
    forced_capture: egui::Color32,
    selected: egui::Color32,
    text: egui::Color32,
}

impl Default for WebCheckersColors {
    fn default() -> Self {
        Self {
            white_cell: egui::Color32::from_rgb(240, 217, 181),    // Светло-бежевый
            black_cell: egui::Color32::from_rgb(181, 136, 99),     // Темно-коричневый
            white_checker: egui::Color32::from_rgb(255, 255, 255), // Белый
            black_checker: egui::Color32::from_rgb(50, 50, 50),    // Темно-серый
            highlight: egui::Color32::from_rgb(255, 184, 108),     // Оранжевый
            possible_move: egui::Color32::from_rgb(144, 238, 144), // Светло-зеленый
            forced_capture: egui::Color32::from_rgb(255, 100, 100), // Красный для обязательных взятий
            selected: egui::Color32::from_rgb(255, 215, 0),        // Золотой
            text: egui::Color32::from_rgb(0, 0, 0),               // Черный
        }
    }
}

impl CheckersWebApp {
    fn new() -> Self {
        let game = Checkers::new();
        let has_forced_captures = game.has_forced_captures();
        
        Self {
            game,
            cell_size: 60.0,
            colors: WebCheckersColors::default(),
            localization: Localization::new(Language::English),
            selected_checker: None,
            possible_moves: Vec::new(),
            error_message: None,
            has_forced_captures,
        }
    }

    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let board_size = self.cell_size * 8.0;
        
        let (response, painter) = ui.allocate_painter(
            egui::vec2(board_size, board_size),
            egui::Sense::click()
        );

        // Фон доски
        painter.rect_filled(
            egui::Rect::from_min_size(
                response.rect.min,
                egui::vec2(board_size, board_size)
            ),
            0.0,
            self.colors.white_cell,
        );

        // Клетки доски
        self.draw_cells(&painter, response.rect);
        
        // Шашки
        self.draw_checkers(&painter, response.rect);
        
        // Выделения
        self.draw_highlights(&painter, response.rect);

        // Клики
        if response.clicked() {
            self.handle_click(&response);
        }
    }

    fn draw_cells(&self, painter: &egui::Painter, rect: egui::Rect) {
        for row in 0..8 {
            for col in 0..8 {
                if crate::checkers::Checkers::is_black_cell(row, col) {
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
                    
                    // Основа шашки
                    let radius = self.cell_size * 0.35;
                    painter.circle_filled(center, radius, color);
                    
                    // Обводка
                    painter.circle_stroke(center, radius, egui::Stroke::new(2.0, self.colors.text));
                    
                    // Корона для дамки
                    if checker.is_king() {
                        self.draw_crown(painter, center, radius);
                    }
                }
            }
        }
    }

    fn draw_crown(&self, painter: &egui::Painter, center: egui::Pos2, radius: f32) {
        let crown_color = self.colors.highlight;
        let crown_size = radius * 0.4;
        
        // Простая корона (треугольник)
        let points = [
            egui::pos2(center.x, center.y - crown_size),
            egui::pos2(center.x - crown_size, center.y + crown_size),
            egui::pos2(center.x + crown_size, center.y + crown_size),
        ];
        
        painter.polygon_filled(&points, crown_color);
        painter.polygon_stroke(&points, egui::Stroke::new(1.0, self.colors.text));
    }

    fn draw_highlights(&self, painter: &egui::Painter, rect: egui::Rect) {
        // Выбранная шашка
        if let Some((row, col)) = self.selected_checker {
            let center = egui::pos2(
                rect.min.x + col as f32 * self.cell_size + self.cell_size / 2.0,
                rect.min.y + row as f32 * self.cell_size + self.cell_size / 2.0,
            );
            
            let radius = self.cell_size * 0.4;
            painter.circle_stroke(center, radius, egui::Stroke::new(3.0, self.colors.selected));
        }
        
        // Возможные ходы
        for (row, col) in &self.possible_moves {
            let center = egui::pos2(
                rect.min.x + col as f32 * self.cell_size + self.cell_size / 2.0,
                rect.min.y + row as f32 * self.cell_size + self.cell_size / 2.0,
            );
            
            let radius = self.cell_size * 0.2;
            
            // Выбираем цвет в зависимости от типа хода
            let move_color = if self.has_forced_captures {
                self.colors.forced_capture // Красный для обязательных взятий
            } else {
                self.colors.possible_move  // Зеленый для обычных ходов
            };
            
            painter.circle_filled(center, radius, move_color);
            painter.circle_stroke(center, radius, egui::Stroke::new(2.0, self.colors.text));
        }
    }

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

    fn handle_cell_click(&mut self, row: usize, col: usize) {
        if !crate::checkers::Checkers::is_black_cell(row, col) {
            return;
        }
        
        // Очищаем предыдущее сообщение об ошибке
        self.error_message = None;
        
        let board = self.game.get_board();
        
        // Клик по шашке текущего игрока
        if let Some(checker) = board[row][col] {
            if checker.player == self.game.current_player {
                self.selected_checker = Some((row, col));
                
                // Обновляем флаг обязательных взятий
                self.has_forced_captures = self.game.has_forced_captures();
                
                // Получаем возможные ходы
                self.possible_moves = self.game.get_possible_moves(row, col)
                    .into_iter()
                    .map(|mv| mv.to)
                    .collect();
                return;
            }
        }
        
        // Клик по пустой клетке
        if let Some((from_row, from_col)) = self.selected_checker {
            if board[row][col].is_none() {
                // Проверяем, есть ли обязательные взятия
                let has_captures = self.game.has_forced_captures();
                let is_capture_move = self.game.is_capture_move(from_row, from_col, row, col);
                
                // Если есть обязательные взятия, но игрок пытается сделать обычный ход
                if has_captures && !is_capture_move {
                    self.error_message = Some(
                        if self.localization.language == Language::Russian {
                            "Обязательно брать шашки противника!".to_string()
                        } else {
                            "You must capture opponent's checkers!".to_string()
                        }
                    );
                    return;
                }
                
                if self.game.make_move(from_row, from_col, row, col) {
                    self.selected_checker = None;
                    self.possible_moves.clear();
                    
                    // Обновляем флаг обязательных взятий
                    self.has_forced_captures = self.game.has_forced_captures();
                    
                    // Если после хода все еще есть обязательные взятия, выбираем новую шашку
                    if self.has_forced_captures {
                        // Автоматически выбираем шашку, которая может продолжить взятие
                        for (r, row_data) in self.game.get_board().iter().enumerate() {
                            for (c, cell) in row_data.iter().enumerate() {
                                if let Some(checker) = cell {
                                    if checker.player == self.game.current_player {
                                        if !self.game.get_possible_captures(r, c).is_empty() {
                                            self.selected_checker = Some((r, c));
                                            self.possible_moves = self.game.get_possible_captures(r, c)
                                                .into_iter()
                                                .map(|mv| mv.to)
                                                .collect();
                                            break;
                                        }
                                    }
                                }
                            }
                            if self.selected_checker.is_some() {
                                break;
                            }
                        }
                    }
                } else {
                    // Ход не удался
                    self.error_message = Some(
                        if self.localization.language == Language::Russian {
                            "Некорректный ход!".to_string()
                        } else {
                            "Invalid move!".to_string()
                        }
                    );
                }
            }
        }
    }

    fn draw_status(&self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        
        // Отображаем сообщение об ошибке, если есть
        if let Some(error_msg) = &self.error_message {
            ui.heading(egui::RichText::new(error_msg)
                .color(egui::Color32::from_rgb(255, 100, 100))
                .size(18.0));
            ui.add_space(10.0);
        }
        
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
            
            let status_text = if self.has_forced_captures {
                if self.localization.language == Language::Russian {
                    format!("{} (обязательно брать!)", text)
                } else {
                    format!("{} (must capture!)", text)
                }
            } else {
                text.to_string()
            };
            
            ui.heading(egui::RichText::new(status_text)
                .color(if self.has_forced_captures { self.colors.forced_capture } else { self.colors.text })
                .size(20.0));
        }
    }

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
        
        if ui.button(egui::RichText::new(self.localization.get_text("new_game"))
            .color(self.colors.text)
            .size(16.0))
            .clicked() {
            self.game.reset();
            self.selected_checker = None;
            self.possible_moves.clear();
            self.error_message = None;
            self.has_forced_captures = false;
        }
    }
}

impl eframe::App for CheckersWebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new(self.localization.get_text("checkers_title"))
                    .color(self.colors.highlight)
                    .size(28.0));
                
                ui.add_space(20.0);
                
                self.draw_board(ui);
                self.draw_status(ui);
                self.draw_controls(ui);
            });
        });
    }
}
