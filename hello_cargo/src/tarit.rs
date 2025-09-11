fn main(){
    // Traits
    let julio = Human;
    println!("{}",julio.say_hi());

    let pelusa = Cat;
    println!("{}", pelusa.say_hi());

}

struct Human;
struct Cat;

trait Talk {
    fn say_hi(&self) -> String;
    fn lenguage(&self) -> String {
        "No tengo idioma".to_string();
    };

}

impl Talk for Human {
    fn say_hi(&self) -> String {
       "Hola amigos".to_string(); 
    }
    fn lenguage(&self) -> String {
        "HAblo humano".to_string();
    }
}


impl Talk for Cat {
    fn say_hi(&self) -> String {
       "Miua".to_string(); 
    }
      fn lenguage(&self) -> String {
        "maullo".to_string();
    }
}