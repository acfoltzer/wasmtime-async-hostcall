use std::error::Error;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::default();
    config.debug_info(true);
    let engine = Engine::new(&config);
    let store = Store::new(&engine);
    let mut linker = Linker::new(&store);

    linker.func(
        "my_module",
        "print_async",
        |ctx: Caller, msg_ptr: i32, msg_len: i32| {
            eprintln!("msg_ptr={:p}, msg_len={}", msg_ptr as *const u8, msg_len);
            let memory = ctx.get_export("memory").unwrap().into_memory().unwrap();
            let heap = unsafe { memory.data_unchecked() };
            let msg_bytes = &heap[msg_ptr as usize..msg_ptr as usize + msg_len as usize];
            let msg = String::from(std::str::from_utf8(msg_bytes).unwrap());
            eprintln!("msg={}", msg);
            futures::executor::block_on(async move {
                println!("{}", msg);
            });
            120
        },
    )?;

    let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
    wasi.add_to_linker(&mut linker)?;

    let module = Module::from_file(store.engine(), "guest/target/wasm32-wasi/debug/guest.wasm")?;
    linker.module("main", &module)?;

    linker.get_default("main")?.get0::<()>()?()?;

    Ok(())
}
