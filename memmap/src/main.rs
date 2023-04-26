#![allow(dead_code, unused)]

extern crate memmap;
use memmap::Mmap;
use memmap::MmapMut;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;

use clap::{App, Arg}; // Crate to process the command line arguments
use memmap::MmapOptions;

use std::io::Read;
use std::io::Write;

use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use std::{thread, time::Duration};
use volatile::Volatile;

const O_DIRECT: i32 = 0x4000; // Double check value
const O_SYNC: i32 = 1052672;

const QPC_PHY_BASEADDRESS: usize = 0xA0000000;

const MAP_SIZE: usize = 4096;
const MAP_MASK: usize = (MAP_SIZE - 1);

const QPC_INFO_REG_OFFSET: usize = 0x00;
const QPC_SDAC_CTRL_REG_OFFSET: usize = 0x28;
const QPC_SDAC_CONFIG_REG_OFFSET: usize = 0x2C;
const QPC_SDAC_SPAN_REG_OFFSET: usize = 0x30;
const QPC_SDAC_VOLTAGE0_REG_OFFSET: usize = 0x34;
const QPC_SDAC_VOLTAGE1_REG_OFFSET: usize = 0x38;
const QPC_SDAC_VOLTAGE2_REG_OFFSET: usize = 0x3C;
const QPC_SDAC_VOLTAGE3_REG_OFFSET: usize = 0x40;
const QPC_SDAC_READBACK_REG_OFFSET: usize = 0x44;

fn map_physical_mem_rw(
    phy_addr: usize,
    mmap_len: usize,
) -> Result<MmapMut, Box<dyn std::error::Error>> {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        //.custom_flags(O_DIRECT | O_SYNC)
        .open("/dev/mem")?;

    let m = unsafe {
        memmap::MmapOptions::new()
            .offset(phy_addr as u64)
            .len(mmap_len)
            .map_mut(&fd)?
    };

    Ok(m)
}

fn write_to_qpc<T>(phy_addr: usize, val_to_write: T)
where
    T: std::fmt::LowerHex,
{
    let sz = std::mem::size_of::<T>();
    let m = match map_physical_mem_rw(phy_addr, sz) {
        Ok(m) => m,
        Err(err) => {
            panic!("Failed to mmap: Err={:?}", err);
        }
    };
    let p = m.as_ptr() as *mut T;
    unsafe {
        std::ptr::write_volatile(p, val_to_write);
    }
}

fn read_from_qpc<T>(phy_addr: usize, no_of_data_units: usize)
where
    T: std::fmt::LowerHex,
{
    let sz = std::mem::size_of::<T>();

    let m = match map_physical_mem_rw(phy_addr, no_of_data_units * sz) {
        Ok(m) => m,
        Err(err) => {
            panic!("Failed to mmap: Err={:?}", err);
        }
    };

    let p = m.as_ptr() as *const T;

    (0..no_of_data_units).for_each(|x| unsafe {
        println!(
            "{:08x}: {:02$x}",
            phy_addr + sz * x,
            std::ptr::read_volatile(p.offset(x as isize)),
            sz * 2
        );
    });
}

fn parse_hex(s: &String) -> Result<usize, Box<dyn std::error::Error>> {
    let s = if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else {
        s
    };
    Ok(usize::from_str_radix(s, 16)?)
}

fn print_usage(name: &str) {
    eprintln!("Write physical memory by specified size.");
    eprintln!("Usage: {} size address val", name);
    eprintln!("  where size={{1,2,4,8}}, address and val in hexadecimal.");
}

