//! # Веб-версия графического интерфейса
//! 
//! Этот модуль содержит веб-версию GUI для игры "Крестики-нолики".
//! Работает в браузере через WebAssembly.

use wasm_bindgen::prelude::*;
use eframe::{egui, WebRunner};
use crate::TicTacToe;

/// Веб-версия GUI для игры "Крестики-нолики"
#[wasm_bindgen]
pub struct TicTacToeWeb {
    runner: WebRunner,
}

#[wasm_bindgen]
impl TicTacToeWeb {
    /// Создает новый веб-интерфейс
    #[wasm_bindgen(constructor)]
    pub fn new() -> TicTacToeWeb {
        let runner = WebRunner::new();
        Self { runner }
    }

    /// Запускает веб-интерфейс
    pub async fn start(&mut self) -> Result<(), JsValue> {
        let options = eframe::WebOptions {
            follow_system_theme: false,
            default_theme: eframe::Theme::Dark,
            ..Default::default()
        };

        self.runner.start(
            "game-canvas",
            options,
            Box::new(|_cc| Box::new(TicTacToeWebApp::new())),
        ).await?;

        Ok(())
    }

    /// Останавливает веб-интерфейс
    pub fn destroy(&mut self) {
        self.runner.destroy();
    }
}

/// Внутреннее веб-приложение
struct TicTacToeWebApp {
    game: TicTacToe,
    cell_size: f32,
    colors: WebGameColors,
}

struct WebGameColors {
    background: egui::Color32,
    grid: egui::Color32,
    x_color: egui::Color32,
    o_color: egui::Color32,
    highlight: egui::Color32,
    text: egui::Color32,
}

impl Default for WebGameColors {
    fn default() -> Self {
        Self {
            background: egui::Color32::from_rgb(40, 44, 52),    // Темно-серый
            grid: egui::Color32::from_rgb(68, 71, 90),          // Светло-серый
            x_color: egui::Color32::from_rgb(255, 85, 85),      // Красный
            o_color: egui::Color32::from_rgb(80, 250, 123),     // Зеленый
            highlight: egui::Color32::from_rgb(255, 184, 108),  // Оранжевый
            text: egui::Color32::from_rgb(248, 248, 242),       // Белый
        }
    }
}

impl TicTacToeWebApp {
    fn new() -> Self {
        Self {
            game: TicTacToe::new(),
            cell_size: 80.0,
            colors: WebGameColors::default(),
        }
    }

    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let board_size = self.cell_size * 3.0;
        
        let (response, painter) = ui.allocate_painter(
            egui::vec2(board_size, board_size),
            egui::Sense::click()
        );

        // Фон
        painter.rect_filled(
            egui::Rect::from_min_size(
                response.rect.min,
                egui::vec2(board_size, board_size)
            ),
            0.0,
            self.colors.background,
        );

        // Сетка
        self.draw_grid(&painter, board_size, response.rect);

        // Символы
        self.draw_symbols(&painter, response.rect);

        // Клики
        if response.clicked() {
            self.handle_click(&response);
        }
    }

    fn draw_grid(&self, painter: &egui::Painter, board_size: f32, rect: egui::Rect) {
        let stroke = egui::Stroke::new(3.0, self.colors.grid);
        
        // Вертикальные линии
        for i in 1..3 {
            let x = rect.min.x + i as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.min.y + board_size)],
                stroke,
            );
        }
        
        // Горизонтальные линии
        for i in 1..3 {
            let y = rect.min.y + i as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.min.x + board_size, y)],
                stroke,
            );
        }
    }

    fn draw_symbols(&self, painter: &egui::Painter, rect: egui::Rect) {
        for (i, cell) in self.game.get_board().iter().enumerate() {
            if let Some(player) = cell {
                let (row, col) = (i / 3, i % 3);
                let center = egui::pos2(
                    rect.min.x + col as f32 * self.cell_size + self.cell_size / 2.0,
                    rect.min.y + row as f32 * self.cell_size + self.cell_size / 2.0,
                );
                
                match player {
                    crate::Player::X => self.draw_x(painter, center),
                    crate::Player::O => self.draw_o(painter, center),
                }
            }
        }
    }

    fn draw_x(&self, painter: &egui::Painter, center: egui::Pos2) {
        let size = self.cell_size * 0.3;
        let stroke = egui::Stroke::new(4.0, self.colors.x_color);
        
        painter.line_segment(
            [egui::pos2(center.x - size, center.y - size), egui::pos2(center.x + size, center.y + size)],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(center.x + size, center.y - size), egui::pos2(center.x - size, center.y + size)],
            stroke,
        );
    }

    fn draw_o(&self, painter: &egui::Painter, center: egui::Pos2) {
        let radius = self.cell_size * 0.25;
        let stroke = egui::Stroke::new(4.0, self.colors.o_color);
        
        painter.circle_stroke(center, radius, stroke);
    }

    fn handle_click(&mut self, response: &egui::Response) {
        if self.game.is_game_over() {
            return;
        }
        
        let click_pos = response.hover_pos().unwrap();
        let board_pos = click_pos - response.rect.min;
        
        let col = (board_pos.x / self.cell_size) as usize;
        let row = (board_pos.y / self.cell_size) as usize;
        
        if row < 3 && col < 3 {
            let position = row * 3 + col;
            self.game.make_move(position);
        }
    }

    fn draw_status(&self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        
        if self.game.is_game_over() {
            match self.game.get_winner() {
                Some(player) => {
                    ui.heading(egui::RichText::new(format!("Победитель: {}!", player.symbol()))
                        .color(self.colors.highlight)
                        .size(24.0));
                }
                None => {
                    ui.heading(egui::RichText::new("Ничья!")
                        .color(self.colors.highlight)
                        .size(24.0));
                }
            }
        } else {
            ui.heading(egui::RichText::new(format!("Ход игрока: {}", self.game.current_player_symbol()))
                .color(self.colors.text)
                .size(20.0));
        }
    }

    fn draw_controls(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        
        if ui.button(egui::RichText::new("Новая игра")
            .color(self.colors.text)
            .size(16.0))
            .clicked() {
            self.game.reset();
        }
    }
}

impl eframe::App for TicTacToeWebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Крестики-нолики")
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
