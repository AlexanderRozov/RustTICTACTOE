@echo off
echo ========================================
echo Сборка релизной веб-версии шашек
echo ========================================
echo.

echo Очистка предыдущих сборок...
cargo clean

echo.
echo Сборка релизной WASM версии...
cargo build --release --target wasm32-unknown-unknown

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ WASM сборка успешна!
    echo.
    echo Упаковка для веб-использования...
    wasm-pack build --target web --release
    
    if %ERRORLEVEL% EQU 0 (
        echo.
        echo ✅ Веб-пакет создан успешно!
        echo.
        echo 📁 Файлы находятся в: pkg\
        echo 🎯 Основные файлы:
        echo    - rust_tic_tac_toe_bg.wasm (WASM модуль)
        echo    - rust_tic_tac_toe.js (JavaScript обертка)
        echo    - rust_tic_tac_toe.d.ts (TypeScript типы)
        echo.
        echo 📊 Размеры файлов:
        dir pkg\*.wasm | findstr ".wasm"
        dir pkg\*.js | findstr ".js"
        echo.
        echo 🌐 Откройте checkers.html в браузере для тестирования
    ) else (
        echo.
        echo ❌ Ошибка создания веб-пакета!
        echo Убедитесь, что установлен wasm-pack
    )
) else (
    echo.
    echo ❌ Ошибка WASM сборки!
    echo Проверьте код на наличие ошибок.
)

echo.
pause
