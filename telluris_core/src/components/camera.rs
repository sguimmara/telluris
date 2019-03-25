use crate::Component;

#[derive(Debug)]
pub struct Camera {
    fov_degrees: f32,
}

impl Camera {
    pub fn fov(&self) -> f32 {
        self.fov_degrees
    }
}

impl Component for Camera {
    fn name(&self) -> &'static str {
        "camera"
    }
}
