@echo off
echo Сборка веб-версии шашек...
echo.

echo Очистка предыдущей сборки...
if exist pkg rmdir /s /q pkg

echo Сборка WebAssembly...
wasm-pack build --target web

echo.
echo Сборка завершена!
echo Теперь откройте checkers.html в браузере
echo.
pause

