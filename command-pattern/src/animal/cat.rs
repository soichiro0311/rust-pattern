use crate::animal::*;

pub struct Cat {
    sound: String,
}

impl Cat {
    pub fn new() -> Cat {
        Cat {
            sound: "meow".to_string(),
        }
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &String {
        &self.sound
    }
}
