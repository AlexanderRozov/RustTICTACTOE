@echo off
echo 🧪 Запуск всех тестов проекта...
echo.

echo 🔍 Проверка компиляции...
cargo check

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ Компиляция успешна!
    echo.
    echo 🧪 Запуск тестов...
    cargo test
    
    if %ERRORLEVEL% EQU 0 (
        echo.
        echo 🎉 Все тесты прошли успешно!
    ) else (
        echo.
        echo ⚠️  Некоторые тесты не прошли
    )
) else (
    echo.
    echo ❌ Ошибка компиляции!
    echo Исправьте ошибки перед запуском тестов.
)

echo.
pause
