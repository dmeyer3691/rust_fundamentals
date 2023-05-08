#![allow(unused_variables)]
#![allow(dead_code)]

mod geo;
use geo::calculations::distance as geo_distance;

mod demo;
mod options;
mod memory;
mod functions;
mod closures;
mod errors;
mod collections;
mod generics;
mod concurrency;
mod random;

fn main() {

     // project();
     // demo::demo();
     // options::options_demo();
     // memory::memory_demo();
     // functions::fn_demo();
     // closures::closure_demo();
     // presence of test_file_dne.json changes behavior of next two functions
     // errors::error_handle_demo();
     // errors::error_propogate_demo();
     // collections::collections_demo();
     // generics::generics_demo();
     // concurrency::concurrency_demo();
     random::random_demo();
     
}


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
                    // let distance = haversin_distance(previous_waypoint_value.1, previous_waypoint_value.2, waypoint.1, waypoint.2);
                    let distance = geo_distance(previous_waypoint_value.1, previous_waypoint_value.2, waypoint.1, waypoint.2);
                    total_distance += distance;
                    previous_waypoint = Option::from(waypoint.clone());

                    println!("The distance between {} and {} is {:.1} kilometers", previous_waypoint_value.0, waypoint.0, distance);
               }
          }
     }
     
}


// mod geo {
//      const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

//      pub fn distance(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
     
//           let radians1 = lat1.to_radians();
//           let radians2 = lat2.to_radians();

//           let delta_latitude = (lat1 - lat2).to_radians();
//           let delta_longitude = (long1 - long2).to_radians();

//           let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
//                + radians1.cos()
//                * radians2.cos()
//                * f64::powi((delta_longitude/2.0).sin(), 2);

//           let central_angle = 2.0 * inner_central_angle.sqrt().asin();
//           let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

//           distance
//      }
// }