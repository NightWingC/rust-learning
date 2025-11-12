use std::cell::RefCell; 
struct Messenger { 
    message: RefCell<String>, 
} 
impl Messenger { 
    fn new(msg: String) -> Messenger { 
        Messenger { 
            message: RefCell::new(msg), 
        }
    }

    fn send(&self, new_msg: &str) { 
         
        let mut msg_ref = self.message.borrow_mut(); 
        *msg_ref = new_msg.to_string(); 
        println!("Message sent: {}", *msg_ref); 
    }

    fn print_message(&self) { 
        let msg_ref = self.message.borrow();

        println!("Current message: {}", *msg_ref);
    }
}

fn main() { 
    let messenger = Messenger::new("Initial message".to_string());

    messenger.print_message(); 
    messenger.send("Hello, Rustacean!"); 
    messenger.print_message(); 
}   

