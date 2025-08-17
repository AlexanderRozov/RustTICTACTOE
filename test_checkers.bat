@echo off
echo Запуск тестов для шашек...
cargo test --package rust_tic_tac_toe --lib checkers_tests::tests
pause
