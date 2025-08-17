@echo off
echo 🧪 Запуск тестов для игры в шашки...
echo.

echo 🔍 Проверка компиляции...
cargo check

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ Компиляция успешна!
    echo.
    echo 🧪 Запуск тестов шашек...
    cargo test --package rust_tic_tac_toe --lib checkers_tests::tests
    
    if %ERRORLEVEL% EQU 0 (
        echo.
        echo 🎉 Все тесты шашек прошли успешно!
    ) else (
        echo.
        echo ⚠️  Некоторые тесты шашек не прошли
    )
) else (
    echo.
    echo ❌ Ошибка компиляции!
    echo Исправьте ошибки перед запуском тестов.
)

echo.
pause
