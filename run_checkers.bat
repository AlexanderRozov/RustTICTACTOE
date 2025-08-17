@echo off
echo Запуск игры в шашки...
echo.
echo Доступные опции:
echo   --gui       - запуск крестиков-ноликов
echo   --checkers  - запуск шашек (текущий выбор)
echo   (без аргументов) - консольная версия
echo.
cargo run -- --checkers
pause

