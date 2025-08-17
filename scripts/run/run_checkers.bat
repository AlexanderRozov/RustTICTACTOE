@echo off
echo 🎮 Запуск игры в шашки...
echo.

if exist "target\debug\rust_tic_tac_toe.exe" (
    echo ✅ Исполняемый файл найден!
    echo 🚀 Запуск игры...
    target\debug\rust_tic_tac_toe.exe --checkers
) else (
    echo ❌ Исполняемый файл не найден!
    echo.
    echo 🔨 Сначала выполните сборку:
    echo    scripts\build\debug\build_checkers.bat
    echo.
    pause
)
