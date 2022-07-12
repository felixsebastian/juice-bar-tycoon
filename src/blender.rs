pub struct Blender {
    pub power: f32,
    pub active: bool,
}

impl Blender {
    pub fn new(power: f32) -> Blender {
        Blender {
            power: power,
            active: false
        }
    }
}
