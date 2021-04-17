//! main app in cortex-m-rtic version
//!
//! See also `main_rt.rs` for a RT-only version.

#![no_std]
#![no_main]
// #![deny(warnings)]

use rtic::cyccnt::{Instant, U32Ext as _};

use heapless::consts::*;
use heapless::Vec;


const CLOCK_FREQ: u32 = 96_000_000;

// type MyBuffer = Vec<u8, U1024>;
type MyBuffer = Vec<u8, U2048>;

#[inline(never)]
fn use_buffer(buffer: &MyBuffer) {
    cortex_m_semihosting::hprintln!("Buffer contains {} elements", buffer.len());
    cortex_m_semihosting::hprintln!("Check stack allocation in gdb now.");
    loop {}
}

#[inline(never)]
fn init_buffer() -> MyBuffer {
    MyBuffer::new()
}

#[inline(never)]
fn add_some_bytes(mut buffer: MyBuffer) -> MyBuffer {
    buffer.push(0).unwrap();
    buffer.push(1).unwrap();
    buffer.push(2).unwrap();
    buffer
}

#[rtic::app(device = rtic_demo_memory::hal::raw, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {

    struct Resources {
        tmpbuffer: MyBuffer,
    }

    #[init(schedule = [])]
    fn init(c: init::Context) -> init::LateResources {

        let random_int = rtic_demo_memory::init_board(c.device, c.core);

        let mut mybuffer = init_buffer();
        mybuffer.push(random_int).unwrap();
        let mybuffer = add_some_bytes(mybuffer);

        init::LateResources {
            tmpbuffer: mybuffer,
        }
    }

    #[idle(resources = [tmpbuffer])]
    fn idle(c: idle::Context) -> ! {
        let idle::Resources { tmpbuffer } = c.resources;


        cortex_m_semihosting::hprintln!("Idle loop");

        loop {
            use_buffer(tmpbuffer);
        }
    }

    // #[task(resources = [], schedule = [check_stack_usage], priority = 5)]
    // fn check_stack_usage(c: check_stack_usage::Context) {
    //     let _res = c.resources;
    // }

    extern "C" {
        fn PLU();
    }

};
