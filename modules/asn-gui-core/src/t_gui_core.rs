// use crate::{AsnGuiElement, AsnGuiMap};

pub trait AsnGuiCore {}

pub trait AsnGuiFabrica {
    // попробуем без трейтов, через задание типов в классах
    // fn get_map(
    //     &self,
    //     map_tiles_bytes: &[u8],
    //     map_width: u32,
    //     map_height: u32,
    // ) -> impl AsnGuiMap + AsnGuiElement;
}
