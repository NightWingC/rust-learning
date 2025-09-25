
enum ConnectionState {
    Conneted,
    Disconnected,
    Connecting,
}


fn main() {
    describe_state(ConnectionState::Conneted);
    priint_number(Some(24));
    priint_number(None);

    let tuple = (5 ,19);


    match tuple {
        (x,y) if x == y => println!("Los dos valores son iguales"),
        (x,y) if x > y => println!("El primer valor es mayor {} > {} ", x, y),
        (x,y) => println!("El primer valor es menor {} < {} ", x, y),

    }

    println!("{}", increment_option(Some(5)));
    println!("{}", increment_option(None));


}

fn describe_state(state: ConnectionState){
    match state {
        ConnectionState::Conneted => println!("La conexion está establecida.."),
        ConnectionState::Disconnected => println!("No hay conexión"),
        ConnectionState::Connecting => println!("Intentando conectar"),
        
    }
}

fn priint_number(num: Option<i32>){
    match num {
        Some(value) => println!("El numero es: {value}"),
        None => println!("No se proporciono ningun valor"),
    }
}

fn increment_option(num: Option<i32>) -> i32 {
    let result = match num {
        Some(value) => value + 1,
        None => 0,
    };

    result
}
