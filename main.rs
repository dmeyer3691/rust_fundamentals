#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::ErrorKind;
use std::io::{Error, Read};
use std::collections::{VecDeque};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn main() {

     // project();
     // demo();
     // options_demo();
     // memory_demo();
     // fn_demo();
     // closure_demo();
     // error_handle_demo();
     // error_propogate_demo();
     // collections();
     // generics();
     concurrency();
     
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

          struct Waypoint {
               name: String,
               latitude: f64,
               longitude: f64
          }

          struct Segment {
               start: Waypoint,
               end: Waypoint
          }

          impl Segment {
               fn new(start: Waypoint, end: Waypoint) -> Self {
                    Self {
                         start,
                         end
                    }
               }
               
               fn distance(&self) -> f32 {
                    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
                    let start_radians = self.start.latitude.to_radians();
                    let end_radians = self.end.latitude.to_radians();
          
                    let delta_latitude = (self.start.latitude - self.end.latitude).to_radians();
                    let delta_longitude = (self.start.longitude - self.end.longitude).to_radians();
          
                    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                         + start_radians.cos()
                         * end_radians.cos()
                         * f64::powi((delta_longitude/2.0).sin(), 2);
          
                    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

                    distance as f32
               }
          }

          struct Boeing {
               required_crew: u8,
               range: u16
          }

          struct Airbus {
               required_crew: u8,
               range: u16
          }

          trait Flight {
               fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16)-> bool;
          }

          impl Flight for Boeing {
               fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16)-> bool {
                    if (available_crew >= required_crew) && (range + 150 > distance) {
                         true
                    } else {
                         false
                    }
               }
          }

          impl Flight for Airbus {
               fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16)-> bool {
                    if (available_crew >= required_crew) && (range + 280 > distance) {
                         true
                    } else {
                         false
                    }
               }
          }

          let boeing = Boeing {
               required_crew: 4,
               range: 7370
          };

          let airbus = Airbus {
               required_crew: 7,
               range: 5280
          };

          let boeing_is_legal = boeing.is_legal(
               boeing.required_crew, 18,
               boeing.range, 2385
          );
     
          let airbus_is_legal = airbus.is_legal(
               airbus.required_crew, 3,
               airbus.range, 2200
          );

          println!("Is the Boeing flight legal? {}\nIs the Airbus flight legal? {}", boeing_is_legal, airbus_is_legal);
          

          let kcle = Waypoint {
               name: "KCLE".to_string(),
               latitude: 41.4075, 
               longitude: -81.851111
          };

          // fills in remaining values from previously defined struct instance
          let kslc = Waypoint {
               name: "KSLC".to_string(),
               ..kcle
          };

          let kcle_kslc = Segment::new(kcle, kslc);
          let kcle_kslc_distance = kcle_kslc.distance();
          println!("{:.1}", kcle_kslc_distance);


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

     fn concurrency() {
          let outer_scope = 412;
          // need to move "ownership" of outerscope variable to new thread with "move"
          let join_handle = thread::spawn( move || {
               outer_scope * 2
          });

          // effectivly blocks esecution until thread completes
          let result = join_handle.join();
          match result {
               Ok(value) => {
                    println!("{}", value);
               }
               Err(_) => {}
          }


          // Text Example
          fn sara_chat(jon_tx:Sender<&str>, sara_rx:Receiver<&str>){
               let result = sara_rx.recv();
               println!("{}", result.unwrap());

               let send_result = jon_tx.send("Hello Jon");
          }

          fn jon_chat(sara_tx:Sender<&str>, jon_rx:Receiver<&str>){
               let send_result: Result<(), mpsc::SendError<&str>> = sara_tx.send("Hello Sara");

               let result = jon_rx.recv();
               println!("{}", result.unwrap());
          }

          let (jon_tx, jon_rx) = mpsc::channel();
          let (sara_tx, sara_rx) = mpsc::channel();

          let jon_handle = thread::spawn(move || {
               jon_chat(sara_tx, jon_rx);
          });

          let sara_handle = thread::spawn(move || {
               sara_chat(jon_tx, sara_rx);
          });

          match jon_handle.join(){
               Ok(_) => {}
               Err(_) => {}
          }

          match sara_handle.join(){
               Ok(_) => {}
               Err(_) => {}
          }

     }

     fn generics() {

          #[derive(Debug)]
          struct NavAid<T, U> {
               name: String,
               frequency: T,
               data: U
          }

          let vor = NavAid {
               name: String::from("DQN"),
               frequency: 114.5,
               data: String::from("DQN is a VOR")
          };
          let ndb_data:Option<String> = Option::None;
          let ndb = NavAid {
               name: String::from("HKF"),
               frequency: 239,
               data: ndb_data
          };

          println!("VOR information is {:?}", vor);
          println!("NDB information is {:?}", ndb);


          let sum = add(256, 262);
          println!("{}", sum);

          // constraints
          fn add<T: Add<Output=T>>(operand1: T, operand2: T) -> T {
               operand1 + operand2
          }

     }


     fn collections() {
          let mut flights:Vec<&str> = Vec::new();

          let vec_macro = vec![1,2,3,4];

          flights.push("Flight 1");
          flights.push("Flight 2");
          flights.push("Flight 3");
          flights.push("Flight 4");
          flights.push("Flight 5");

          for flight in flights.iter() {
               println!("{}", flight);
          }

          if let Some(fourth) = flights.get(3){
               println!("\n{}\n", fourth);
          }

          flights.remove(2);
          flights.insert(2, "New Flight 3");

          for flight in flights.iter() {
               println!("{}", flight);
          }
          println!("\n");


          let length = flights.len();
          println!("Number of flights: {}", length);

          let three_exists = flights.contains(&"Flight 3");
          println!("Flight 3 exists: {}", three_exists);
          let two_exists = flights.contains(&"Flight 2");
          println!("Flight 2 exists: {}", two_exists);

          flights.clear();

          let length = flights.len();
          println!("Number of flights: {}", length);

          let exists = flights.contains(&"Flight 3");
          println!("Flight 3 exists: {}", exists);

          let mut flights2:VecDeque<&str> = VecDeque::new();
          flights2.push_front("Flight 2");
          flights2.push_back("Flight 3");
          flights2.push_back("Flight 4");
          flights2.push_front("Flight 1");
          flights2.push_back("Flight 5");

          println!("\n");

          for flight in flights2.iter() {
               println!("{}", flight);
          }

          // Maps
          let mut flights3 = HashMap::new();

          flights3.insert("DA918", ("11:12","Orlando"));
          flights3.insert("DA428", ("12:05","Salt Lake City"));
          flights3.insert("DA98", ("9:43","London"));
          flights3.insert("DA113", ("6:20","Boston"));
          flights3.insert("DA41", ("15:30","Berlin"));
          flights3.insert("DA2815", ("17:11","Nashville"));

          let flight_number = "DA2815";
          let option = flights3.get(flight_number);
          let (time, destination) = option.unwrap();
          println!("\nFlight {} departs from {} at {}", flight_number, destination, time);

          if !flights3.contains_key(flight_number){
               flights3.insert(flight_number, ("12:00", "Puerto Rico"));
          } else {
               println!("\nFlight {} is already entered\n", flight_number);
          }

          // Sets
          let mut flights4 = HashSet::new();

          flights4.insert(("DA918","11:12","Orlando"));
          flights4.insert(("DA428","12:05","Salt Lake City"));
          flights4.insert(("DA98","9:43","London"));
          flights4.insert(("DA113","6:20","Boston"));
          flights4.insert(("DA41","15:30","Berlin"));
          flights4.insert(("DA2815","17:11","Nashville"));

          for flight in flights4.iter() {
               println!("{:?}", flight);
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