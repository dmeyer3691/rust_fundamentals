#[allow(dead_code)]
pub fn demo() {
     let unused_variable: &str = "Hello, World!";

     let person_name_slice: &str = "Donald Mallard";
     let person_name_string: String = person_name_slice.to_string();

     let duck = "Duck";
     let airlines = "Airlines";
     let airline_name = format!("{} {}", duck, airlines);

     let mut slogan = String::new();
     slogan.push_str("We hit the ground");
     slogan.push(' ');
     slogan = slogan + "every time";
     println!("{}", slogan);

     let location: (&str, f32, f32) = ("KCLE", 41.4094069, -81.8546911);
     // let (name, latitude, longitude) = location;
     println!("Location name: {}, latitude: {}, longitude: {}",
     // name, latitude, longitude);
     location.0, location.1, location.2);

     let scope_test = "outer_scope";
     println!("{}", scope_test);
     {
          let scope_test = "inner_scope";
          println!("{}", scope_test);
     }
     println!("{}", scope_test)
}