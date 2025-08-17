//! # Графический интерфейс для игры "Крестики-нолики"
//! 
//! Этот модуль содержит GUI версию игры с использованием egui.
//! Предоставляет современный интерфейс с красивыми цветами и анимациями.
//! 
//! ## Использование
//! ```rust
//! use rust_tic_tac_toe::gui::TicTacToeGUI;
//! 
//! let mut gui = TicTacToeGUI::new();
//! gui.run()?;
//! ```
//! 
//! ## Особенности
//! - Современный дизайн с темной темой
//! - Адаптивный интерфейс
//! - Управление мышью
//! - Красивые цвета для X и O

use eframe::egui;
use crate::TicTacToe;
use rust_tic_tac_toe::{Localization, Language};

/// Основная структура графического интерфейса игры
/// 
/// Содержит игровую логику, настройки отображения и цветовую схему.
/// Обрабатывает пользовательский ввод и отрисовку игрового поля.
pub struct TicTacToeGUI {
    /// Игровая логика
    game: TicTacToe,
    /// Размер одной клетки игрового поля в пикселях
    cell_size: f32,
    /// Цветовая схема интерфейса
    colors: GameColors,
    /// Локализация
    localization: Localization,
}

/// Цветовая схема для графического интерфейса
/// 
/// Использует темную тему в стиле популярных редакторов кода
/// с яркими акцентами для игровых элементов.
struct GameColors {
    /// Цвет фона игрового поля
    background: egui::Color32,
    /// Цвет сетки игрового поля
    grid: egui::Color32,
    /// Цвет символа X (красный)
    x_color: egui::Color32,
    /// Цвет символа O (зеленый)
    o_color: egui::Color32,
    /// Цвет выделения и заголовков (оранжевый)
    highlight: egui::Color32,
    /// Цвет основного текста (белый)
    text: egui::Color32,
}

impl Default for GameColors {
    /// Создает цветовую схему по умолчанию
    /// 
    /// Использует темную тему с яркими акцентами:
    /// - Темно-серый фон
    /// - Светло-серые линии сетки
    /// - Красный для X
    /// - Зеленый для O
    /// - Оранжевый для выделения
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

impl TicTacToeGUI {
    /// Создает новый графический интерфейс
    /// 
    /// Инициализирует игру с настройками по умолчанию:
    /// - Размер клетки: 80 пикселей
    /// - Стандартная цветовая схема
    /// - Новая игра
    pub fn new() -> Self {
        Self {
            game: TicTacToe::new(),
            cell_size: 80.0,
            colors: GameColors::default(),
            localization: Localization::new(Language::English), // По умолчанию английский
        }
    }

