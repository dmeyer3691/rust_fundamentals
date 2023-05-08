pub fn closure_demo() {
    let name = "Duck Airlines";

    let write_message_closure = |closure_scope_var: String| -> String {
         println!("This is the closure");
         println!("{}. {}", name, closure_scope_var);
         String::from(format!("{}. {}", name, closure_scope_var))
    };

    let closure_ret_val = write_message_closure(String::from("We hit the ground every time."));

    println!("{}", closure_ret_val);
}