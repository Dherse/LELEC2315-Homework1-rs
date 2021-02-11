// TODO: use bindgen to generate binding to the hps_0.h file
use std::{error::Error, fs::OpenOptions, mem::size_of, os::unix::fs::OpenOptionsExt, thread, time::Duration};

use memmap::MmapOptions;
use libc::O_SYNC;
use crate::sys::{COUNTER_PIO_BASE, BUTTON_PIO_BASE, ALT_LWFPGASLVS_OFST};

mod sys;
fn main() -> Result<(), Box<dyn Error>> {
    let mem = OpenOptions::new()
        .write(true)
        .read(true)
        .custom_flags(O_SYNC)
        .open("/dev/mem")?;
    
    let mut map = unsafe {
        MmapOptions::new()
            .offset(ALT_LWFPGASLVS_OFST as _)
            .len(COUNTER_PIO_BASE as usize + size_of::<u32>())
            .map_mut(&mem)?
    };

    let counter: &mut u32 = unsafe { &mut *map.as_mut_ptr().add(COUNTER_PIO_BASE as usize).cast() };
    let buttons: &u32 = unsafe { &*map.as_ptr().add(BUTTON_PIO_BASE as usize).cast() };

    *counter = 24999999;
    loop {
        let increase_timer = (*buttons & 0x1) == 0;
        let decrease_timer = (*buttons & 0x2) == 0;

        if increase_timer && *counter < 49_999_999 {
            *counter += 2_500_000;
        }

        if decrease_timer && *counter > 2_500_000 {
            *counter -= 2_500_000;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
