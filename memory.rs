#[allow(dead_code)]
pub fn memory_demo() {
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