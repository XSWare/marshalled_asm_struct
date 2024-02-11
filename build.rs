fn main() {
    nasm_rs::compile_library_args("libutils.a", &["asm/utils.asm"], &[]).unwrap();
}
