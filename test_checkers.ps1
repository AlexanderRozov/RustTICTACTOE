Write-Host "Запуск тестов для шашек..." -ForegroundColor Green
cargo test --package rust_tic_tac_toe --lib checkers_tests::tests
Read-Host "Нажмите Enter для продолжения"
