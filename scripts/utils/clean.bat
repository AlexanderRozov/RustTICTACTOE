@echo off
echo 🧹 Очистка проекта...
echo.

echo 📁 Удаление папки target...
if exist "target" (
    rmdir /s /q "target"
    echo ✅ Папка target удалена
) else (
    echo ℹ️  Папка target не найдена
)

echo.
echo 📦 Удаление папки pkg...
if exist "pkg" (
    rmdir /s /q "pkg"
    echo ✅ Папка pkg удалена
) else (
    echo ℹ️  Папка pkg не найдена
)

echo.
echo 🔍 Очистка кэша Cargo...
cargo clean

echo.
echo ✅ Очистка завершена!
echo Проект готов к новой сборке.
echo.
pause
