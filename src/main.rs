use systemstat::{System, Platform};

fn main() {
    let sys = System::new();
    match sys.memory() {
        Ok(mem) => {
            //  println!("Total: {} MiB", mem.total);
            // println!("Used: {} MiB", mem.used);
           
	    println!("Free: {} MiB", mem.free);
        }
        Err(e) => eprintln!("Error getting memory info: {}", e),
    }
}
