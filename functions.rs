pub fn fn_demo() {
    let mut original = String::from ("original value");
    println!("\nOuter scope original: \t\"{}\"", original);

    {
         print_original(&original);
         change_original(&mut original);
         println!("inner scope original): \t\"{}\"", original);
    }
   
}

fn print_original(original: &String) {
    println!("fn print_original: \t\"{}\"", original);
} 

fn change_original(original: &mut String) {
    let next= original;
    *next = String::from("next value");
    println!("fn change_original: \t\"{}\"", next);
}