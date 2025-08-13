use crate::AsnGuiWindowConfig;

pub trait TAsnWindowManager {
    type AsnWindow;
    type AsnWindowContext;

    fn new_window(
        &mut self,
        ctx: &Self::AsnWindowContext,
        conf: &AsnGuiWindowConfig,
    ) -> Result<Self::AsnWindow, Box<dyn std::error::Error>>;
}
