use std::mem::MaybeUninit;

#[link(name = "struct")]
extern "C" {
    fn new(val_ptr: *mut i64) -> extern "C" fn();
}

fn main() {
    let asm_struct = AsmStruct::new();
    asm_struct.say_hello();
    println!("value is {}", asm_struct.val);
}

#[repr(C)]
struct AsmStruct {
    val: i64,
    function_ptr: MaybeUninit<extern "C" fn()>,
}

impl AsmStruct {
    pub fn new() -> Self {
        let mut asm_struct = Self {
            function_ptr: MaybeUninit::uninit(),
            val: 0,
        };

        unsafe {
            asm_struct.function_ptr.write(new(&mut asm_struct.val as *mut i64));
        }
        asm_struct
    }

    pub fn say_hello(&self) {
        unsafe {
            (self.function_ptr.assume_init())();
        }
    }
}
