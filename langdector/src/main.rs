use whatlang::{detect};

//creating a function
fn what_language( x: String) {
    let text = &x;
//using .unwrap() for exception handling
    let info = detect(text).unwrap();
//println! is one of the major macros used in rust
    println!("Language type {}", info.lang());
    println!("Language Script {}", info.script());
    println!("System confidence {}", info.confidence());
    println!("System reliability {}", info.is_reliable());

}

fn main() {
//creating string alice
    let alice = String::from(
        "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!",
    );
//calling the function
    what_language(alice);
}