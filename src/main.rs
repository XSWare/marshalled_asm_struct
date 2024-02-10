#[link(name = "struct")]
extern "C" {
    fn new() -> AsmStruct;
}

fn main() {
    let asm_struct = AsmStruct::new();
    println!("value 1 is {}", asm_struct.val1);
    println!("value 2 is {}", asm_struct.val2);
    asm_struct.say_hello();
}

#[repr(C)]
struct AsmStruct {
    val1: i64,
    val2: i64,
    function_ptr: extern "C" fn(),
}

impl AsmStruct {
    pub extern "C" fn new() -> Self {
        unsafe { new() }
    }

    pub fn say_hello(&self) {
        (self.function_ptr)();
    }
}
