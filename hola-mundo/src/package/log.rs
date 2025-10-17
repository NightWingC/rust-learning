use log::{ debug, info, warn, error };
use env_logger;
fn main() {
    env_logger::init();

    // "export RUST_LOG=info" ejecutar en la terminal
    // export RUST_LOG=debug ejecutar el debug
    debug!("Esto es un mensaje de DEBUG");
    info!("Un mensaje informativo");
    warn!("Mensaje de advertencia");
    error!("Mensaje de error");

}