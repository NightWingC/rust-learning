struct Punto {
    x: i32,
    y: i32,
}
fn main_one() {
  let mut punto: Punto = Punto { x:5, y: 10};
    mover_punto(&mut punto);
    println!("Punto original: ({},{})", punto.x, punto.y);
}

fn mover_punto(p: &mut Punto){
    p.x += 10;
    p.y += 6;
}


struct Usuario {
    name: String,
    mail: String,
    activo: bool
}

struct Grupo {
    nombre: String,
    usuarios: Vec<Usuario>,

}
fn main() {
    let usuario1 = Usuario {
        name: String::from("Alice"),
        mail: String::from("alice@gmil.com"),
        activo: true,
    };

    let usuario2 = Usuario {
        name: String::from("Bob"),
        mail: String::from("bob@gmil.com"),
        activo: false,
    };

    let mut grupo = Grupo {
        nombre: String::from("Administradores"),
        usuarios: vec![usuario1, usuario2],
    };

    imprimir_usuarios(&grupo);
    activar_usuarios(&mut grupo);
    imprimir_usuarios(&grupo);

}

fn imprimir_usuarios(grupo: &Grupo){
    println!("Grupo: {}", grupo.nombre);
    for usuario in &grupo.usuarios {
        println!("Usuario: {}, Activo: {}", usuario.name, usuario.activo);
    }
}

fn activar_usuarios(grupo: &mut Grupo){
    for usuario in &mut grupo .usuarios  {
        usuario.activo = true;
    }
}