#![feature(asm)]

static mut BUFFER: [u8; 4] = [0; 4];

fn main() {
    unsafe {
        // Breakpoint to catch in the debugger just before the `vmcall`
        asm!("int3");
        asm!("vmcall");

        // Force the Out of Bounds Write!
        let buffer_addr = BUFFER.as_ptr() as usize;
        let buffer = &mut *(buffer_addr as *mut [u8; 0x4000]);
        buffer[0x3ff0] = 0x12;

        // Print to keep the optimizer happy and not optimize everything out
        print!("{:x?}\n", BUFFER);
    }
}
