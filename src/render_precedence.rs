pub enum RenderPrecedence {
    Gui,
    EffectOverlay,
    Map,
    CameraView,
}

pub fn to_numeric(render_recedence: RenderPrecedence) -> u32 {
    match render_recedence {
        RenderPrecedence::Gui => 0,
        RenderPrecedence::EffectOverlay => 1,
        RenderPrecedence::Map => 2,
        RenderPrecedence::CameraView => 3,
    }
}
