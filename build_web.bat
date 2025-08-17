@echo off
echo Building Rust Tic-Tac-Toe for Web...

REM Проверяем, установлен ли wasm-pack
wasm-pack --version >nul 2>&1
if errorlevel 1 (
    echo wasm-pack не установлен. Устанавливаем...
    cargo install wasm-pack
)

REM Собираем веб-версию
echo Сборка WASM...
wasm-pack build --target web

if errorlevel 1 (
    echo Ошибка сборки!
    pause
    exit /b 1
)

echo.
echo Сборка завершена успешно!
echo.
echo Для запуска веб-версии:
echo 1. Откройте index.html в браузере
echo 2. Или запустите локальный сервер:
echo    python -m http.server 8000
echo    затем откройте http://localhost:8000
echo.
pause
