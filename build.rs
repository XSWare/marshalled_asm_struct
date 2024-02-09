fn main() {
    nasm_rs::compile_library_args("libstruct.a", &["asm/struct.asm"], &[]).unwrap();
}
