mod animal;

#[cfg(test)]
mod tests {
    use crate::animal::breeder::*;
    use crate::animal::cat::*;
    use crate::animal::dog::*;
    use crate::animal::*;
    #[test]
    fn test_dog_make_sound() {
        let dog = Dog::new();
        assert_eq!("bow", dog.make_sound());
    }

    #[test]
    fn test_cat_make_sound() {
        let cat = Cat::new();
        assert_eq!("meow", cat.make_sound());
    }

    #[test]
    fn test_breeder_pets_make_sound() {
        let mut breeder = Breeder::new();
        breeder.get(Box::new(Dog::new()));
        breeder.get(Box::new(Cat::new()));

        let pets = breeder.pets();
        assert_eq!("bow", pets[0].make_sound());
        assert_eq!("meow", pets[1].make_sound());
    }
}
