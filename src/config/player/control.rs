#[derive(Clone, Copy)]
pub struct ControlConfig {
    pub base_walk_speed: f32,
    pub dash_speed_rate: f32,
    pub dash_time: f32,
    pub dash_cooltime: f32,
    pub run_speed_rate: f32,
    pub ground_threshold: f32,
    pub jump_init_vel: f32,
}
impl Default for ControlConfig {
    fn default() -> Self {
        Self {
            base_walk_speed: 50.0,
            dash_speed_rate: 10.0,
            dash_time: 0.2,
            dash_cooltime: 2.0,
            run_speed_rate: 1.5,
            ground_threshold: 0.7,
            jump_init_vel: 200.0,
        }
    }
}
