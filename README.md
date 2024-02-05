# Hierarchy (Experimental)
Add OOP class hierarchy to Rust.
## Example

```rust
// animal.rs
use hierarchy::class;

class!(pub Animal {
    let name: String;

    pub fn new(name: String) -> Animal {
        Animal {
            name
        }
    }

    pub fn make_sound(&self) {
        println!("No sound")
    }

    pub fn get_name(&self) -> String {
        // use `get_xxx_struct().xxx` to access a struct field from a public non-static method 
        self.get_animal_struct().name.clone()
    }
});

// dog.rs
use hierarchy::class;
use crate::animal::{Animal, AnimalTrait};

class!(pub Dog extends Animal {
    pub fn new(name: String) -> Dog {
        Dog { animal: Animal::new(name) }
    }

    override Animal {
        pub fn make_sound(&self) -> () {
            println!("Wolf! Wolf!")
        }
    }

    pub fn fetch(&self) -> () {
        println!("{} is fetching a ball.", self.get_name());
    }
});

// cat.rs
use hierarchy::class;
use crate::animal::{Animal, AnimalTrait};

class!(pub Cat extends Animal {
    pub fn new(name: String) -> Cat {
        Cat {
            animal: Animal::new(name)
        }
    }

    override Animal {
        pub fn make_sound(&self) -> () {
            println!("Meow!");
        }
    }

    pub fn scratch(&self) -> () {
        println!("{} is scratching.", self.get_name());
    }
});

// main.rs
fn main() {
    let my_animal = Animal::new("Generic Animal".to_string());
    let my_dog = Dog::new("Buddy".to_string());
    let my_cat = Cat::new("Whiskers".to_string());

    // Calling methods
    my_animal.make_sound();     // No sound
    my_dog.make_sound();        // Wolf! Wolf!
    my_dog.fetch();             // Buddy is fetching a ball.
    my_cat.make_sound();        // Meow!
    my_cat.scratch();           // Whiskers is scratching.

    // Demonstrating polymorphism
    let mut animals: Vec<Box<dyn AnimalTrait>> = Vec::new();
    animals.push(Box::new(Dog::new("Rex".to_string())));
    animals.push(Box::new(Cat::new("Mittens".to_string())));

    // Polymorphic calls
    for animal in animals {
        animal.make_sound();    // Output:
                                // Wolf! Wolf!
                                // Meow!
    }
}

```