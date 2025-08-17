Write-Host "Запуск веб-версии игры 'Крестики-нолики'..." -ForegroundColor Green
Write-Host ""

# Проверяем, что веб-версия собрана
if (-not (Test-Path "pkg\rust_tic_tac_toe.js")) {
    Write-Host "Ошибка: Веб-версия не собрана!" -ForegroundColor Red
    Write-Host "Сначала запустите build_web.ps1" -ForegroundColor Yellow
    Read-Host "Press Enter to continue"
    exit 1
}

# Открываем игру в браузере по умолчанию
Write-Host "Открываю игру в браузере..." -ForegroundColor Cyan
Start-Process "index.html"

Write-Host ""
Write-Host "Игра открыта в браузере!" -ForegroundColor Green
Write-Host "Если игра не работает, попробуйте:" -ForegroundColor Yellow
Write-Host "1. Запустить локальный сервер: python -m http.server 8080" -ForegroundColor White
Write-Host "2. Открыть http://localhost:8080" -ForegroundColor White
Write-Host ""
Read-Host "Press Enter to continue"
