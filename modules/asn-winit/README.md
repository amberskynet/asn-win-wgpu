# asn-winit

Модуль для интеграции с библиотекой winit в проекте Amber Sky Network.

## Описание

Этот модуль предоставляет удобную обертку для работы с библиотекой winit, которая используется для создания окон и обработки событий в графических приложениях.

## Основные компоненты

- `WinitRenderManager` - трейт для менеджеров рендеринга, совместимых с winit
- `run` - функция для запуска приложения
- `RenderManagerState` - состояние менеджера рендеринга
- `AsnWinitState` - состояние приложения asn-winit

## Структура модуля

- `app_state.rs` - управление состоянием приложения и обработка событий
- `asn_winit_state.rs` - состояние приложения
- `data.rs` - константы модуля
- `error.rs` - обработка ошибок
- `keyboard_handler.rs` - обработка клавиатурных событий
- `winit_utils.rs` - утилиты для работы с winit

## Использование

```rust
use asn_winit::{run, WinitRenderManager};

// Реализуйте трейт WinitRenderManager для вашего менеджера рендеринга
struct MyRenderManager;

impl WinitRenderManager for MyRenderManager {
    // Реализация методов трейта
}

// Запустите приложение
run(MyRenderManager).unwrap();
```

## Обработка ошибок

Модуль использует пользовательский тип ошибок `AsnWinitError` для более точной обработки различных ситуаций:

- `EventLoopCreationError` - ошибка создания цикла событий
- `WindowCreationError` - ошибка создания окна
- `RendererInitializationError` - ошибка инициализации рендерера
- `WindowResizeError` - ошибка изменения размера окна
- `RenderError` - ошибка отрисовки

## Лицензия

MIT