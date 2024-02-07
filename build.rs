fn main() {
    nasm_rs::compile_library_args("libadd.a", &["asm/add.asm"], &[]).unwrap();
}
