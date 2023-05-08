use std::ops::Add;

pub fn generics_demo() {

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