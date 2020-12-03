# Various hypervisor tests

### Building

Be sure to enable `crt-static`

```
set RUSTFLAGS=-Ctarget-feature=+crt-static
```

### Typical workflow

Each test breaks with `int3` to catch in the debugger. At this point, the snapshot can be
taken in VirtualBox. Single stepping after the `int3` should find a `vmcall`, which will
cause the VM to break out of the hypervisor. 

Each test has a goal which ends with a `vmcall`, so the hypervisor must also catch the 
`vmcall` VmExit and check `eax`. A success will have `0xdeadbeefcafebabe` while a failure
will have `0x0`

## restore_memory

Calls `GetCursor()` to trigger a syscall and then checks for static buffer for a written
value.
