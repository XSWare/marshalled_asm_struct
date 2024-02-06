#[link(name = "add")]
extern "C" {
    fn asm_add(a: u64, b: u64) -> u64;
}

fn main() {
    let sum = unsafe { asm_add(1, 2) };
    println!("{}", sum);
}
