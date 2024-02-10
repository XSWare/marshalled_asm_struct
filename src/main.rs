#[link(name = "struct")]
extern "C" {
    fn new() -> AsmStruct;
}

fn main() {
    let asm_struct = AsmStruct::new();
    println!("value is {}", asm_struct.val);
    asm_struct.say_hello();
}

#[repr(C)]
struct AsmStruct {
    val: i64,
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
