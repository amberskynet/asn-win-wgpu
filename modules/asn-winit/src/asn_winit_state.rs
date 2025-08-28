use crate::WinitRenderManager;

pub struct AsnWinitState<R>
where
    R: WinitRenderManager,
{
    pub r: R,
}
