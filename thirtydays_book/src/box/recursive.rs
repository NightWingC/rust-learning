struct  List {
    head: Option<Box<ListItem>>
}

struct ListItem {
    value: i32,
    next: Option<Box<ListItem>>
}

impl List {
    fn new() -> List {
       List { head: None } 
    }

    fn push (&mut self, value: i32){
        let new_item = Box::new(ListItem {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_item);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|boxed_item| {
            self.head = boxed_item.next;
            boxed_item.value
        })
    }
}



fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("Popped: {:?}", list.pop());
    println!("Popped: {:?}", list.pop());
    println!("Popped: {:?}", list.pop());
    println!("Popped: {:?}", list.pop());

    let list_recursive = List {
        head: Some(Box::new(ListItem { 
            value: 10, 
            next: Some(Box::new(ListItem { 
                value: 20, next: None, 
            })) 
        }))
    };

    if let Some(first_node) = &list_recursive.head {
        println!("First node value: {}", first_node.value);
        if let Some(second_node) = &first_node.next {
            println!("Second node value: {}", second_node.value);
        }
    } 
}