fn main() {
    let mut qpc_info_reg_val: u32 = 0;
    let mut qpc_calib_reg_val: u32 = 0;
    let mut qpc_ch_span_reg_val: u32 = 0;

    println!("QPC calibration register value:");
    read_from_qpc::<u32>(QPC_PHY_BASEADDRESS + QPC_SDAC_CTRL_REG_OFFSET, 1);

    /* Step 1.1 : enable internal voltage reference: SDAC_CTRL.REF = 0 */
    qpc_calib_reg_val &= !(1 << 2);

    /* Step 1.2: Ensure SPI peripheral communication: SDAC_CTRL.MSPAN = 111 */
    qpc_calib_reg_val |= (0x7 << 3); // Set 3-5 bits

    /* Step 2: Ensure the reset is disabled on any new configuration */
    qpc_calib_reg_val &= !(1 << 1);

    /* Stop the DAC signal generation */
    qpc_calib_reg_val &= !(1);

    /* Set the reserved bits */
    qpc_calib_reg_val |= (0x7 << 13); // Set 15:13 bits

    write_to_qpc::<u32>(
        QPC_PHY_BASEADDRESS + QPC_SDAC_CTRL_REG_OFFSET,
        qpc_calib_reg_val as u32,
    );

    /* Step 3: Issue Channel Span values by writing to SDAC_SPAN register */
    /* DAC0 span codes and ranges : 0000 = (0-5)V or (0-2VREF). Configuring for
     * all the channels */
    qpc_ch_span_reg_val = 0;

    write_to_qpc::<u32>(
        QPC_PHY_BASEADDRESS + QPC_SDAC_SPAN_REG_OFFSET,
        qpc_ch_span_reg_val as u32,
    );

    /* Step 4: Issue DAC Codes corresponding to the required Channel Voltages in
     * SDAC_VOLTAGE0-SDAC_VOLTAGE3 registers.
     */
    write_to_qpc::<u32>(
        QPC_PHY_BASEADDRESS + QPC_SDAC_VOLTAGE0_REG_OFFSET,
        0x60008000 as u32,
    );

    write_to_qpc::<u32>(
        QPC_PHY_BASEADDRESS + QPC_SDAC_VOLTAGE1_REG_OFFSET,
        0xf000fff0 as u32,
    );

    const DAC_CODE: u32 = 0x80008000;
    write_to_qpc::<u32>(
        QPC_PHY_BASEADDRESS + QPC_SDAC_VOLTAGE2_REG_OFFSET,
        DAC_CODE as u32,
    );

    write_to_qpc::<u32>(
        QPC_PHY_BASEADDRESS + QPC_SDAC_VOLTAGE3_REG_OFFSET,
        DAC_CODE as u32,
    );

    /* Step 5: Configure required channels to be powered-up by setting
     * corresponding bits in CHANNEL. SDAC_CTRL.CHANNEL = 00001111 (Enable
     * Ch3-Ch0)
     */
    qpc_calib_reg_val |= (0xF << 16); // Set 16-19 bits

    /* Step 6: Initiate DAC outputs by setting GEN: SDAC_CTRL.GEN = 1 */
    qpc_calib_reg_val |= 0x1; // Set the 0 bit

    write_to_qpc::<u32>(
        QPC_PHY_BASEADDRESS + QPC_SDAC_CTRL_REG_OFFSET,
        qpc_calib_reg_val as u32,
    );

    println!("QPC calibration register value:");
    read_from_qpc::<u32>(QPC_PHY_BASEADDRESS + QPC_SDAC_CTRL_REG_OFFSET, 1);

    // Let's sleep for 2 seconds:
    thread::sleep(Duration::from_secs(60));

    /* Step 7: Stop DAC outputs by clearing GEN.: SDAC_CTRL.GEN = 0 */

    println!("QPC calibration register value:");
    read_from_qpc::<u32>(QPC_PHY_BASEADDRESS + QPC_SDAC_CTRL_REG_OFFSET, 1);

    /*
        /* Read, Modify and write */
        qpc_calib_reg_val &= ~(0x1); // Clear 0 bit.

        rc = write32_to_qpc((const int) qpc_calib_reg_val,
                            qpc_vaddr_dac + QPC_SDAC_CTRL_REG_OFFSET);

        if (rc) {
          printf("Failed to read the QPC register\n");
          goto err;
        }

        qpc_close_memory_map();
    */
}
