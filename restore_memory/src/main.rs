#![feature(asm)]

#[link(name="User32")]
extern "system" {
    fn GetCursor() -> u64;
}

fn main() {

    let mut buffer = [0; 100];

    // Call the function to load User32
    unsafe { 
        let cursor = GetCursor();
        print!("Cursor: {:#x}\n", cursor);
    }

    print!("Buffer addr: {:p}\n", &buffer);

    unsafe {
        // Breakpoint to catch in the debugger just before the `vmcall`
        asm!("int3");
        asm!("vmcall");

        let cursor = GetCursor();

        if buffer[..4] == ['T' as u8, 'E' as u8, 'S' as u8, 'T' as u8] {
            asm!("mov rax, 0xdeadbeefcafebabe ; vmcall");
        } else {
            asm!("mov rax, 0x0 ; vmcall");
        }

    }
}
