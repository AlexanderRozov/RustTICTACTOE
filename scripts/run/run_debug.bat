@echo off
echo 🚀 Запуск отладочной версии шашек...
echo.

if exist "target\debug\rust_tic_tac_toe.exe" (
    echo ✅ Отладочная версия найдена!
    echo 🎯 Запуск игры...
    echo.
    target\debug\rust_tic_tac_toe.exe --checkers
) else (
    echo ❌ Отладочная версия не найдена!
    echo.
    echo 🔨 Сначала выполните сборку:
    echo    scripts\build\debug\build_debug.bat
    echo.
    echo или
    echo.
    echo    scripts\build\debug\build_debug.ps1
    echo.
    pause
)
