use crate::animal::*;

pub struct Breeder {
    pets: Vec<Box<Animal>>,
}
impl Breeder {
    pub fn new() -> Breeder {
        Breeder { pets: vec![] }
    }

    pub fn get(&mut self, animal: Box<Animal>) {
        self.pets.push(animal);
    }

    pub fn pets(&self) -> &Vec<Box<Animal>> {
        &self.pets
    }
}
