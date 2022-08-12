use std::io;

fn main() {
    println!("Pig Latin Translation device!");

    println!("enter words to have them translated");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let output1 = &input[1..];
    let output2 = &input[0..1];
    let output3 = "ay";
   println!("translation");
   println!("{}{}{}", output1, output2, output3);
}
