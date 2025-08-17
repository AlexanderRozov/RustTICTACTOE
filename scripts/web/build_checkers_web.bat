@echo off
echo ========================================
echo Сборка веб-версии шашек
echo ========================================
echo.

echo 🌐 Сборка WASM версии...
cargo build --target wasm32-unknown-unknown

if %ERRORLEVEL% EQU 0 (
    echo ✅ WASM сборка успешна!
    echo.
    echo 📦 Упаковка для веб-использования...
    wasm-pack build --target web
    
    if %ERRORLEVEL% EQU 0 (
        echo ✅ Веб-пакет создан успешно!
        echo.
        echo 🌐 Откройте checkers.html в браузере
    ) else (
        echo ❌ Ошибка создания веб-пакета!
    )
) else (
    echo ❌ Ошибка WASM сборки!
)

echo.
pause
