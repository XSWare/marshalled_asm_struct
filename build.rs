fn main() {
    nasm_rs::compile_library("libadd.a", &["asm/add.asm"]).unwrap();
}
