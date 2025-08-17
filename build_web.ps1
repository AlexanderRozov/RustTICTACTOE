Write-Host "Building Rust Tic-Tac-Toe for Web..." -ForegroundColor Green

# Проверяем, установлен ли wasm-pack
try {
    wasm-pack --version | Out-Null
    Write-Host "wasm-pack найден" -ForegroundColor Green
} catch {
    Write-Host "wasm-pack не установлен. Устанавливаем..." -ForegroundColor Yellow
    cargo install wasm-pack
}

# Собираем веб-версию
Write-Host "Сборка WASM..." -ForegroundColor Green
wasm-pack build --target web

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Сборка завершена успешно!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Для запуска веб-версии:" -ForegroundColor Cyan
    Write-Host "1. Откройте index.html в браузере" -ForegroundColor White
    Write-Host "2. Или запустите локальный сервер:" -ForegroundColor White
    Write-Host "   python -m http.server 8000" -ForegroundColor Yellow
    Write-Host "   затем откройте http://localhost:8000" -ForegroundColor Yellow
} else {
    Write-Host "Ошибка сборки!" -ForegroundColor Red
}

Write-Host ""
Read-Host "Press Enter to continue"
