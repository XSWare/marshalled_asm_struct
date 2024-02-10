use std::mem::MaybeUninit;

#[link(name = "struct")]
extern "C" {
    fn new(val_ptr: *mut AsmStruct);
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
    pub fn new() -> Self {
        let mut asm_struct_uninit: MaybeUninit<AsmStruct> = MaybeUninit::uninit();

        unsafe {
            new(asm_struct_uninit.as_mut_ptr());
            asm_struct_uninit.assume_init()
        }
    }

    pub fn say_hello(&self) {
        (self.function_ptr)();
    }
}
