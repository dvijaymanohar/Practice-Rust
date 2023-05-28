pub struct QpcAdac {
    mmap: MmapMut,
}

#[repr(C)]
struct Registers {
    information_register: Volatile<u32>,
    counter_low: Volatile<u32>,
    counter_high: Volatile<u32>,
    compare_0: Volatile<u32>,
    compare_1: Volatile<u32>,
    compare_2: Volatile<u32>,
    compare_3: Volatile<u32>,
}

impl QpcAdac {
    fn map_registers() -> MmapMut {
        // All peripherals can be described by an offset from the Peripheral Base Address, which starts at:
        // 0xA0000000
        const PERIPHERAL_BASE_ADDRESS: u64 = 0xA0000000;

        let f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")
            .unwrap();
            //.expect("failed to open the file");

        // Create a new memory map builder and build a map.
        let mmap: MmapMut = unsafe {
            MmapOptions::new()
                .offset(PERIPHERAL_BASE_ADDRESS)
                .len(4096)
                .map_mut(&f)
                .unwrap()
        };
        mmap
    }

    pub fn read_qpc_register(&self, offset:i32) -> u32 {
        unsafe {
            //let registers = self.mmap.as_ptr() as *const Registers;
            //(*registers).counter_low.read()
            *(self.mmap + offset.as_ptr())
        }
    }
}

pub fn take_system_timer() -> QpcAdac {
    let mmap = QpcAdac::map_registers();
    QpcAdac { mmap: mmap }
}
