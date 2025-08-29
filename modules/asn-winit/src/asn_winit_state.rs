use crate::WinitRenderManager;

/// Состояние приложения asn-winit
///
/// Содержит менеджер рендеринга и флаг инициализации.
pub struct AsnWinitState<R>
where
    R: WinitRenderManager,
{
    /// Менеджер рендеринга
    pub r: R,

    /// Флаг инициализации
    pub is_init: bool,
}
