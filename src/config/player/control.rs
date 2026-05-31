#[derive(Clone, Copy)]
pub struct ControlConfig {
    pub base_walk_speed: f32,
    pub ground_threshold: f32,
}
impl Default for ControlConfig {
    fn default() -> Self {
        Self {
            base_walk_speed: 50.0,
            ground_threshold: 0.7,
        }
    }
}
