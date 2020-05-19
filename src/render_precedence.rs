pub enum RenderPrecedence {
    Gui,
    EffectOverlay,
    Map,
    CameraView
}

pub fn to_numeric(render_recedence: RenderPrecedence) -> u32 {
    match render_recedence {
        Gui => 0,
        EffectOverlay => 1,
        Map => 2,
        CameraView => 3
    }
}
