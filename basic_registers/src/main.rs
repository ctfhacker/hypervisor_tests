#![feature(asm)]

fn main() {

    unsafe {
        // Breakpoint to catch in the debugger just before the `vmcall`
        asm!("int3");
        asm!("vmcall");

        let rax: u64;
        let rbx: u64;
        let rcx: u64;
        let rdx: u64;
        asm!("mov {}, rax", out(reg) rax);
        asm!("mov {}, rbx", out(reg) rbx);
        asm!("mov {}, rcx", out(reg) rcx);
        asm!("mov {}, rdx", out(reg) rdx);

        if rax == 0xdead && rbx == 0xbeef && rcx == 0xcafe && rdx == 0xbabe {
            asm!("mov rax, 0xdeadbeefcafebabe ; vmcall");
        } else {
            asm!("mov rax, 0 ; vmcall");
        }

    }
}
