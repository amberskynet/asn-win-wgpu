use asn_gui_core::AsnGuiElement;

use crate::state::RenderContext;

use super::WgpuMap;

impl AsnGuiElement for WgpuMap {
    type AsnRenderContext = RenderContext;

    fn update(&mut self, _ctx: &Self::AsnRenderContext) {
        todo!()
    }

    fn draw(&mut self, _ctx: &Self::AsnRenderContext) {
        todo!()
    }
}
