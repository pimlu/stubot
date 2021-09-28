#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

//#[macro_use]
extern crate alloc;

use hal::prelude::*;
use hal::serial::*;
use hal::stm32;
use stm32g4xx_hal as hal;

use core::fmt::Write;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

use chess::*;
use engine::*;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    let start = cortex_m_rt::heap_start() as usize;
    let size = 16 * 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(hal::rcc::Config::hsi());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb8.into_push_pull_output();

    let tx = gpioa.pa2.into_alternate();
    let rx = gpioa.pa3.into_alternate();
    let mut usart = dp
        .USART2
        .usart(tx, rx, FullConfig::default().baudrate(9600.bps()), &mut rcc)
        .unwrap();

    let mut state = State::default();
    let mut searcher = Searcher::new();

    loop {
        writeln!(usart, "{}", state.board_string()).unwrap();
        let (mv, score) = searcher.negamax(&mut state, SearchParams::new(4), &BlockSignal {});
        writeln!(usart, "best move: {}. score: {}.", mv.unwrap(), score).unwrap();
        led.toggle().unwrap();
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
