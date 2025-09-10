// snake_case
fn main(){
    mostrar_bienvenida();
    let nro = seleccion_numero(&8);
    println!("Tu numero es {}", nro);

    let nro2 = {
        10
    };

    println!("Tu numero es {}", nro2);

    saludar_con_nombre("Julio");
}

fn saludar_con_nombre(name: &str){
    println!("Hola {}", name);
}

fn seleccion_numero(nro: &i32) -> i32 {
    *nro + 4
}

fn mostrar_bienvenida(){
    println!("Bienvenidos a rust")
}