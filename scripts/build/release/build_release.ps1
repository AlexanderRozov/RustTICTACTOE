Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Сборка релизной версии шашек" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Очистка предыдущих сборок..." -ForegroundColor Yellow
cargo clean

Write-Host ""
Write-Host "Сборка релизной версии..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Сборка успешно завершена!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📁 Файлы находятся в: target\release\" -ForegroundColor White
    Write-Host "🎯 Исполняемый файл: rust_tic_tac_toe.exe" -ForegroundColor White
    
    $exePath = "target\release\rust_tic_tac_toe.exe"
    if (Test-Path $exePath) {
        $fileInfo = Get-Item $exePath
        Write-Host "📊 Размер файла: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor White
        Write-Host "📅 Дата сборки: $($fileInfo.LastWriteTime)" -ForegroundColor White
    }
    
    Write-Host ""
    Write-Host "🚀 Запуск игры..." -ForegroundColor Green
    & $exePath --checkers
} else {
    Write-Host ""
    Write-Host "❌ Ошибка сборки!" -ForegroundColor Red
    Write-Host "Проверьте код на наличие ошибок." -ForegroundColor Red
}

Write-Host ""
Read-Host "Нажмите Enter для продолжения"
