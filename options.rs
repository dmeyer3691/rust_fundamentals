#[allow(dead_code)]
pub fn options_demo() {
     let phrase = String::from("Duck Airlines");
     let letter = phrase.chars().nth(5);

     let value = match letter {
          Some(character) => character.to_string(),
          None => String::from("No value")
     };

     println!("{}", value);

     let animal = "My Duck";
     match animal {
          "Duck" => println!("Quack"),
          "Dog" => println!("Bark"),
          _ => println!("Unknown animal")
     };

}