// Main process for a Tokio dynamic library test

use std::sync::Arc;
use tokio::runtime::Runtime;
use anyhow::Result;
use libloading::{Library, Symbol};

// Main process
// Note - not #[tokio::main], we build the runtime ourselves
fn main() -> Result<()> {
    let runtime: Arc<Runtime>;

    if std::env::var("RUNTIME").as_deref() == Ok("single") {
        println!("Running in single mode");
        runtime = Arc::new(tokio::runtime::Builder::new_current_thread()
                           .enable_all()
                           .build()
                           .unwrap());
    } else {
        println!("Running in multi mode");
        runtime = Arc::new(tokio::runtime::Builder::new_multi_thread()
                           .enable_all()
                           .build()
                           .unwrap());
    }

    // Try an async here
    runtime.spawn(async {
        println!("--- Inside main async 1 ---");
     });

    // Load the library
    let lib_file = "target/debug/librust_tokio_dylib_sample.so";
    let lib_library: Arc<Library>;
    let lib_init: Symbol<fn(Arc<Runtime>)>;
    unsafe {
        lib_library = Arc::new(Library::new(lib_file)?);
        lib_init = lib_library.get(b"init")?;
    }

    // Initialise the lib, passing the runtime
    println!("Running dylib init()");
    lib_init(runtime.clone());
    println!("Returned from dylib init()");

    // Try another async here
    runtime.spawn(async {
        println!("--- Inside main async 2 ---");
     });

    // Wait for asyncs to run
    runtime.block_on(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
     });

    // Shut down
    println!("Shutting down");
    Arc::try_unwrap(runtime)
        .expect("Multiple references to the runtime still exist")
        .shutdown_timeout(tokio::time::Duration::from_secs(1));

    println!("Done!");
    Ok(())
}

