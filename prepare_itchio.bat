@echo off
echo ========================================
echo Подготовка игры для публикации на itch.io
echo ========================================
echo.

REM Проверяем наличие Rust и wasm-pack
echo Проверяю зависимости...
rustc --version >nul 2>&1
if errorlevel 1 (
    echo Ошибка: Rust не установлен!
    echo Установите Rust с https://rustup.rs/
    pause
    exit /b 1
)

wasm-pack --version >nul 2>&1
if errorlevel 1 (
    echo Устанавливаю wasm-pack...
    cargo install wasm-pack
    if errorlevel 1 (
        echo Ошибка установки wasm-pack!
        pause
        exit /b 1
    )
)

echo.
echo Собираю веб-версию...
wasm-pack build --target web --release

if errorlevel 1 (
    echo Ошибка сборки!
    pause
    exit /b 1
)

echo.
echo Создаю архив для itch.io...
if exist itchio_package.zip del itchio_package.zip

REM Создаем папку для пакета
if exist itchio_package rmdir /s /q itchio_package
mkdir itchio_package

REM Копируем необходимые файлы
copy itchio.html itchio_package\index.html
xcopy pkg\*.* itchio_package\pkg\ /E /I /Y
copy LICENSE itchio_package\
copy Cargo.toml itchio_package\

REM Создаем README для itch.io
echo # Крестики-нолики - Rust + WebAssembly > itchio_package\README.md
echo. >> itchio_package\README.md
echo Классическая игра "Крестики-нолики", написанная на Rust и скомпилированная в WebAssembly. >> itchio_package\README.md
echo. >> itchio_package\README.md
echo ## Особенности >> itchio_package\README.md
echo - Нативная производительность Rust в браузере >> itchio_package\README.md
echo - Современный и красивый интерфейс >> itchio_package\README.md
echo - Работает на всех устройствах >> itchio_package\README.md
echo - Полностью на русском языке >> itchio_package\README.md
echo. >> itchio_package\README.md
echo ## Технологии >> itchio_package\README.md
echo - Rust - игровая логика >> itchio_package\README.md
echo - WebAssembly - компиляция в браузер >> itchio_package\README.md
echo - egui - графический интерфейс >> itchio_package\README.md
echo. >> itchio_package\README.md
echo ## Управление >> itchio_package\README.md
echo - Клик по клетке для хода >> itchio_package\README.md
echo - Кнопка "Новая игра" для перезапуска >> itchio_package\README.md

REM Создаем архив
powershell -command "Compress-Archive -Path 'itchio_package\*' -DestinationPath 'itchio_package.zip' -Force"

if errorlevel 1 (
    echo Ошибка создания архива!
    pause
    exit /b 1
)

echo.
echo ========================================
echo Готово! Пакет создан: itchio_package.zip
echo ========================================
echo.
echo Что делать дальше:
echo 1. Загрузите itchio_package.zip на itch.io
echo 2. Установите тип проекта: HTML
echo 3. Настройте метаданные и скриншоты
echo 4. Опубликуйте игру!
echo.
echo Файлы в пакете:
dir itchio_package /b
echo.
pause
