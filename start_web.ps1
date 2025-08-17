Write-Host "Запуск веб-версии игры 'Крестики-нолики'..." -ForegroundColor Green
Write-Host ""

# Проверяем, что веб-версия собрана
if (-not (Test-Path "pkg\rust_tic_tac_toe.js")) {
    Write-Host "Ошибка: Веб-версия не собрана!" -ForegroundColor Red
    Write-Host "Сначала запустите build_web.ps1" -ForegroundColor Yellow
    Read-Host "Press Enter to continue"
    exit 1
}

# Запускаем локальный сервер и открываем игру
Write-Host "Запускаю локальный сервер на порту 9001..." -ForegroundColor Cyan
Start-Process python -ArgumentList "-m", "http.server", "9001" -WindowStyle Minimized

# Ждем запуска сервера
Write-Host "Ожидаю запуска сервера..." -ForegroundColor Yellow
Start-Sleep -Seconds 2

# Открываем игру в браузере через HTTP
Write-Host "Открываю игру в браузере..." -ForegroundColor Cyan
Start-Process "http://localhost:9001"

Write-Host ""
Write-Host "Игра открыта в браузере!" -ForegroundColor Green
Write-Host "Игра запущена через локальный сервер на http://localhost:9001" -ForegroundColor Green
Write-Host "Если игра не работает, проверьте:" -ForegroundColor Yellow
Write-Host "1. Что Python установлен и доступен в PATH" -ForegroundColor White
Write-Host "2. Что порт 9000 не занят другими приложениями" -ForegroundColor White
Write-Host ""
Read-Host "Press Enter to continue"
