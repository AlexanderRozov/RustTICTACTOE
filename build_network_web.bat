@echo off
echo Сборка веб-версии шашек с сетевым функционалом...
echo.

REM Проверяем наличие wasm-pack
where wasm-pack >nul 2>nul
if %errorlevel% neq 0 (
    echo Ошибка: wasm-pack не найден!
    echo Установите wasm-pack: cargo install wasm-pack
    pause
    exit /b 1
)

echo Очистка предыдущих сборок...
if exist pkg rmdir /s /q pkg
if exist target\wasm32-unknown-unknown rmdir /s /q target\wasm32-unknown-unknown

echo Сборка WASM модуля...
wasm-pack build --target web --out-dir pkg

if %errorlevel% neq 0 (
    echo Ошибка сборки!
    pause
    exit /b 1
)

echo.
echo Сборка завершена успешно!
echo.
echo Файлы созданы в папке pkg:
echo - rust_tic_tac_toe.js - JavaScript обертка
echo - rust_tic_tac_toe_bg.wasm - WASM модуль
echo - package.json - метаданные пакета
echo.
echo Для запуска откройте network_checkers.html в браузере
echo.
pause
