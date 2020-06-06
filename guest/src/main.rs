#[link(wasm_import_module = "my_module")]
extern "C" {
    #[no_mangle]
    fn print_async(msg_ptr: *const u8, msg_len: usize) -> i32;
}

fn main() {
    let msg = "hello, wasmtime!";
    unsafe {
        assert_eq!(print_async(msg.as_ptr(), msg.len()), 120);
    }
}
