use asn_wgpu::WgpuGraphContext;
use wgpu_map::{MapParams, MapTilesParams, WgpuMap};

use crate::gui_handler::map_utils::generate_random_map;

pub struct GuiHandlerState {
    m: WgpuMap,
    map_width: u32,
    map_height: u32,
    tiles_width: u32,
    tiles_height: u32,
}

pub fn new_handler_state() -> GuiHandlerState {
    GuiHandlerState {
        m: todo!(),
        map_width: todo!(),
        map_height: todo!(),
        tiles_width: todo!(),
        tiles_height: todo!(),
    }
}

fn get_map(gcx: &WgpuGraphContext) -> WgpuMap {
    let map_tiles_bytes = include_bytes!("../../../tiles_64_95.png");
    let tiles_width = 64;
    let tiles_height = 95;

    let tiles_params = MapTilesParams {
        map_tiles_bytes,
        tiles_width,
        tiles_height,
    };

    let map_width = 32;
    let map_height = 32;
    let mut map = generate_random_map(map_width, map_height, map_width * map_height - 1);

    let map_params = MapParams {
        map_width,
        map_height,
        tile_indices: &map,
    };

    wgpu_map::get_map(gcx, &tiles_params, &map_params)
}
