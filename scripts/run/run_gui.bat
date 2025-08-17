@echo off
echo 🎮 Запуск GUI версии...
echo.

if exist "target\debug\rust_tic_tac_toe.exe" (
    echo ✅ Исполняемый файл найден!
    echo 🚀 Запуск GUI...
    target\debug\rust_tic_tac_toe.exe --gui
) else (
    echo ❌ Исполняемый файл не найден!
    echo.
    echo 🔨 Сначала выполните сборку:
    echo    scripts\build\debug\build_debug.bat
    echo.
    pause
)
