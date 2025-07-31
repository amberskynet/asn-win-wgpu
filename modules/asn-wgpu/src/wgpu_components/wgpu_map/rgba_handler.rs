use crate::texture;
use wgpu::{Device, Queue};

/// Класс для инициализации и обработки RGBA-массивов
///
/// `RgbaHandler` предоставляет удобный интерфейс для работы с RGBA-данными изображений.
/// Он позволяет создавать, модифицировать и управлять RGBA-массивами, а также
/// создавать текстуры из этих данных для использования в WGPU.
///
/// # Примеры
///
/// ```rust
/// use asn_wgpu::RgbaHandler;
///
/// // Создание нового RGBA-обработчика
/// let mut handler = RgbaHandler::new(256, 256);
///
/// // Заполнение синим цветом
/// handler.fill(0, 0, 255, 255);
///
/// // Установка красного пикселя
/// handler.set_pixel(100, 100, 255, 0, 0, 255).unwrap();
///
/// // Создание градиента
/// handler.create_gradient((255, 0, 0, 255), (0, 0, 255, 255));
/// ```
pub struct RgbaHandler {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl RgbaHandler {
    /// Создает новый экземпляр RgbaHandler с указанными размерами
    ///
    /// # Аргументы
    ///
    /// * `width` - ширина изображения в пикселях
    /// * `height` - высота изображения в пикселях
    ///
    /// # Возвращает
    ///
    /// Новый экземпляр `RgbaHandler` с пустым RGBA-массивом (все пиксели черные с прозрачностью 0)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let handler = RgbaHandler::new(100, 200);
    /// assert_eq!(handler.dimensions(), (100, 200));
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        let data = vec![0u8; size];

        Self {
            width,
            height,
            data,
        }
    }

    /// Создает RgbaHandler из существующего RGBA-массива
    ///
    /// # Аргументы
    ///
    /// * `rgba` - массив байтов с RGBA-данными
    /// * `width` - ширина изображения в пикселях
    /// * `height` - высота изображения в пикселях
    ///
    /// # Возвращает
    ///
    /// `Result<Self, String>` - успешный результат или ошибка с описанием
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если размер массива не соответствует ожидаемому размеру
    /// (width * height * 4 байта)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let rgba_data = vec![255u8; 100 * 100 * 4]; // Белый квадрат
    /// let handler = RgbaHandler::from_rgba(&rgba_data, 100, 100).unwrap();
    /// ```
    pub fn from_rgba(rgba: &[u8], width: u32, height: u32) -> Result<Self, String> {
        let expected_size = (width * height * 4) as usize;

        if rgba.len() != expected_size {
            return Err(format!(
                "Неверный размер RGBA-массива. Ожидается {}, получено {}",
                expected_size,
                rgba.len()
            ));
        }

        Ok(Self {
            width,
            height,
            data: rgba.to_vec(),
        })
    }

    /// Создает текстуру из RGBA-данных
    ///
    /// # Аргументы
    ///
    /// * `device` - WGPU устройство
    /// * `queue` - WGPU очередь команд
    /// * `label` - метка для текстуры
    ///
    /// # Возвращает
    ///
    /// `Result<texture::Texture, String>` - созданная текстура или ошибка
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    /// use wgpu::Device;
    ///
    /// let mut handler = RgbaHandler::new(256, 256);
    /// handler.fill(255, 0, 0, 255); // Красный цвет
    ///
    /// // Создание текстуры (требует WGPU контекст)
    /// // let texture = handler.create_texture(&device, &queue, "my_texture").unwrap();
    /// ```
    pub fn create_texture(
        &self,
        device: &Device,
        queue: &Queue,
        label: &str,
    ) -> Result<texture::Texture, String> {
        texture::Texture::from_rgba(device, queue, &self.data, self.width, self.height, label)
            .map_err(|e| format!("Ошибка создания текстуры: {}", e))
    }

    /// Обновляет данные RGBA-массива
    ///
    /// # Аргументы
    ///
    /// * `new_data` - новый массив RGBA-данных
    ///
    /// # Возвращает
    ///
    /// `Result<(), String>` - успех или ошибка
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если размер нового массива не соответствует текущим размерам
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// let new_data = vec![255u8; 100 * 100 * 4];
    /// handler.update_data(&new_data).unwrap();
    /// ```
    pub fn update_data(&mut self, new_data: &[u8]) -> Result<(), String> {
        let expected_size = (self.width * self.height * 4) as usize;

        if new_data.len() != expected_size {
            return Err(format!(
                "Неверный размер данных. Ожидается {}, получено {}",
                expected_size,
                new_data.len()
            ));
        }

        self.data.copy_from_slice(new_data);
        Ok(())
    }

