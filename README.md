# rust-tokio-dylib

This is a test playground for Tokio in Rust dynamic libraries.  Currently it
demonstrates that if you pass the runtime explicitly to the library and
call it directly it can work fine in single-threaded mode, but not in
multi-threaded.

## What's going on

In `main/src/main.rs` we create a runtime, either multi- or single-threaded
depending on the RUNTIME environment variable.  We then use `libloading`
to load a dynamic library and call its `init` function.  Then we sleep for
a bit and shut down the runtime again.

In the dynamic library `lib/src/lib.rs` `init()` we spawn an async which logs
`+++ Inside dylib async +++`.  Note that we are calling the runtime we get
passed in directly, not relying on the thread-local variables.

There's lots of other logging to show the process, too.

## Single-threaded

Run in single-threaded mode with:

`$ RUNTIME=single cargo run`

Output:

```
Running in single mode
Running dylib init()
Inside dylib init()
Finished dylib init()
Returned from dylib init()
--- Inside main async 1 ---
+++ Inside dylib async +++
--- Inside main async 2 ---
Shutting down
Done!
```

Notice that all the asyncs happen after initialisation (when it goes to sleep),
and both the async's in main.rs (`---`) and the one in the library (`+++`)
are run.

## Multi-threaded

Run in multi-threaded mode with:

`$ cargo run`

Output:

```
Running in multi mode
--- Inside main async 1 ---
Running dylib init()
Inside dylib init()
Finished dylib init()
Returned from dylib init()
Shutting down
Done!
```

Notice that the first async in main.rs (`--- 1`) happens immediately (in a worker
thread), but neither the one in the library (`+++`) or the one in main after it
(`--- 2`) are run.

## Why?

I suspect that although we are explicitly passing and using the runtime, the
multi-threaded runtime uses some other static data which is not encompassed
by it, and the dylib has a different version.
