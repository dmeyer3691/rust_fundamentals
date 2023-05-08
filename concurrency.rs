use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};


pub fn concurrency_demo() {
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
