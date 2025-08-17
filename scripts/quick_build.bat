@echo off
echo 🚀 БЫСТРАЯ СБОРКА ПРОЕКТА
echo ================================
echo.
echo Выберите тип сборки:
echo.
echo 1. 🔨 Отладочная сборка
echo 2. 🚀 Релизная сборка
echo 3. 🌐 Веб-версия
echo 4. 🎯 Полная релизная сборка
echo 5. 🎮 Сборка шашек
echo 6. 🌐 Сборка веб-шашек
echo 7. 🎯 Подготовка для itch.io
echo 8. ❌ Выход
echo.
set /p choice="Введите номер (1-8): "

if "%choice%"=="1" (
    echo.
    echo 🔨 Запуск отладочной сборки...
    call "build\debug\build_debug.bat"
) else if "%choice%"=="2" (
    echo.
    echo 🚀 Запуск релизной сборки...
    call "build\release\build_release.bat"
) else if "%choice%"=="3" (
    echo.
    echo 🌐 Запуск сборки веб-версии...
    call "web\build_web.bat"
) else if "%choice%"=="4" (
    echo.
    echo 🎯 Запуск полной релизной сборки...
    call "build\release\build_all_release.bat"
) else if "%choice%"=="5" (
    echo.
    echo 🎮 Запуск сборки шашек...
    call "build\debug\build_checkers.bat"
) else if "%choice%"=="6" (
    echo.
    echo 🌐 Запуск сборки веб-шашек...
    call "web\build_checkers_web.bat"
) else if "%choice%"=="7" (
    echo.
    echo 🎯 Запуск подготовки для itch.io...
    call "utils\prepare_itchio.bat"
) else if "%choice%"=="8" (
    echo.
    echo 👋 До свидания!
    goto :end
) else (
    echo.
    echo ❌ Неверный выбор!
    echo Введите число от 1 до 8.
    echo.
    pause
    goto :start
)

:end
echo.
pause
