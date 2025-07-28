use asn_wgpu::RgbaHandler;

fn main() {
    println!("Пример интеграции RgbaHandler с WgpuMap");

    // Создаем RGBA-обработчик для карты
    let mut map_handler = RgbaHandler::new(512, 512);
    println!(
        "Создан RGBA-обработчик для карты размером {:?}",
        map_handler.dimensions()
    );

    // Создаем градиентную карту
    map_handler.create_gradient((0, 0, 255, 255), (255, 255, 0, 255));
    println!("Создан градиент от синего к желтому");

    // Добавляем некоторые элементы на карту
    for x in 100..200 {
        for y in 100..200 {
            map_handler.set_pixel(x, y, 255, 0, 0, 255).unwrap(); // Красный квадрат
        }
    }
    println!("Добавлен красный квадрат в центр карты");

    // Создаем еще один RGBA-обработчик для текстуры
    let mut texture_handler = RgbaHandler::new(256, 256);
    texture_handler.fill(0, 255, 0, 255); // Зеленый фон

    // Добавляем узор на текстуру
    for x in 0..256 {
        for y in 0..256 {
            if (x + y) % 20 < 10 {
                texture_handler.set_pixel(x, y, 255, 255, 255, 255).unwrap(); // Белые полосы
            }
        }
    }
    println!(
        "Создана текстура с узором размером {:?}",
        texture_handler.dimensions()
    );

    // Демонстрируем работу с пикселями
    let pixel = map_handler.get_pixel(150, 150).unwrap();
    println!(
        "Цвет пикселя в центре красного квадрата: R={}, G={}, B={}, A={}",
        pixel.0, pixel.1, pixel.2, pixel.3
    );

    // Демонстрируем изменение размера
    let mut small_handler = RgbaHandler::new(100, 100);
    small_handler.fill(128, 128, 128, 255); // Серый цвет
    println!(
        "Создан маленький обработчик размером {:?}",
        small_handler.dimensions()
    );

    small_handler.resize(200, 200);
    println!("Размер изменен на {:?}", small_handler.dimensions());

    // Демонстрируем клонирование
    let original = RgbaHandler::new(50, 50);
    let clone = original.clone();
    println!(
        "Создана копия обработчика размером {:?}",
        clone.dimensions()
    );

    // Демонстрируем создание из существующих данных
    let rgba_data = vec![255u8; 64 * 64 * 4]; // Белый квадрат 64x64
    let white_handler = RgbaHandler::from_rgba(&rgba_data, 64, 64).unwrap();
    println!(
        "Создан обработчик из данных размером {:?}",
        white_handler.dimensions()
    );

    // Демонстрируем обновление данных
    let mut update_handler = RgbaHandler::new(32, 32);
    let new_data = vec![128u8; 32 * 32 * 4]; // Серый цвет
    update_handler.update_data(&new_data).unwrap();
    println!(
        "Данные обновлены для обработчика размером {:?}",
        update_handler.dimensions()
    );

    println!("\nВсе операции с RGBA-обработчиками выполнены успешно!");
    println!("Теперь можно использовать эти данные для создания текстур в WGPU.");

    // Примечание: для реального использования с WGPU потребуется инициализация устройства
    println!("\nПримечание: Для создания текстур требуется инициализированное WGPU устройство.");
    println!("Пример создания текстуры:");
    println!(
        "let texture = map_handler.create_texture(&device, &queue, \"map_texture\").unwrap();"
    );
}
