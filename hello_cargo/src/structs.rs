// Structs y Enums
struct Usuario {
    nombre: String,
    email: String,
    nacimiento: i32,
    activo: bool,
}

impl Usuario {
    fn edad(&self) -> i32 {
        let actual = 2025;
        return actual - self.nacimiento;
    }
}
fn main(){
    let mut user = Usuario {
        nombre: "Julio".to_string(),
        email: String::from("algo@algo.com"),
        nacimiento: 2000,
        activo: true,
    };

    println!("Usuario: {}, Edad: {}", user.nombre, user.nacimiento);

    user.activo = false;

    // shorthan int
    let user1 = newUsuer("Pepe".to_string(), "algo@algo.mx".to_string());

    let user2 = Usuario {
        nombre: "Juan".to_string(),
        email: "otro@otro.com".to_string(),
        ..user1
    };

    // tuple structs
    struct Point(i32,i32,i32,);

    let pointA = Point(12,33,66);

    let edad  = 

}

fn newUsuer(name: String, email: String) -> Usuario {
    return Usuario {
        nombre: name,
        email: email,
        nacimiento: 1991,
        activo: true,

    }
}