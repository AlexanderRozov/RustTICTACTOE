Write-Host "Запуск игры в шашки..." -ForegroundColor Green
Write-Host ""
Write-Host "Доступные опции:" -ForegroundColor Yellow
Write-Host "  --gui       - запуск крестиков-ноликов" -ForegroundColor Cyan
Write-Host "  --checkers  - запуск шашек (текущий выбор)" -ForegroundColor Cyan
Write-Host "  (без аргументов) - консольная версия" -ForegroundColor Cyan
Write-Host ""

cargo run -- --checkers

Read-Host "Нажмите Enter для выхода"

