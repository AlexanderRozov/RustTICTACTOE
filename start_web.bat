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

REM Открываем игру в браузере по умолчанию
echo Открываю игру в браузере...
start "" "index.html"

echo.
echo Игра открыта в браузере!
echo Если игра не работает, попробуйте:
echo 1. Запустить локальный сервер: python -m http.server 8080
echo 2. Открыть http://localhost:8080
echo.
pause
