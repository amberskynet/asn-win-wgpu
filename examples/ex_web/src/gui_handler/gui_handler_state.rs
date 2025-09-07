use wgpu_map::WgpuMap;

pub struct GuiHandlerState {
    m: WgpuMap,
    map_width: u32,
    map_height: u32,
    tiles_width: u32,
    tiles_height: u32,
}
