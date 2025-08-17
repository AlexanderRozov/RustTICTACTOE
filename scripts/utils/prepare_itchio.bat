@echo off
echo ========================================
echo Подготовка проекта для itch.io
echo ========================================
echo.

echo 🔨 Сборка релизной версии...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo ✅ Сборка успешна!
    echo.
    echo 🌐 Сборка веб-версии...
    cargo build --release --target wasm32-unknown-unknown
    
    if %ERRORLEVEL% EQU 0 (
        echo ✅ WASM сборка успешна!
        echo.
        echo 📦 Упаковка для веб-использования...
        wasm-pack build --target web --release
        
        if %ERRORLEVEL% EQU 0 (
            echo ✅ Веб-пакет создан!
            echo.
            echo 📁 Файлы готовы для itch.io:
            echo    - target\release\rust_tic_tac_toe.exe
            echo    - pkg\ (веб-версия)
            echo    - checkers.html
            echo.
            echo 🚀 Откройте itchio.html для инструкций
        ) else (
            echo ❌ Ошибка создания веб-пакета!
        )
    ) else (
        echo ❌ Ошибка WASM сборки!
    )
) else (
    echo ❌ Ошибка сборки!
)

echo.
pause
