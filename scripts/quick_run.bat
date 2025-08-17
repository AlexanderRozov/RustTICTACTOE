@echo off
echo 🎮 БЫСТРЫЙ ЗАПУСК ИГРЫ
echo ================================
echo.
echo Выберите версию для запуска:
echo.
echo 1. 🔨 Отладочная версия
echo 2. 🚀 Релизная версия
echo 3. 🌐 Веб-версия (открыть в браузере)
echo 4. 🎮 Игра в шашки
echo 5. 🖥️ GUI версия
echo 6. ❌ Выход
echo.
set /p choice="Введите номер (1-6): "

if "%choice%"=="1" (
    echo.
    echo 🔨 Запуск отладочной версии...
    call "run\run_debug.bat"
) else if "%choice%"=="2" (
    echo.
    echo 🚀 Запуск релизной версии...
    call "run\run_release.bat"
) else if "%choice%"=="3" (
    echo.
    echo 🌐 Открытие веб-версии...
    if exist "checkers.html" (
        start checkers.html
        echo ✅ Веб-версия открыта в браузере
    ) else (
        echo ❌ Файл checkers.html не найден!
        echo Сначала выполните сборку веб-версии.
    )
) else if "%choice%"=="4" (
    echo.
    echo 🎮 Запуск игры в шашки...
    call "run\run_checkers.bat"
) else if "%choice%"=="5" (
    echo.
    echo 🖥️ Запуск GUI версии...
    call "run\run_gui.bat"
) else if "%choice%"=="6" (
    echo.
    echo 👋 До свидания!
    goto :end
) else (
    echo.
    echo ❌ Неверный выбор!
    echo Введите число от 1 до 6.
    echo.
    pause
    goto :start
)

:end
echo.
pause
