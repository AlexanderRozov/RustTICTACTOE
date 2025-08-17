# 🚀 **Релизная сборка проекта "Шашки"**

Этот документ описывает процесс создания релизной версии проекта для распространения.

## 📋 **Что такое релизная сборка?**

Релизная сборка (`--release`) в Rust:
- ✅ **Оптимизирует код** для максимальной производительности
- ✅ **Убирает отладочную информацию** и символы
- ✅ **Создает компактные исполняемые файлы**
- ✅ **Улучшает время выполнения** в 2-10 раз
- ✅ **Готова для распространения** конечным пользователям

## 🛠️ **Требования**

- Rust (последняя стабильная версия)
- wasm-pack (для веб-версии)
- Windows 10/11 (для .bat скриптов)

## 📁 **Структура релизных файлов**

После успешной сборки:

```
RustTicTacToe/
├── target/
│   └── release/                    # ← Релизная сборка
│       ├── rust_tic_tac_toe.exe   # Основной исполняемый файл
│       └── rust_tic_tac_toe.pdb   # Отладочные символы (можно удалить)
├── pkg/                           # ← Веб-пакет
│   ├── rust_tic_tac_toe_bg.wasm  # Оптимизированный WASM
│   ├── rust_tic_tac_toe.js       # JavaScript обертка
│   └── rust_tic_tac_toe.d.ts     # TypeScript типы
└── Скрипты сборки
    ├── build_release.bat          # Десктопная сборка
    ├── build_web_release.bat      # Веб-сборка
    └── build_all_release.bat      # Полная сборка
```

## 🎯 **Варианты сборки**

### **1. Только десктопная версия**
```bash
# Batch
build_release.bat

# PowerShell
.\build_release.ps1

# Ручная сборка
cargo build --release
```

### **2. Только веб-версия**
```bash
# Batch
build_web_release.bat

# PowerShell
.\build_web_release.ps1

# Ручная сборка
cargo build --release --target wasm32-unknown-unknown
wasm-pack build --target web --release
```

### **3. Полная сборка (рекомендуется)**
```bash
# Batch
build_all_release.bat

# Ручная сборка
cargo clean
cargo build --release
cargo build --release --target wasm32-unknown-unknown
wasm-pack build --target web --release
cargo test --release
```

## ⚡ **Сравнение Debug vs Release**

| Характеристика | Debug | Release |
|----------------|-------|---------|
| **Размер файла** | ~15-20 MB | ~5-8 MB |
| **Производительность** | Базовая | 2-10x быстрее |
| **Отладочная информация** | Полная | Минимальная |
| **Время сборки** | Быстро | Медленнее (оптимизация) |
| **Готовность к распространению** | Нет | Да |

## 🔧 **Оптимизации релизной сборки**

### **Cargo.toml настройки**
```toml
[profile.release]
opt-level = 3          # Максимальная оптимизация
lto = true            # Link Time Optimization
codegen-units = 1     # Одна единица компиляции
panic = 'abort'       # Быстрое завершение при панике
strip = true          # Удаление символов (Linux/macOS)
```

### **WASM оптимизации**
```bash
# Дополнительная оптимизация WASM
wasm-opt -O4 -o pkg/rust_tic_tac_toe_bg.wasm pkg/rust_tic_tac_toe_bg.wasm
```

## 📊 **Ожидаемые результаты**

### **Десктопная версия**
- Размер: 5-8 MB (вместо 15-20 MB)
- Время запуска: <1 секунды
- Производительность: 2-10x быстрее

### **Веб-версия**
- WASM размер: 200-500 KB
- JavaScript: 50-100 KB
- Время загрузки: <2 секунды

## 🚀 **Распространение**

### **Для Windows пользователей**
1. Скопируйте `target\release\rust_tic_tac_toe.exe`
2. Создайте папку с игрой
3. Добавьте README с инструкциями
4. Упакуйте в ZIP архив

### **Для веб-сайта**
1. Скопируйте содержимое папки `pkg\`
2. Скопируйте `checkers.html`
3. Загрузите на веб-сервер
4. Убедитесь, что MIME-типы настроены для .wasm

## 🧪 **Тестирование релизной версии**

```bash
# Запуск тестов в релизном режиме
cargo test --release

# Запуск игры
target\release\rust_tic_tac_toe.exe --checkers

# Тестирование веб-версии
# Откройте checkers.html в браузере
```

## 🔍 **Устранение проблем**

### **Ошибка "wasm-pack не найден"**
```bash
cargo install wasm-pack
```

### **Ошибка WASM сборки**
```bash
rustup target add wasm32-unknown-unknown
```

### **Большой размер файлов**
- Убедитесь, что используется `--release`
- Проверьте настройки в `Cargo.toml`
- Удалите отладочные символы (.pdb файлы)

## 📈 **Мониторинг производительности**

### **Измерение времени выполнения**
```bash
# Windows
time target\release\rust_tic_tac_toe.exe --checkers

# PowerShell
Measure-Command { .\target\release\rust_tic_tac_toe.exe --checkers }
```

### **Анализ размера файлов**
```bash
# Windows
dir target\release\rust_tic_tac_toe.exe
dir pkg\*.wasm
dir pkg\*.js
```

## 🎉 **Готово к релизу!**

После успешной сборки у вас будет:
- ✅ Оптимизированная десктопная версия
- ✅ Компактная веб-версия
- ✅ Все тесты пройдены
- ✅ Файлы готовы к распространению

**Удачной игры в шашки!** 🎯
