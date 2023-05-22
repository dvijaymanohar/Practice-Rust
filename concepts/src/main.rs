// Define a trait named Greet with a single method greet. It represents the behavior of being able to greet someone.

// Define a trait named `Greet` with a single method `greet`
trait Greet {
    fn greet(&self) -> String;
}

// Implement the `Greet` trait for the `Person` struct
struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

// Implement the `Greet` trait for the `Robot` struct
struct Robot {
    id: u32,
}

impl Greet for Robot {
    fn greet(&self) -> String {
        format!("Beep boop! I am robot #{}", self.id)
    }
}

// A function that takes any value implementing the `Greet` trait
fn introduce(greeter: &dyn Greet) {
    println!("{}", greeter.greet());
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
    };

    let robot = Robot { id: 42 };

    introduce(&person);
    introduce(&robot);
}

// In this example,

// We then define two structs: Person and Robot. Both structs implement the Greet trait by providing an implementation for the greet method.

// Next, we define a function named introduce that takes a reference to any value implementing the Greet trait. Inside the function, we call the greet method on the input value and print the greeting.

// In the main function, we create an instance of Person and Robot. We pass these instances to the introduce function, which calls the greet method on each value and prints the respective greetings.

// This demonstrates how traits allow us to define common behavior across different types and use them interchangeably by leveraging dynamic dispatch through trait objects.
