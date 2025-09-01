# ASN Web WGPU

Web версия ASN WGPU приложения, использующая WebAssembly.

## Быстрый старт

### Сборка

```bash
# Из корневой директории проекта
./build-web.sh
```

### Запуск

```bash
cd web
python3 -m http.server 8080
# или
npx serve .
```

Затем откройте http://localhost:8080 в браузере.

## Структура

- `index.html` - Основная HTML страница
- `pkg/` - Собранные WASM файлы (создается после сборки)
- `README.md` - Этот файл

## Возможности

- ✅ WebAssembly модуль
- ✅ WebGL рендеринг через WGPU
- ✅ Асинхронная инициализация
- ✅ Современный UI
- ✅ Обработка ошибок

## Требования

- Современный браузер с поддержкой WebAssembly
- WebGL 2.0
- HTTP сервер для локальной разработки

## Отладка

Откройте Developer Tools в браузере для просмотра логов и ошибок.

## Сборка вручную

```bash
# Установка wasm-pack (если не установлен)
cargo install wasm-pack

# Сборка
wasm-pack build --target web --out-dir web/pkg
```
