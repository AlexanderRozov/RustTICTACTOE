Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Подготовка игры для публикации на itch.io" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Проверяем наличие Rust и wasm-pack
Write-Host "Проверяю зависимости..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version
    Write-Host "Rust найден: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Ошибка: Rust не установлен!" -ForegroundColor Red
    Write-Host "Установите Rust с https://rustup.rs/" -ForegroundColor Yellow
    Read-Host "Press Enter to continue"
    exit 1
}

try {
    $wasmPackVersion = wasm-pack --version
    Write-Host "wasm-pack найден: $wasmPackVersion" -ForegroundColor Green
} catch {
    Write-Host "Устанавливаю wasm-pack..." -ForegroundColor Yellow
    cargo install wasm-pack
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Ошибка установки wasm-pack!" -ForegroundColor Red
        Read-Host "Press Enter to continue"
        exit 1
    }
}

Write-Host ""
Write-Host "Собираю веб-версию..." -ForegroundColor Yellow
wasm-pack build --target web --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Ошибка сборки!" -ForegroundColor Red
    Read-Host "Press Enter to continue"
    exit 1
}

Write-Host ""
Write-Host "Создаю архив для itch.io..." -ForegroundColor Yellow

# Удаляем старые файлы
if (Test-Path "itchio_package.zip") {
    Remove-Item "itchio_package.zip" -Force
}

if (Test-Path "itchio_package") {
    Remove-Item "itchio_package" -Recurse -Force
}

# Создаем папку для пакета
New-Item -ItemType Directory -Name "itchio_package" | Out-Null
New-Item -ItemType Directory -Name "itchio_package\pkg" | Out-Null

# Копируем необходимые файлы
Copy-Item "itchio.html" "itchio_package\index.html"
Copy-Item "pkg\*" "itchio_package\pkg\" -Recurse
Copy-Item "LICENSE" "itchio_package\"
Copy-Item "Cargo.toml" "itchio_package\"

# Создаем README для itch.io
$readmeContent = @"
# Крестики-нолики - Rust + WebAssembly

Классическая игра "Крестики-нолики", написанная на Rust и скомпилированная в WebAssembly.

## Особенности
- Нативная производительность Rust в браузере
- Современный и красивый интерфейс
- Работает на всех устройствах
- Полностью на русском языке

## Технологии
- Rust - игровая логика
- WebAssembly - компиляция в браузер
- egui - графический интерфейс

## Управление
- Клик по клетке для хода
- Кнопка "Новая игра" для перезапуска
"@

$readmeContent | Out-File -FilePath "itchio_package\README.md" -Encoding UTF8

# Создаем архив
Compress-Archive -Path "itchio_package\*" -DestinationPath "itchio_package.zip" -Force

if ($LASTEXITCODE -ne 0) {
    Write-Host "Ошибка создания архива!" -ForegroundColor Red
    Read-Host "Press Enter to continue"
    exit 1
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Готово! Пакет создан: itchio_package.zip" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Что делать дальше:" -ForegroundColor Yellow
Write-Host "1. Загрузите itchio_package.zip на itch.io" -ForegroundColor White
Write-Host "2. Установите тип проекта: HTML" -ForegroundColor White
Write-Host "3. Настройте метаданные и скриншоты" -ForegroundColor White
Write-Host "4. Опубликуйте игру!" -ForegroundColor White
Write-Host ""
Write-Host "Файлы в пакете:" -ForegroundColor Yellow
Get-ChildItem "itchio_package" -Recurse | Select-Object Name, FullName
Write-Host ""
Read-Host "Press Enter to continue"

