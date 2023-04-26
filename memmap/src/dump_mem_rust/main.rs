extern crate memmap;
use memmap::Mmap;
use std::fs::File;
use std::error::Error;

fn map_physical_mem(addr: usize, len: usize) -> Result<Mmap, Box<Error>> {
    let m = unsafe {
        memmap::MmapOptions::new()
            .offset(addr as u64)
            .len(len)
            .map(&File::open("/dev/mem")?)?
    };
    Ok(m)
}

// fn dump_mem_u8(addr: usize, len: usize) {
//     let m = match map_physical_mem(addr, len) {
//         Ok(m) => m,
//         Err(err) => {
//             panic!("Failed to mmap: Err={:?}", err);
//         }
//     };
//     (0..len).for_each(|x| println!("{:016x}: {:02x}", addr + x, m[x]));
// }

fn dump_mem<T>(addr: usize, len: usize)
where
    T: std::fmt::LowerHex,
{
    let sz = std::mem::size_of::<T>();
    let m = match map_physical_mem(addr, len * sz) {
        Ok(m) => m,
        Err(err) => {
            panic!("Failed to mmap: Err={:?}", err);
        }
    };
    let p = m.as_ptr() as *const T;
    (0..len).for_each(|x| unsafe {
        println!(
            "{:016x}: {:02$x}",
            addr + sz * x,
            std::ptr::read_volatile(p.offset(x as isize)),
            sz * 2
        );
    });
}

fn parse_hex(s: &String) -> Result<usize, Box<Error>> {
    let s = if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else {
        s
    };
    Ok(usize::from_str_radix(s, 16)?)
}

fn print_usage(name: &str) {
    eprintln!("Dump physical memory by specified size.");
    eprintln!("Usage: {} size address [len]", name);
    eprintln!("  where size={{1,2,4,8}}, address and len in hexadecimal.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        print_usage(&args[0]);
        return;
    }
    let addr = parse_hex(&args[2]).expect("address parse error");
    let len: usize = if args.len() >= 4 {
        parse_hex(&args[3]).unwrap_or(1)
    } else {
        1
    };
    match args[1].parse() {
        Ok(1) => dump_mem::<u8>(addr, len),
        Ok(2) => dump_mem::<u16>(addr, len),
        Ok(4) => dump_mem::<u32>(addr, len),
        Ok(8) => dump_mem::<u64>(addr, len),
        _ => print_usage(&args[0]),
    }
}
