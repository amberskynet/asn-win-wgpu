use crate::AsnGuiMap;

pub trait AsnGuiCore {}

pub trait AsnGuiFabrica {
    fn get_map(&self, map_tiles_bytes: &[u8], map_width: u32, map_height: u32) -> impl AsnGuiMap;
}
