# 📁 **Папка скриптов**

Здесь находятся все скрипты для сборки, запуска и управления проектом.

## 🗂️ **Структура папок:**

```
scripts/
├── build/              # Скрипты сборки
│   ├── debug/         # Отладочная сборка
│   └── release/       # Релизная сборка
├── run/               # Скрипты запуска
├── test/              # Скрипты тестирования
├── web/               # Скрипты для веб-версии
└── utils/             # Утилитарные скрипты
```

## 🎯 **Быстрый старт:**

### **Сборка:**
```bash
# Отладочная сборка
scripts/build/debug/build_debug.bat

# Релизная сборка
scripts/build/release/build_release.bat

# Полная сборка
scripts/build/release/build_all_release.bat
```

### **Запуск:**
```bash
# Отладочная версия
scripts/run/run_debug.bat

# Релизная версия
scripts/run/run_release.bat
```

### **Тестирование:**
```bash
# Запуск всех тестов
scripts/test/run_tests.bat

# Тесты только для шашек
scripts/test/test_checkers.bat
```

## 🔧 **Поддерживаемые оболочки:**

- **Batch (.bat)** - для Windows Command Prompt
- **PowerShell (.ps1)** - для Windows PowerShell

## 📋 **Описание скриптов:**

### **Сборка (build/)**
- `build_debug.bat/.ps1` - отладочная сборка
- `build_release.bat/.ps1` - релизная сборка
- `build_all_release.bat/.ps1` - полная релизная сборка

### **Запуск (run/)**
- `run_debug.bat/.ps1` - запуск отладочной версии
- `run_release.bat/.ps1` - запуск релизной версии
- `run_checkers.bat/.ps1` - запуск шашек

### **Тестирование (test/)**
- `run_tests.bat/.ps1` - запуск всех тестов
- `test_checkers.bat/.ps1` - тесты для шашек

### **Веб-версия (web/)**
- `build_web.bat/.ps1` - сборка веб-версии
- `build_web_release.bat/.ps1` - релизная сборка веб-версии
- `start_web.bat/.ps1` - запуск веб-сервера

### **Утилиты (utils/)**
- `clean.bat/.ps1` - очистка проекта
- `setup.bat/.ps1` - настройка окружения