    /// Устанавливает пиксель по координатам (x, y)
    ///
    /// # Аргументы
    ///
    /// * `x` - координата X (0 <= x < width)
    /// * `y` - координата Y (0 <= y < height)
    /// * `r` - красный компонент (0-255)
    /// * `g` - зеленый компонент (0-255)
    /// * `b` - синий компонент (0-255)
    /// * `a` - альфа-компонент (0-255)
    ///
    /// # Возвращает
    ///
    /// `Result<(), String>` - успех или ошибка
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если координаты выходят за границы изображения
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.set_pixel(50, 50, 255, 0, 0, 255).unwrap(); // Красный пиксель
    /// ```
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Координаты ({}, {}) выходят за границы изображения {}x{}",
                x, y, self.width, self.height
            ));
        }

        let index = ((y * self.width + x) * 4) as usize;
        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
        self.data[index + 3] = a;

        Ok(())
    }

    /// Получает пиксель по координатам (x, y)
    ///
    /// # Аргументы
    ///
    /// * `x` - координата X (0 <= x < width)
    /// * `y` - координата Y (0 <= y < height)
    ///
    /// # Возвращает
    ///
    /// `Result<(u8, u8, u8, u8), String>` - RGBA значения или ошибка
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если координаты выходят за границы изображения
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.set_pixel(50, 50, 255, 128, 64, 255).unwrap();
    /// let pixel = handler.get_pixel(50, 50).unwrap();
    /// assert_eq!(pixel, (255, 128, 64, 255));
    /// ```
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<(u8, u8, u8, u8), String> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Координаты ({}, {}) выходят за границы изображения {}x{}",
                x, y, self.width, self.height
            ));
        }

        let index = ((y * self.width + x) * 4) as usize;
        Ok((
            self.data[index],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        ))
    }

    /// Заполняет всю область указанным цветом
    ///
    /// # Аргументы
    ///
    /// * `r` - красный компонент (0-255)
    /// * `g` - зеленый компонент (0-255)
    /// * `b` - синий компонент (0-255)
    /// * `a` - альфа-компонент (0-255)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.fill(0, 0, 255, 255); // Синий цвет
    /// ```
    pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for i in (0..self.data.len()).step_by(4) {
            self.data[i] = r;
            self.data[i + 1] = g;
            self.data[i + 2] = b;
            self.data[i + 3] = a;
        }
    }

    /// Создает градиент от одного цвета к другому
    ///
    /// Градиент создается по вертикали (сверху вниз).
    ///
    /// # Аргументы
    ///
    /// * `start_color` - начальный цвет (R, G, B, A)
    /// * `end_color` - конечный цвет (R, G, B, A)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.create_gradient((255, 0, 0, 255), (0, 0, 255, 255)); // От красного к синему
    /// ```
    pub fn create_gradient(&mut self, start_color: (u8, u8, u8, u8), end_color: (u8, u8, u8, u8)) {
        for y in 0..self.height {
            let t = y as f32 / (self.height - 1) as f32;

            let r = ((1.0 - t) * start_color.0 as f32 + t * end_color.0 as f32) as u8;
            let g = ((1.0 - t) * start_color.1 as f32 + t * end_color.1 as f32) as u8;
            let b = ((1.0 - t) * start_color.2 as f32 + t * end_color.2 as f32) as u8;
            let a = ((1.0 - t) * start_color.3 as f32 + t * end_color.3 as f32) as u8;

            for x in 0..self.width {
                self.set_pixel(x, y, r, g, b, a).unwrap();
            }
        }
    }

    /// Получает размеры изображения
    ///
    /// # Возвращает
    ///
    /// Кортеж (width, height) с размерами изображения
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let handler = RgbaHandler::new(100, 200);
    /// assert_eq!(handler.dimensions(), (100, 200));
    /// ```
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Получает ссылку на данные
    ///
    /// # Возвращает
    ///
    /// Ссылка на массив байтов с RGBA-данными
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let handler = RgbaHandler::new(100, 100);
    /// let data = handler.data();
    /// assert_eq!(data.len(), 100 * 100 * 4);
    /// ```
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Получает мутабельную ссылку на данные
    ///
    /// # Возвращает
    ///
    /// Мутабельная ссылка на массив байтов с RGBA-данными
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// let data = handler.data_mut();
    /// data[0] = 255; // Установка красного компонента первого пикселя
    /// ```
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Изменяет размер изображения
    ///
    /// # Аргументы
    ///
    /// * `new_width` - новая ширина
    /// * `new_height` - новая высота
    ///
    /// # Примечание
    ///
    /// При изменении размера все данные сбрасываются в нули
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.resize(200, 200);
    /// assert_eq!(handler.dimensions(), (200, 200));
    /// ```
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
        let new_size = (new_width * new_height * 4) as usize;
        self.data.resize(new_size, 0);
    }

    /// Создает копию текущего состояния
    ///
    /// # Возвращает
    ///
    /// Новый экземпляр `RgbaHandler` с копией всех данных
    ///
    /// # Пример
    ///
    /// ```rust
    /// use asn_wgpu::RgbaHandler;
    ///
    /// let mut handler = RgbaHandler::new(100, 100);
    /// handler.fill(255, 0, 0, 255);
    /// let copy = handler.clone();
    /// assert_eq!(handler.dimensions(), copy.dimensions());
    /// ```
    pub fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
        }
    }
}

impl Default for RgbaHandler {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let handler = RgbaHandler::new(100, 200);
        assert_eq!(handler.dimensions(), (100, 200));
        assert_eq!(handler.data().len(), 100 * 200 * 4);
    }

    #[test]
    fn test_from_rgba() {
        let rgba = vec![255u8; 100 * 200 * 4];
        let handler = RgbaHandler::from_rgba(&rgba, 100, 200).unwrap();
        assert_eq!(handler.dimensions(), (100, 200));
    }

    #[test]
    fn test_set_get_pixel() {
        let mut handler = RgbaHandler::new(10, 10);
        handler.set_pixel(5, 5, 255, 128, 64, 255).unwrap();
        let pixel = handler.get_pixel(5, 5).unwrap();
        assert_eq!(pixel, (255, 128, 64, 255));
    }

    #[test]
    fn test_fill() {
        let mut handler = RgbaHandler::new(10, 10);
        handler.fill(255, 0, 0, 255);

        for y in 0..10 {
            for x in 0..10 {
                let pixel = handler.get_pixel(x, y).unwrap();
                assert_eq!(pixel, (255, 0, 0, 255));
            }
        }
    }
}
