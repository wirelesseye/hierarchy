# Hierarchy (Experimental)
Add OOP class hierarchy to Rust.
## Example

```rust
use hierarchy::class;

class!(ClassA {
    let one: i32;

    pub final fn new(one: i32) -> ClassA {
        ClassA { one }
    }
    
    pub fn print_name(&self) -> () {
        println!("ClassA")
    }
});

class!(ClassB extends ClassA {
    let two: i32;

    pub final fn new(one: i32, two: i32) -> ClassB {
        ClassB { class_a: ClassA::new(one), two }
    }
});

class!(ClassC extends ClassB < ClassA {
    let three: i32;

    pub final fn new(one: i32, two: i32, three: i32) -> ClassC {
        ClassC { class_b: ClassB::new(one, two), three: three }
    }
});

class!(ClassD extends ClassC < ClassB < ClassA {
    let four: i32;

    override ClassC {
        pub fn get_three(&self) -> &i32 {
            &42
        }
    }

    override ClassA {
        pub fn print_name(&self) -> () {
            println!("Class D");
        }
    }

    pub final fn new(one: i32, two: i32, three: i32, four: i32) -> ClassD {
        ClassD { class_c: ClassC::new(one, two, three), four }
    }
});

fn main() {
    let d = ClassD::new(1, 2, 3, 4);
    println!("{}", d.get_one()); // 1
    println!("{}", d.get_three()); // 42
    d.print_name(); // Class D
}
```