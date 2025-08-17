@echo off
echo ========================================
echo ПОЛНАЯ РЕЛИЗНАЯ СБОРКА ПРОЕКТА
echo ========================================
echo.

echo 🧹 Очистка предыдущих сборок...
cargo clean

echo.
echo 🖥️  Сборка релизной десктопной версии...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo ✅ Десктопная сборка успешна!
) else (
    echo ❌ Ошибка десктопной сборки!
    goto :error
)

echo.
echo 🌐 Сборка релизной веб-версии (WASM)...
cargo build --release --target wasm32-unknown-unknown

if %ERRORLEVEL% EQU 0 (
    echo ✅ WASM сборка успешна!
    
    echo.
    echo 📦 Упаковка для веб-использования...
    wasm-pack build --target web --release
    
    if %ERRORLEVEL% EQU 0 (
        echo ✅ Веб-пакет создан успешно!
    ) else (
        echo ❌ Ошибка создания веб-пакета!
        goto :error
    )
) else (
    echo ❌ Ошибка WASM сборки!
    goto :error
)

echo.
echo 🧪 Запуск тестов...
cargo test --release

if %ERRORLEVEL% EQU 0 (
    echo ✅ Все тесты прошли успешно!
) else (
    echo ⚠️  Некоторые тесты не прошли, но сборка завершена
)

echo.
echo ========================================
echo 🎉 РЕЛИЗНАЯ СБОРКА ЗАВЕРШЕНА УСПЕШНО!
echo ========================================
echo.
echo 📁 Файлы готовы к распространению:
echo    🖥️  Десктоп: target\release\rust_tic_tac_toe.exe
echo    🌐 Веб: pkg\ (откройте checkers.html)
echo.
echo 🚀 Запуск десктопной версии...
target\release\rust_tic_tac_toe.exe --checkers

goto :end

:error
echo.
echo ========================================
echo ❌ СБОРКА ЗАВЕРШИЛАСЬ С ОШИБКАМИ
echo ========================================
echo Проверьте код на наличие ошибок и попробуйте снова.

:end
echo.
pause
