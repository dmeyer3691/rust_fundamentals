#![allow(unused_variables)]


fn main() {

     project();
     // demo();
     // options_demo();
     
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

}