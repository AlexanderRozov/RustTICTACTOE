Write-Host "Сборка веб-версии шашек..." -ForegroundColor Green
Write-Host ""

Write-Host "Очистка предыдущей сборки..." -ForegroundColor Yellow
if (Test-Path "pkg") {
    Remove-Item -Recurse -Force "pkg"
}

Write-Host "Сборка WebAssembly..." -ForegroundColor Yellow
wasm-pack build --target web

Write-Host ""
Write-Host "Сборка завершена!" -ForegroundColor Green
Write-Host "Теперь откройте checkers.html в браузере" -ForegroundColor Cyan
Write-Host ""

Read-Host "Нажмите Enter для выхода"

