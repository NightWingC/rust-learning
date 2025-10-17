trait Dibujante {
    fn dibujar(&self);
}

struct Circulo {
    radio: f64,
}

impl Dibujante for Circulo {
    fn dibujar(&self) {
        println!("dibujando un circulo de radio {}", self.radio);
    }
}

struct Cuadrado {
    lado: f64,
}

impl  Dibujante for Cuadrado {
    fn dibujar(&self) {
            println!("dibujando un cuadrado de radio {}", self.lado);
    }
}

fn dibujar_estatico<T: Dibujante>(forma: &T){
    forma.dibujar();
}

fn dibujar_dinamico(forma: &dyn Dibujante){
    forma.dibujar();
}

fn main(){
    let c = Circulo { radio: 10.0 };
    let s = Cuadrado { lado: 4.0 };

    c.dibujar();
    s.dibujar();

    dibujar_estatico(&c);
    dibujar_estatico(&s);


    // Heap de memoria
    let mut formas: Vec<Box<dyn Dibujante>> = Vec::new();

    formas.push(Box::new(Circulo { radio: 5.0 }));
    formas.push(Box::new(Cuadrado { lado: 6.0 }));

    for forma in formas.iter(){
        dibujar_dinamico(forma.as_ref());
    } 

}