use crate::AsnGuiFabrica;

pub trait AsnGuiHandler {
    fn init(&mut self, f: &impl AsnGuiFabrica);
    fn update(&mut self);
    fn draw(&mut self);
}
