// Simple dynamic library to try out Tokio async
use std::sync::Arc;
use tokio::runtime::Runtime;

#[no_mangle]
pub fn init(runtime: Arc<Runtime>) {

    println!("Inside dylib init()");

    runtime.spawn(async {
        println!("+++ Inside dylib async +++");
     });

    println!("Finished dylib init()");
}

