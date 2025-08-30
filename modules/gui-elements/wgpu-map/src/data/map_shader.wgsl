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

// Структура для хранения информации о размере текстуры тайлов и карты
struct TilesInfo {
    tiles_width: f32,   // Количество тайлов по ширине
    tiles_height: f32,  // Количество тайлов по высоте
    map_width: f32,     // Ширина карты в тайлах
    map_height: f32,    // Высота карты в тайлах
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
    let tile_index_x = u32(tile_info.r);
    let tile_index_y = u32(tile_info.g);
    
    // Рассчитываем текстурные координаты для выборки из текстуры тайлов
    // Учитываем размер одного тайла в текстуре
    let tile_width = 1.0 / u_tiles_info.tiles_width;
    let tile_height = 1.0 / u_tiles_info.tiles_height;
    
    // Получаем размеры карты в тайлах
    let map_width = u_tiles_info.map_width;
    let map_height = u_tiles_info.map_height;
    
    // Рассчитываем суб-координаты внутри тайла на основе входных текстурных координат
    // in.tex_coords - координаты в текстуре карты (0.0 - 1.0)
    // fract(in.tex_coords.x * u_tiles_info.tiles_width) - дробная часть координаты X в тайлах
    // Это дает нам координату внутри конкретного тайла (0.0 - 1.0)
    let sub_u = fract(in.tex_coords.x * u_tiles_info.map_width) * tile_width;
    let sub_v = fract((1.0 - in.tex_coords.y) * u_tiles_info.map_height) * tile_height;
    
    // Рассчитываем окончательные текстурные координаты для выборки из текстуры тайлов
    // tile_index_x и tile_index_y - индексы тайла
    // Умножаем на размер тайла, чтобы получить смещение в текстуре тайлов
    let tile_uv = vec2<f32>(f32(tile_index_x) * tile_width + sub_u, f32(tile_index_y) * tile_height + sub_v);
    
    // return vec4<f32>(in.tex_coords.x, in.tex_coords.y, 0.0, 1.0);
    // return vec4<f32>(sub_u, sub_v, 0.0, 1.0);
    // Выбираем цвет тайла из текстуры тайлов
    return textureSample(t_tiles, s_tiles, tile_uv);
}

