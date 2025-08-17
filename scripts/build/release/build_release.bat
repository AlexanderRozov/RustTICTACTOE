@echo off
echo ========================================
echo Сборка релизной версии шашек
echo ========================================
echo.

echo Очистка предыдущих сборок...
cargo clean

echo.
echo Сборка релизной версии...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ Сборка успешно завершена!
    echo.
    echo 📁 Файлы находятся в: target\release\
    echo 🎯 Исполняемый файл: rust_tic_tac_toe.exe
    echo 📊 Размер файла:
    dir target\release\rust_tic_tac_toe.exe | findstr "rust_tic_tac_toe.exe"
    echo.
    echo 🚀 Запуск игры...
    target\release\rust_tic_tac_toe.exe --checkers
) else (
    echo.
    echo ❌ Ошибка сборки!
    echo Проверьте код на наличие ошибок.
)

echo.
pause
