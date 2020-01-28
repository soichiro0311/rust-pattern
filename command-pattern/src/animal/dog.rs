use crate::animal::*;

pub struct Dog {
    sound: String,
}

impl Dog {
    pub fn new() -> Dog {
        Dog {
            sound: "bow".to_string(),
        }
    }
}

impl Animal for Dog {
    fn make_sound(&self) -> &String {
        &self.sound
    }
}
