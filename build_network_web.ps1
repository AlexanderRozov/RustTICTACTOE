Write-Host "Сборка веб-версии шашек с сетевым функционалом..." -ForegroundColor Green
Write-Host ""

# Проверяем наличие wasm-pack
try {
    $null = Get-Command wasm-pack -ErrorAction Stop
} catch {
    Write-Host "Ошибка: wasm-pack не найден!" -ForegroundColor Red
    Write-Host "Установите wasm-pack: cargo install wasm-pack" -ForegroundColor Yellow
    Read-Host "Нажмите Enter для выхода"
    exit 1
}

Write-Host "Очистка предыдущих сборок..." -ForegroundColor Yellow
if (Test-Path "pkg") {
    Remove-Item -Recurse -Force "pkg"
}
if (Test-Path "target\wasm32-unknown-unknown") {
    Remove-Item -Recurse -Force "target\wasm32-unknown-unknown"
}

Write-Host "Сборка WASM модуля..." -ForegroundColor Yellow
wasm-pack build --target web --out-dir pkg

if ($LASTEXITCODE -ne 0) {
    Write-Host "Ошибка сборки!" -ForegroundColor Red
    Read-Host "Нажмите Enter для выхода"
    exit 1
}

Write-Host ""
Write-Host "Сборка завершена успешно!" -ForegroundColor Green
Write-Host ""
Write-Host "Файлы созданы в папке pkg:" -ForegroundColor Cyan
Write-Host "- rust_tic_tac_toe.js - JavaScript обертка" -ForegroundColor White
Write-Host "- rust_tic_tac_toe_bg.wasm - WASM модуль" -ForegroundColor White
Write-Host "- package.json - метаданные пакета" -ForegroundColor White
Write-Host ""
Write-Host "Для запуска откройте network_checkers.html в браузере" -ForegroundColor Cyan
Write-Host ""
Read-Host "Нажмите Enter для выхода"
