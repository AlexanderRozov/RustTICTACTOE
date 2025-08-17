@echo off
echo Запуск веб-версии игры "Крестики-нолики"...
echo.

REM Проверяем, что веб-версия собрана
if not exist "pkg\rust_tic_tac_toe.js" (
    echo Ошибка: Веб-версия не собрана!
    echo Сначала запустите build_web.bat
    pause
    exit /b 1
)

REM Запускаем локальный сервер и открываем игру
echo Запускаю локальный сервер на порту 9001...
start /min python -m http.server 9001
timeout /t 2 /nobreak >nul

REM Открываем игру в браузере через HTTP
echo Открываю игру в браузере...
start "" "http://localhost:9001"

echo.
echo Игра открыта в браузере!
echo Игра запущена через локальный сервер на http://localhost:9001
echo Если игра не работает, проверьте:
echo 1. Что Python установлен и доступен в PATH
echo 2. Что порт 9000 не занят другими приложениями
echo.
pause
