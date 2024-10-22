use clap::Parser;


#[derive(Parser, Debug)] 
struct Cli {
    #[arg(short, long)]
    get: String,
}

fn main() {
    let args = Cli::parse();
  
    match args.get.as_str() {
        "contact" => {
            let result = "My Contact info is (+91)8601426869";
            println!("Result: {}", result);
        },"address" => {
            let result = "My Address is marathalli Benagluru";
            println!("Result: {}", result);
        },"info" => {
           let result = "A competitive programing enthusiast.
           Interested in problem solving, data structure and algorithm.
           Interested in Development";
           println!("Result: {}", result);
        },"social" => {
            let res ="my linkedin url : https://www.linkedin.com/in/mridul-shukla-899123174/";
            println!("Result: {}", res);
        },"help" => {
            println!("Available commands:");
            println!("  contact   - Show contact information");
            println!("  address   - Show address");
            println!("  info      - Show personal information");
            println!("  social    - Show social media links");
            println!("  help      - Show this help message");    
        },
        _ => {
             eprintln!("Unsupported operation: try --get help command");
        }
    }
}
