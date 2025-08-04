//! Пример использования оптимизированной карты для быстрого обновления текстур

use asn_wgpu::{
    RgbaHandler,
    wgpu_components::wgpu_map::optimized_map::{OptimizedWgpuMap, UpdateRegion},
};
use std::time::Instant;

fn main() {
    println!("=== Демонстрация API оптимизированной карты ===");

    // Создаем тестовые данные
    let map_width = 512;
    let map_height = 512;
    let mut test_data = vec![0u8; (map_width * map_height * 4) as usize];

    // Демонстрируем работу с RgbaHandler
    let mut handler = RgbaHandler::new(map_width, map_height);
    handler.fill_random();

    println!("Создан RgbaHandler размером {}x{}", map_width, map_height);
    println!("Размер данных: {} байт", test_data.len());

    // Демонстрируем частичное обновление
    let region = UpdateRegion {
        x: 100,
        y: 100,
        width: 50,
        height: 50,
    };

    println!("Область для обновления: {:?}", region);
    println!("Размер области: {} пикселей", region.width * region.height);

    // Симуляция производительности
    println!("\n=== Симуляция производительности ===");

    // Тест 1: Обновление всей карты
    let start = Instant::now();
    for i in 0..100 {
        // Генерируем новые данные
        for j in 0..test_data.len() {
            test_data[j] = ((i + j) % 256) as u8;
        }
    }
    let duration = start.elapsed();
    println!("Генерация данных для 100 обновлений: {:?}", duration);
    println!("Среднее время на генерацию: {:?}", duration / 100);

    // Тест 2: Частичное обновление
    let start = Instant::now();
    for i in 0..100 {
        let region_data = vec![255u8; (region.width * region.height * 4) as usize];
        // Здесь была бы оптимизированная карта
    }
    let duration = start.elapsed();
    println!("Частичное обновление 100 раз: {:?}", duration);
    println!(
        "Среднее время на частичное обновление: {:?}",
        duration / 100
    );

    println!("\n=== Рекомендации по оптимизации ===");
    println!("1. Используйте частичное обновление для небольших изменений");
    println!("2. Группируйте обновления в batch операции");
    println!("3. Используйте двойную буферизацию для избежания артефактов");
    println!("4. Кэшируйте часто используемые области");
    println!("5. Используйте компрессию текстур для экономии памяти");

    println!("\n=== API оптимизированной карты ===");
    println!("OptimizedWgpuMap::new() - создание оптимизированной карты");
    println!("update_map() - обновление всей карты");
    println!("update_region() - частичное обновление области");
    println!("update_queue() - асинхронное обновление очереди");
    println!("draw() - отрисовка карты");
    println!("update_bind_group() - обновление bind group");
}

/// Пример структуры для игрового цикла
pub struct GameMapExample {
    frame_count: u32,
    last_update: Instant,
}

impl GameMapExample {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        // Обновляем каждые 60 кадров
        if self.frame_count % 60 == 0 {
            self.last_update = Instant::now();
            println!("Обновление карты на кадре {}", self.frame_count);
        }
    }

    pub fn get_frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn get_time_since_update(&self) -> std::time::Duration {
        self.last_update.elapsed()
    }
}
