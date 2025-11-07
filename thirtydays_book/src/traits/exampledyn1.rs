trait  Draw {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with label: \"{}\"", self.label);
    }
}

fn draw_item(item: &dyn Draw){
    item.draw();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with {} options", self.options.len());
    }
}

fn get_options() -> impl Iterator<Item = String> { 
    vec![ 
        String::from("Apple"), 
        String::from("Banana"),
    ].into_iter();
}

fn get_component() -> Box<dyn Draw> {
    Box::new(Button { 
        width: 75, 
        height: 15, 
        label: String::from("Cancel"), 
    }) 
}


fn main() {
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("Click me"),
    };

    let select_box = SelectBox {
        width: 100,
        height: 30,
        options: vec![
            String::from("Yest"),
            String::from("No"),
            String::from("Maybe"),
        ]
    };

    draw_item(&button);
    draw_item(&select_box);

    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(button),
        Box::new(select_box),
    ];

    for component in components {
        component.draw();
    }

     for option in get_options() { 
        println!("{}", option); 
    } 

    let component = get_component(); 
    component.draw();

    
}