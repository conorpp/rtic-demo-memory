#![no_std]

pub use lpc55_hal as hal;
use hal::prelude::*;

use panic_semihosting as _;

#[inline(never)]
pub fn init_board(
    device_peripherals: hal::raw::Peripherals,
    // core_peripherals: rtic::Peripherals
    core_peripherals: hal::raw::CorePeripherals,
) -> u8 {

    let hal = hal::Peripherals::from((device_peripherals, core_peripherals));

    let mut anactrl = hal.anactrl;
    let mut pmc = hal.pmc;
    let mut syscon = hal.syscon;

    let _clocks = hal::ClockRequirements::default()
        .system_frequency(96.mhz())
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .expect("Clock configuration failed");

    // something that won't get optimized
    let random_int = anactrl.release().analog_ctrl_status.read().bits();
    return random_int as u8;
}
