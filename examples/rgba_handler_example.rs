use asn_wgpu::wgpu_components::wgpu_map::RgbaHandler;

fn main() {
    println!("Пример использования RgbaHandler");

    // Создаем новый RGBA-обработчик с размерами 256x256
    let mut handler = RgbaHandler::new(256, 256);
    println!("Создан RGBA-обработчик размером {:?}", handler.dimensions());

    // Заполняем изображение синим цветом
    handler.fill(0, 0, 255, 255);
    println!("Изображение заполнено синим цветом");

    // Устанавливаем красный пиксель в центре
    handler.set_pixel(128, 128, 255, 0, 0, 255).unwrap();
    println!("Установлен красный пиксель в центре (128, 128)");

    // Получаем цвет пикселя в центре
    let pixel = handler.get_pixel(128, 128).unwrap();
    println!(
        "Цвет пикселя в центре: R={}, G={}, B={}, A={}",
        pixel.0, pixel.1, pixel.2, pixel.3
    );

    // Создаем градиент от красного к синему
    let mut gradient_handler = RgbaHandler::new(100, 100);
    gradient_handler.create_gradient((255, 0, 0, 255), (0, 0, 255, 255));
    println!(
        "Создан градиент от красного к синему размером {:?}",
        gradient_handler.dimensions()
    );

    // Создаем RGBA-обработчик из существующих данных
    let rgba_data = vec![255u8; 100 * 100 * 4]; // Белый квадрат 100x100
    let white_handler = RgbaHandler::from_rgba(&rgba_data, 100, 100).unwrap();
    println!(
        "Создан RGBA-обработчик из существующих данных размером {:?}",
        white_handler.dimensions()
    );

    // Проверяем размер данных
    println!("Размер данных: {} байт", handler.data().len());
    println!("Ожидаемый размер: {} байт", 256 * 256 * 4);

    println!("Все операции выполнены успешно!");
}
