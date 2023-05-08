use std::collections::{VecDeque};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn collections_demo() {
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