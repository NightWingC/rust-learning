struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    fn crecer(&mut self, delta: u32) {
        self.ancho += delta;
        self.alto += delta;
    }

    fn eliminar(self) {
        println!("La instancia fue elemininada");
    }

    fn nuevo(ancho: u32, alto: u32) -> Rectangulo {
        Rectangulo { ancho, alto }
    }
}
fn main(){
    let mut rect = Rectangulo::nuevo(20, 30);
    println!("Area {}", rect.area());
    rect.crecer(2);
    println!("Area despues de crecer: {}", rect.area());
    rect.eliminar();
    // println!("Area {}", rect.area());
}