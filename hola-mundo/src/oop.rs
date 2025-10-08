mod personaje {
    pub struct Personaje {
        pub nombre: String,
        nivel: u32,
    }

    impl Personaje {
        pub fn nuevo(nombre:String) -> Self {
            Self{ nombre, nivel: 1 }
        }

        pub fn subir_nivel(&mut self){
            self.nivel += 1;
        }

        pub fn ver_nivel(&self) -> u32 {
            self.nivel
        }
    }
}


fn oop(){
    let mut p = personaje::Personaje::nuevo("Alice".to_string());
    p.subir_nivel();
    println("Nivel: {}", p.ver_nivel());
}

// Herencia 
struct AnimalBase {
    edad: u32,
}

impl AnimalBase {
    fn cumplir_anios(&mut self){
        self.edad += 1;
    }

    fn obtener_edad(&self) -> u32 {
        self.edad
    }
}

// Composición
struct Perro {
    base: AnimalBase,
    nombre: String,
}

impl Perro {
    fn nuevo(nombre: &str) -> Self {
        Self { 
            base: AnimalBase { edad: 0 }, 
            nombre: nombre.to_string() 
        }
    }

    fn ladrar(&self){
        println!("Guaa: {}", self.nombre)
    }

    fn cumplir_anios(&mut self){
        self.base.cumplir_anios();
    }

    fn ver_edad(&self) -> u32 {
        self.base.obtener_edad()
    }
}

fn main(){
    let mut fido = Perro::nuevo("Fido");
    println!("Perro: {}", fido.nombre);
    println!("Edad: {}", fido.ver_edad());

    fido.ladrar();
    fido.cumplir_anios();
    println!("Edad tras cumplir años: {}", fido.ver_edad());

    fido.cumplir_anios();
    println!("Edad actual: {}", fido.ver_edad());

}


struct AnimalBase {
    edad: u32,
}

impl AnimalBase {
    fn cumplir_anios(&mut self){
        self.edad += 1;
    }

    fn obtener_edad(&self) -> u32 {
        self.edad
    }
}

// Composición
struct Perro {
    base: AnimalBase,
    nombre: String,
}

impl Perro {
    fn nuevo(nombre: &str) -> Self {
        Self { 
            base: AnimalBase { edad: 0 }, 
            nombre: nombre.to_string() 
        }
    }

    fn ladrar(&self){
        println!("Guaa: {}", self.nombre)
    }

    fn cumplir_anios(&mut self){
        self.base.cumplir_anios();
    }

    fn ver_edad(&self) -> u32 {
        self.base.obtener_edad()
    }
}

fn main(){
    let mut fido = Perro::nuevo("Fido");
    println!("Perro: {}", fido.nombre);
    println!("Edad: {}", fido.ver_edad());

    fido.ladrar();
    fido.cumplir_anios();
    println!("Edad tras cumplir años: {}", fido.ver_edad());

    fido.cumplir_anios();
    println!("Edad actual: {}", fido.ver_edad());

}