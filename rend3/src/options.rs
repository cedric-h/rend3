use winit::dpi::PhysicalSize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VSyncMode {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RendererOptions {
    pub vsync: VSyncMode,
    pub size: PhysicalSize<u32>,
}
