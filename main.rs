#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::ErrorKind;
use std::io::{Error, Read};

fn main() {

     // project();
     // demo();
     // options_demo();
     // memory_demo();
     // fn_demo();
     // closure_demo();
     // error_handle_demo();
     error_propogate_demo();

     
     #[allow(dead_code)]
     fn project() {
          const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

          // concrete_example();

          fn concrete_example() {
               let kcle_latitude_degrees: f64 = 41.4075;
               let kcle_longitude_degrees: f64 = -81.851111;
               println!("Location name: {}, latitude: {}, longitude: {}",
                         "KCLE", kcle_latitude_degrees, kcle_longitude_degrees);

               let kslc_latitude_degrees: f64 = 40.7861;
               let kslc_longitude_degrees: f64 = -111.9822;
               println!("Location name: {}, latitude: {}, longitude: {}",
                         "KSLC", kslc_latitude_degrees, kslc_longitude_degrees);

               let kcle_latitude_radians = kcle_latitude_degrees.to_radians();
               let kslc_latitude_radians = kslc_latitude_degrees.to_radians();

               let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
               let delta_longitude = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

               let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + kcle_latitude_radians.cos()
                    * kslc_latitude_radians.cos()
                    * f64::powi((delta_longitude/2.0).sin(), 2);

               let central_angle = 2.0 * inner_central_angle.sqrt().asin();

               let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
               println!("The distance between the two points is {:.1} kilometers", distance);
          }
          

          fn haversin_distance(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
     
               let radians1 = lat1.to_radians();
               let radians2 = lat2.to_radians();
     
               let delta_latitude = (lat1 - lat2).to_radians();
               let delta_longitude = (long1 - long2).to_radians();
     
               let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + radians1.cos()
                    * radians2.cos()
                    * f64::powi((delta_longitude/2.0).sin(), 2);
     
               let central_angle = 2.0 * inner_central_angle.sqrt().asin();
               let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

               distance
          }

          let route = [
               ("KCLE", 41.4075, -81.851111),
               ("ONL", 42.47050, -98.68690),
               ("BFF", 41.89420, -103.48200),
               ("KSLC", 40.7861, -111.9822)
          ];

          let mut total_distance = 0.0;
          let mut previous_waypoint: Option<(&str, f64, f64)> = None;

          for waypoint in route.iter() {
               match previous_waypoint {
                    None => {
                         previous_waypoint = Option::from(waypoint.clone());
                         continue;
                    }
                    Some(previous_waypoint_value) => {
                         let distance = haversin_distance(previous_waypoint_value.1, previous_waypoint_value.2, waypoint.1, waypoint.2);
                         total_distance += distance;
                         previous_waypoint = Option::from(waypoint.clone());

                         println!("The distance between {} and {} is {:.1} kilometers", previous_waypoint_value.0, waypoint.0, distance);
                    }
               }
          }
          
     }
     


     #[allow(dead_code)]
     fn demo() {
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

     #[allow(dead_code)]
     fn options_demo() {
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

     #[allow(dead_code)]
     fn memory_demo() {
          let mut original = String::from ("original value");
          println!("\nOuter scope original: \t\"{}\"", original);

          // part 1
          // let next = original;
          // won't work because original doesn't exist anymore really. next is now owner of that memory in heap
          // println!("{}", original);

          // part 2
          // this makes next a pointer to original rather than borrowing value.
          // let next = &original;
          // original = String::from("new value");
          // rust won't let you do this since we've already pointed to it

          // part 3
          {
               let next = &mut original;

               *next = String::from("next value");
               println!("\ninner scope next: \t\"{}\"", next);
               println!("\ninner scope original: \t\"{}\"", original);

          }

          println!("\nOuter scope original: \t\"{}\"", original);


          // lifetime part 1
          let outer_scope;
          {
               let inner_scope = 5;
               outer_scope = inner_scope;
          }
          println!("{}", outer_scope);

          // lifetime part 2

          // wont work because value only exists in scope of function
          // and the returned pointer is now not pointing to anything
          // let returned_ref = return_bad_ref();
          // fn return_bad_ref() -> &i32 {
          //      let value = 4;
          //      &value
          // }
          
          let referenced_int = 6;
          let returned_value = return_one_param(&referenced_int);
          println!("{}", returned_value);

          fn return_one_param(value: &i32) -> &i32 {
               value
          }


          let value_one = 24;
          let value_two = 67;
          let value = explicit_lifetime(&value_one, &value_two);
          println!("{}", value);

          fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
               if p1 > p2 {
                    p1
               } else {
                    p2
               }
          }

     }


     fn fn_demo() {
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

     fn closure_demo() {
          let name = "Duck Airlines";

          let write_message_closure = |closure_scope_var: String| -> String {
               println!("This is the closure");
               println!("{}. {}", name, closure_scope_var);
               String::from(format!("{}. {}", name, closure_scope_var))
          };

          let closure_ret_val = write_message_closure(String::from("We hit the ground every time."));
     
          println!("{}", closure_ret_val);
     }


     fn error_demo() {
          let filename = "./test_file_dne.json";

          let file_handle = File::open(filename);

          match file_handle {
               Ok(file) => {
                    println!("{:#?}", file);
               }
               Err(error) => {
                    // panic!("panic message");
                    println!("{:#?}", error);
                    match error.kind() {
                         ErrorKind::NotFound => {
                              match File::create(filename) {
                                   Ok(file) => {
                                        println!("File Created");
                                   }
                                   Err(error) => {
                                        println!("{:#?}", error);
                                   }
                              }
                         }
                         // match any other error type
                         _ => {
                              println!("{:#?}", error);
                         }
                    }

               }
          }
     }


     fn error_propogate_demo() {

          let filename = "./test_file_dne.json";
          let file_data = read_file(filename);
          match file_data {
               Ok(data) => {
                    println!("{}", data);
               }
               Err(error) => {
                    println!("Error down in stack");
                    println!("{}", error);
               }
          }

          fn read_file(filename: &str) -> Result<String, Error> {
               // ? at the end is what does the magic here of passing error up stack
               let mut file_handle = File::open(filename)?;
               let mut file_data = String::new();
               file_handle.read_to_string(&mut file_data)?;
               Ok(file_data)
          }
     }
}