//! # Тесты для игры в шашки (исправленная версия)
//! 
//! Этот модуль содержит все тесты для проверки корректности игровой логики шашек.

use crate::checkers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Checkers::new();
        assert!(!game.is_game_over());
        assert_eq!(game.current_player_symbol(), "⚪");
        assert_eq!(game.get_winner(), None);
    }

    #[test]
    fn test_initial_board_setup() {
        let game = Checkers::new();
        let board = game.get_board();
        
        // Проверяем, что белые шашки в верхних рядах
        for row in 0..3 {
            for col in 0..8 {
                if Checkers::is_black_cell(row, col) {
                    assert!(board[row][col].is_some());
                    assert_eq!(board[row][col].unwrap().player, CheckersPlayer::White);
                }
            }
        }
        
        // Проверяем, что черные шашки в нижних рядах
        for row in 5..8 {
            for col in 0..8 {
                if Checkers::is_black_cell(row, col) {
                    assert!(board[row][col].is_some());
                    assert_eq!(board[row][col].unwrap().player, CheckersPlayer::Black);
                }
            }
        }
    }

    #[test]
    fn test_valid_move() {
        let mut game = Checkers::new();
        
        // Белая шашка ходит вперед по диагонали
        assert!(game.make_move(2, 1, 3, 2));
        assert_eq!(game.current_player_symbol(), "⚫");
    }

    #[test]
    fn test_invalid_move() {
        let mut game = Checkers::new();
        
        // Попытка хода на белую клетку
        assert!(!game.make_move(2, 1, 3, 1));
        
        // Попытка хода не по диагонали
        assert!(!game.make_move(2, 1, 2, 3));
        
        // Попытка хода назад для обычной шашки
        assert!(!game.make_move(5, 0, 4, 1));
    }

    #[test]
    fn test_promotion_to_king() {
        let mut game = Checkers::new();
        
        // Убираем все шашки кроме одной белой в предпоследнем ряду
        game.clear_board();
        game.set_checker(6, 1, Some(Checker::new(CheckersPlayer::White)));
        
        // Делаем ход в последний ряд
        assert!(game.make_move(6, 1, 7, 0));
        
        // Проверяем, что шашка стала дамкой
        if let Some(checker) = game.get_checker_at(7, 0) {
            assert!(checker.is_king());
        } else {
            panic!("Шашка должна быть на позиции 7,0");
        }
    }

    #[test]
    fn test_black_cell_detection() {
        assert!(Checkers::is_black_cell(0, 1));
        assert!(Checkers::is_black_cell(1, 0));
        assert!(!Checkers::is_black_cell(0, 0));
        assert!(!Checkers::is_black_cell(1, 1));
    }
    
    #[test]
    fn test_forced_capture() {
        let mut game = Checkers::new();
        
        // Создаем ситуацию, где белая шашка может взять черную
        // Убираем все шашки кроме нужных
        game.clear_board();
        
        // Белая шашка на позиции (2, 1)
        game.set_checker(2, 1, Some(Checker::new(CheckersPlayer::White)));
        // Черная шашка на позиции (3, 2)
        game.set_checker(3, 2, Some(Checker::new(CheckersPlayer::Black)));
        // Пустая клетка на позиции (4, 3)
        
        // Проверяем, что есть обязательное взятие
        assert!(game.has_forced_captures());
        
        // Проверяем, что обычный ход заблокирован
        assert!(!game.make_move(2, 1, 3, 0));
        
        // Проверяем, что взятие разрешено
        assert!(game.make_move(2, 1, 4, 3));
        
        // Проверяем, что черная шашка удалена
        assert!(game.get_checker_at(3, 2).is_none());
        
        // Проверяем, что белая шашка перемещена
        assert!(game.get_checker_at(4, 3).is_some());
        assert_eq!(game.get_checker_at(4, 3).unwrap().player, CheckersPlayer::White);
    }
    
    #[test]
    fn test_multiple_captures() {
        let mut game = Checkers::new();
        
        // Создаем ситуацию для множественного взятия
        game.clear_board();
        
        // Белая шашка на позиции (1, 0)
        game.set_checker(1, 0, Some(Checker::new(CheckersPlayer::White)));
        // Черные шашки на позициях (2, 1) и (4, 3)
        game.set_checker(2, 1, Some(Checker::new(CheckersPlayer::Black)));
        game.set_checker(4, 3, Some(Checker::new(CheckersPlayer::Black)));
        // Пустые клетки на позициях (3, 2) и (5, 4)
        
        // Первое взятие
        assert!(game.make_move(1, 0, 3, 2));
        
        // Проверяем, что игрок не переключился (может продолжить взятие)
        assert_eq!(game.current_player, CheckersPlayer::White);
        
        // Второе взятие
        assert!(game.make_move(3, 2, 5, 4));
        
        // После второго взятия игра должна закончиться, потому что у черных не остается шашек
        assert!(game.is_game_over());
        assert_eq!(game.get_winner(), Some(CheckersPlayer::White));
        
        // Проверяем, что все черные шашки удалены
        assert!(game.get_checker_at(2, 1).is_none());
        assert!(game.get_checker_at(4, 3).is_none());
        
        // Проверяем, что белая шашка на конечной позиции
        assert!(game.get_checker_at(5, 4).is_some());
        assert_eq!(game.get_checker_at(5, 4).unwrap().player, CheckersPlayer::White);
    }
    
    #[test]
    fn test_capture_validation() {
        let mut game = Checkers::new();
        
        // Создаем ситуацию для проверки валидации взятий
        game.clear_board();
        
        // Белая шашка на позиции (2, 1)
        game.set_checker(2, 1, Some(Checker::new(CheckersPlayer::White)));
        // Черная шашка на позиции (3, 2)
        game.set_checker(3, 2, Some(Checker::new(CheckersPlayer::Black)));
        
        // Попытка взять шашку на расстоянии 1 (некорректно)
        assert!(!game.make_move(2, 1, 3, 2));
        
        // Попытка взять шашку на расстоянии 3 (некорректно)
        assert!(!game.make_move(2, 1, 5, 5));
        
        // Попытка взять свою шашку (некорректно)
        game.set_checker(3, 2, Some(Checker::new(CheckersPlayer::White)));
        assert!(!game.make_move(2, 1, 4, 3));
    }
    
    #[test]
    fn test_regular_checker_cannot_capture_backwards() {
        let mut game = Checkers::new();
        
        // Создаем ситуацию, где белая шашка не может рубить назад
        game.clear_board();
        
        // Белая шашка на позиции (5, 2)
        game.set_checker(5, 2, Some(Checker::new(CheckersPlayer::White)));
        // Черная шашка на позиции (4, 3) - ПЕРЕД белой
        game.set_checker(4, 3, Some(Checker::new(CheckersPlayer::Black)));
        // Пустая клетка на позиции (3, 4) - ЗА черной
        
        // Проверяем, что белая шашка НЕ может рубить назад (вперед для черных)
        let captures = game.get_possible_captures(5, 2);
        assert!(captures.is_empty(), "Обычная шашка не должна рубить назад");
        
        // Проверяем, что обычный ход вперед разрешен
        assert!(game.make_move(5, 2, 4, 1));
    }
    
    #[test]
    fn test_king_can_capture_to_any_position_on_diagonal() {
        let mut game = Checkers::new();
        
        // Создаем ситуацию для дамки
        game.clear_board();
        
        // Белая дамка на позиции (3, 2)
        let mut white_king = Checker::new(CheckersPlayer::White);
        white_king.promote_to_king();
        game.set_checker(3, 2, Some(white_king));
        
        // Черная шашка на позиции (4, 3)
        game.set_checker(4, 3, Some(Checker::new(CheckersPlayer::Black)));
        // Пустые клетки на позициях (5, 4), (6, 5), (7, 6)
        
        // Проверяем, что дамка может приземлиться на любую из этих позиций
        let captures = game.get_possible_captures(3, 2);
        assert_eq!(captures.len(), 3, "Дамка должна иметь 3 варианта приземления");
        
        // Проверяем, что можно приземлиться на самую дальнюю позицию
        assert!(game.make_move(3, 2, 7, 6));
        
        // Проверяем, что черная шашка удалена
        assert!(game.get_checker_at(4, 3).is_none());
        
        // Проверяем, что белая дамка перемещена
        assert!(game.get_checker_at(7, 6).is_some());
        assert!(game.get_checker_at(7, 6).unwrap().is_king());
    }
    
    #[test]
    fn test_king_cannot_capture_over_obstacles() {
        let mut game = Checkers::new();
        
        // Создаем ситуацию для дамки с препятствием
        game.clear_board();
        
        // Белая дамка на позиции (2, 1)
        let mut white_king = Checker::new(CheckersPlayer::White);
        white_king.promote_to_king();
        game.set_checker(2, 1, Some(white_king));
        
        // Черная шашка на позиции (3, 2)
        game.set_checker(3, 2, Some(Checker::new(CheckersPlayer::Black)));
        // Препятствие на позиции (5, 4) - блокирует дальние позиции
        game.set_checker(5, 4, Some(Checker::new(CheckersPlayer::White)));
        // Пустые клетки на позициях (4, 3), (6, 5) - только до препятствия
        
        // Проверяем, что дамка может приземлиться только до препятствия
        let captures = game.get_possible_captures(2, 1);
        assert_eq!(captures.len(), 2, "Дамка должна иметь 2 варианта приземления (до препятствия)");
        
        // Проверяем, что можно приземлиться на позицию (4, 3)
        assert!(game.make_move(2, 1, 4, 3));
    }

    #[test]
    fn test_move_logging() {
        let mut game = Checkers::new();
        
        // Делаем несколько ходов для проверки логгирования
        println!("Начинаем тест логгирования...");
        
        // Первый ход: белые
        assert!(game.make_move(5, 0, 4, 1));
        assert_eq!(game.move_history_len(), 1);
        
        // Проверяем первый ход в истории
        let first_move = game.get_move_at(0).unwrap();
        assert_eq!(first_move.move_number, 1);
        assert_eq!(first_move.player, CheckersPlayer::White);
        assert_eq!(first_move.from, (5, 0));
        assert_eq!(first_move.to, (4, 1));
        assert!(matches!(first_move.move_type, MoveType::Regular));
        assert!(first_move.captured.is_empty());
        
        // Второй ход: черные
        assert!(game.make_move(2, 1, 3, 2));
        assert_eq!(game.move_history_len(), 2);
        
        // Проверяем второй ход в истории
        let second_move = game.get_move_at(1).unwrap();
        assert_eq!(second_move.move_number, 2);
        assert_eq!(second_move.player, CheckersPlayer::Black);
        assert_eq!(second_move.from, (2, 1));
        assert_eq!(second_move.to, (3, 2));
        assert!(matches!(second_move.move_type, MoveType::Regular));
        assert!(second_move.captured.is_empty());
        
        // Проверяем отображение истории
        game.display_move_history();
        game.display_game_state();
        
        println!("Тест логгирования завершен успешно!");
    }

    #[test]
    fn test_black_king_simple_moves() {
        let mut game = Checkers::new();
        
        // Убираем все шашки
        game.clear_board();
        
        // Создаем черную дамку в позиции (3, 0) - первый столбец
        let mut black_king = Checker::new(CheckersPlayer::Black);
        black_king.promote_to_king();
        game.set_checker(3, 0, Some(black_king));
        
        // Устанавливаем текущего игрока как черные
        game.current_player = CheckersPlayer::Black;
        
        // Получаем все возможные ходы
        let possible_moves = game.get_possible_moves(3, 0);
        
        // Проверяем, что дамка может ходить вправо-вверх (к позиции 0, 3)
        let up_right_move = possible_moves.iter().find(|m| m.to == (0, 3));
        assert!(up_right_move.is_some(), "Черная дамка должна иметь возможность ходить вправо-вверх");
        
        // Проверяем, что дамка может ходить вправо-вниз (к позиции 6, 3)
        let down_right_move = possible_moves.iter().find(|m| m.to == (6, 3));
        assert!(down_right_move.is_some(), "Черная дамка должна иметь возможность ходить вправо-вниз");
        
        // Проверяем, что дамка НЕ может ходить влево (это выходит за пределы доски)
        let left_moves = possible_moves.iter().filter(|m| m.to.1 < 0).count();
        assert_eq!(left_moves, 0, "Черная дамка не должна иметь возможность ходить влево из первого столбца");
        
        println!("Тест простых ходов черной дамки завершен успешно!");
    }

    #[test]
    fn test_black_king_first_column_moves() {
        let mut game = Checkers::new();
        
        // Убираем все шашки
        game.clear_board();
        
        // Создаем черную дамку в позиции (3, 0) - первый столбец
        let mut black_king = Checker::new(CheckersPlayer::Black);
        black_king.promote_to_king();
        game.set_checker(3, 0, Some(black_king));
        
        // Устанавливаем текущего игрока как черные
        game.current_player = CheckersPlayer::Black;
        
        // Получаем все возможные ходы
        let possible_moves = game.get_possible_moves(3, 0);
        
        // Проверяем, что у дамки есть ходы (не пустой список)
        assert!(!possible_moves.is_empty(), "Черная дамка в первом столбце должна иметь возможность ходить");
        
        // Проверяем, что все ходы ведут вправо (col > 0)
        for movement in &possible_moves {
            assert!(movement.to.1 > 0, "Все ходы должны вести вправо из первого столбца");
        }
        
        println!("Тест ходов черной дамки в первом столбце завершен успешно!");
        println!("Найдено ходов: {}", possible_moves.len());
        for movement in &possible_moves {
            println!("  От ({},{}) к ({},{})", movement.from.0, movement.from.1, movement.to.0, movement.to.1);
        }
    }
}
