// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

// Структура для хранения информации о размере текстуры тайлов
struct TilesInfo {
    tiles_width: f32,  // Количество тайлов по ширине
    tiles_height: f32, // Количество тайлов по высоте
}

@group(0) @binding(0)
var<uniform> u_tiles_info: TilesInfo; // Uniform-буфер с информацией о тайлах

@group(0) @binding(1)
var t_tiles: texture_2d<f32>; // Текстура с тайлами
@group(0) @binding(2)
var s_tiles: sampler; // Сэмплер для текстуры тайлов
@group(0) @binding(3)
var t_map: texture_2d<f32>; // Текстура карты с координатами тайлов
@group(0) @binding(4)
var s_map: sampler; // Сэмплер для текстуры карты

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Получаем координаты тайла из текстуры карты (красный и зеленый компоненты)
    let tile_info = textureSample(t_map, s_map, in.tex_coords);
    
    // Извлекаем индексы тайла из красного и зеленого компонентов
    // Значения нормализованы (0.0 - 1.0), преобразуем в индексы
    let tile_index_x = u32(tile_info.r * u_tiles_info.tiles_width);
    let tile_index_y = u32(tile_info.g * u_tiles_info.tiles_height);
    
    // Рассчитываем текстурные координаты для выборки из текстуры тайлов
    // Учитываем размер одного тайла в текстуре
    let tile_width = 1.0 / u_tiles_info.tiles_width;
    let tile_height = 1.0 / u_tiles_info.tiles_height;
    
    // Центрируем координаты в середине тайла для более точной выборки
    let tile_center_x = (f32(tile_index_x) + 0.5) * tile_width;
    let tile_center_y = (f32(tile_index_y) + 0.5) * tile_height;
    
    // Создаем вектор координат для выборки из текстуры тайлов
    let tile_uv = vec2<f32>(tile_center_x, tile_center_y);
    
    // Выбираем цвет тайла из текстуры тайлов
    return textureSample(t_tiles, s_tiles, tile_uv);
}