    /// Запускает графический интерфейс
    /// 
    /// Создает окно с заданными параметрами и запускает главный цикл.
    /// 
    /// ## Параметры окна
    /// - Начальный размер: 400x500 пикселей
    /// - Минимальный размер: 350x450 пикселей
    /// - Заголовок: "Крестики-нолики"
    pub fn run(&mut self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 500.0])
                .with_min_inner_size([350.0, 450.0])
                .with_title("Крестики-нолики"),
            ..Default::default()
        };
        
        let app = self.clone();
        eframe::run_native(
            "Крестики-нолики",
            options,
            Box::new(|_cc| Box::new(app)),
        )
    }

    /// Отрисовывает игровое поле
    /// 
    /// Создает интерактивную область для игрового поля размером 3x3 клетки.
    /// Обрабатывает клики мыши и отрисовывает все элементы поля.
    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let board_size = self.cell_size * 3.0;
        
        // Создаем область для отрисовки с поддержкой кликов
        let (response, painter) = ui.allocate_painter(
            egui::vec2(board_size, board_size),
            egui::Sense::click()
        );

        // Рисуем фон игрового поля
        painter.rect_filled(
            egui::Rect::from_min_size(
                response.rect.min,
                egui::vec2(board_size, board_size)
            ),
            0.0, // Без скругления углов
            self.colors.background,
        );

        // Рисуем сетку игрового поля
        self.draw_grid(&painter, board_size, response.rect);

        // Рисуем символы X и O
        self.draw_symbols(&painter, response.rect);

        // Обрабатываем клики мыши
        if response.clicked() {
            self.handle_click(&response);
        }
    }

    /// Отрисовывает сетку игрового поля
    /// 
    /// Рисует 4 линии, разделяющие поле на 9 клеток:
    /// - 2 вертикальные линии
    /// - 2 горизонтальные линии
    fn draw_grid(&self, painter: &egui::Painter, board_size: f32, rect: egui::Rect) {
        let stroke = egui::Stroke::new(3.0, self.colors.grid);
        
        // Вертикальные линии (разделяют столбцы)
        for i in 1..3 {
            let x = rect.min.x + i as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.min.y + board_size)],
                stroke,
            );
        }
        
        // Горизонтальные линии (разделяют строки)
        for i in 1..3 {
            let y = rect.min.y + i as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.min.x + board_size, y)],
                stroke,
            );
        }
    }

    /// Отрисовывает символы X и O на игровом поле
    /// 
    /// Проходит по всем клеткам доски и рисует символы игроков
    /// в центре соответствующих клеток.
    fn draw_symbols(&self, painter: &egui::Painter, rect: egui::Rect) {
        for (i, cell) in self.game.get_board().iter().enumerate() {
            if let Some(player) = cell {
                // Вычисляем позицию клетки (строка и столбец)
                let (row, col) = (i / 3, i % 3);
                
                // Вычисляем центр клетки для размещения символа
                let center = egui::pos2(
                    rect.min.x + col as f32 * self.cell_size + self.cell_size / 2.0,
                    rect.min.y + row as f32 * self.cell_size + self.cell_size / 2.0,
                );
                
                // Рисуем соответствующий символ
                match player {
                    rust_tic_tac_toe::Player::X => self.draw_x(painter, center),
                    rust_tic_tac_toe::Player::O => self.draw_o(painter, center),
                }
            }
        }
    }

    /// Отрисовывает символ X
    /// 
    /// Рисует две пересекающиеся линии, образующие крест.
    /// Размер символа составляет 30% от размера клетки.
    fn draw_x(&self, painter: &egui::Painter, center: egui::Pos2) {
        let size = self.cell_size * 0.3;
        let stroke = egui::Stroke::new(4.0, self.colors.x_color);
        
        // Первая диагональ (слева сверху вправо вниз)
        painter.line_segment(
            [egui::pos2(center.x - size, center.y - size), egui::pos2(center.x + size, center.y + size)],
            stroke,
        );
        
        // Вторая диагональ (справа сверху влево вниз)
        painter.line_segment(
            [egui::pos2(center.x + size, center.y - size), egui::pos2(center.x - size, center.y + size)],
            stroke,
        );
    }

    /// Отрисовывает символ O
    /// 
    /// Рисует окружность с толщиной линии 4 пикселя.
    /// Радиус составляет 25% от размера клетки.
    fn draw_o(&self, painter: &egui::Painter, center: egui::Pos2) {
        let radius = self.cell_size * 0.25;
        let stroke = egui::Stroke::new(4.0, self.colors.o_color);
        
        // Рисуем окружность
        painter.circle_stroke(center, radius, stroke);
    }

    /// Обрабатывает клики мыши по игровому полю
    /// 
    /// Преобразует координаты клика в позицию на игровом поле
    /// и делает соответствующий ход, если это возможно.
    /// Ходы не принимаются после окончания игры.
    fn handle_click(&mut self, response: &egui::Response) {
        // Проверяем, не закончена ли игра
        if self.game.is_game_over() {
            return; // Игра закончена - ходы не принимаются
        }
        
        // Получаем позицию клика относительно игрового поля
        let click_pos = response.hover_pos().unwrap();
        let board_pos = click_pos - response.rect.min;
        
        // Вычисляем номер строки и столбца
        let col = (board_pos.x / self.cell_size) as usize;
        let row = (board_pos.y / self.cell_size) as usize;
        
        // Проверяем корректность позиции и делаем ход
        if row < 3 && col < 3 {
            let position = row * 3 + col;
            self.game.make_move(position);
        }
    }

    /// Отрисовывает статус игры
    /// 
    /// Показывает текущего игрока или результат игры:
    /// - "Ход игрока: X/O" - во время игры
    /// - "Победитель: X/O!" - при победе
    /// - "Ничья!" - при ничьей
    fn draw_status(&self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        
        if self.game.is_game_over() {
            // Игра закончена - показываем результат
            match self.game.get_winner() {
                Some(player) => {
                    let text = self.localization.get_text("winner").replace("{}", player.symbol());
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
            // Игра продолжается - показываем текущего игрока
            let text = self.localization.get_text("current_player_turn").replace("{}", self.game.current_player_symbol());
            ui.heading(egui::RichText::new(text)
                .color(self.colors.text)
                .size(20.0));
        }
    }

    /// Отрисовывает элементы управления
    /// 
    /// Создает кнопки для управления игрой:
    /// - "Новая игра" - сбрасывает игру
    /// - "Выход" - закрывает приложение
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
/// 
/// Обрабатывает обновление интерфейса, отрисовку всех элементов
/// и взаимодействие с пользователем.
impl eframe::App for TicTacToeGUI {
    /// Основной метод обновления интерфейса
    /// 
    /// Вызывается каждый кадр для отрисовки интерфейса.
    /// Создает центральную панель с игровым полем и элементами управления.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Центрируем все элементы интерфейса
            ui.vertical_centered(|ui| {
                // Заголовок игры
                ui.heading(egui::RichText::new("Крестики-нолики")
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

impl Clone for TicTacToeGUI {
    /// Создает копию графического интерфейса
    /// 
    /// Требуется для egui, который может создавать несколько экземпляров
    /// приложения для разных окон или контекстов.
    fn clone(&self) -> Self {
        Self {
            game: self.game.clone(),
            cell_size: self.cell_size,
            colors: self.colors.clone(),
            localization: self.localization.clone(),
        }
    }
}

impl Clone for GameColors {
    /// Создает копию цветовой схемы
    /// 
    /// Простое копирование всех цветовых значений.
    fn clone(&self) -> Self {
        Self {
            background: self.background,
            grid: self.grid,
            x_color: self.x_color,
            o_color: self.o_color,
            highlight: self.highlight,
            text: self.text,
        }
    }
}
