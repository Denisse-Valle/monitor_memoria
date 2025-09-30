use systemstat::{System, Platform};

fn main() {
    let sys = System::new();

    match sys.memory() {
        Ok(mem) => {
            // systemstat devuelve los valores en Kilobytes, hay que convertir.
            let total_gb = mem.total.as_u64() as f64 / 1024.0 / 1024.0;
            let free_gb = mem.free.as_u64() as f64 / 1024.0 / 1024.0;

            println!("Memoria Total: {:.2} GB", total_gb);
            println!("Memoria Libre: {:.2} GB", free_gb);
        },
        Err(e) => eprintln!("Error al obtener info de la memoria: {}", e),
    }
}
