@echo off
echo 🚀 Запуск релизной версии шашек...
echo.

if exist "target\release\rust_tic_tac_toe.exe" (
    echo ✅ Релизная версия найдена!
    echo 🎯 Запуск игры...
    echo.
    target\release\rust_tic_tac_toe.exe --checkers
) else (
    echo ❌ Релизная версия не найдена!
    echo.
    echo 🔨 Сначала выполните сборку:
    echo    scripts\build\release\build_release.bat
    echo.
    echo или
    echo.
    echo    scripts\build\release\build_all_release.bat
    echo.
    pause
)
