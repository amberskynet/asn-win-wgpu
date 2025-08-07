use crate::AsnGuiElement;

pub trait AsnGuiMap: AsnGuiElement {
    fn update_map(&mut self, rgba: &[u8]);
}
