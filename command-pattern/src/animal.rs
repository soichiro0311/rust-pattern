pub mod breeder;
pub mod cat;
pub mod dog;

pub trait Animal {
    fn make_sound(&self) -> &String;
}
