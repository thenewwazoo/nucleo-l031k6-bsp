#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate embedded_hal;
extern crate nucleo_l031k6_bsp as bsp;
extern crate panic_halt;
extern crate stm32l0x1;
extern crate stm32l0x1_hal as hal;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use embedded_hal::digital::StatefulOutputPin;
use embedded_hal::prelude::*;
use hal::gpio::PullDown;
use rt::ExceptionFrame;

#[entry]
fn main() -> ! {
    let mut p = cortex_m::Peripherals::take().unwrap();
    let d = hal::stm32l0x1::Peripherals::take().unwrap();

    let mut board = bsp::init::<hal::power::VCoreRange1>(d.PWR, d.FLASH, d.RCC);

    let ticks = board.rcc.cfgr.context().unwrap().sysclk().0;
    board.systick_start(&mut p.SYST, SystClkSource::Core, ticks / 1000);

    let pins = board.pins(d.GPIOA, d.GPIOB, d.GPIOC);

    let mut user_led = board.user_led(pins.d13);
    let input_line = pins.d12.into_input::<PullDown>();

    loop {
        if user_led.is_set_low() {
            user_led.set_high();
        } else {
            user_led.set_low();
        }

        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    //asm::bkpt();
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    //panic!("HardFault at {:#?}", ef);
    panic!("Hardfault");
}

#[exception]
fn DefaultHandler(_irqn: i16) {
    //panic!("Unhandled exception (IRQn = {})", irqn);
    panic!("Unhandled exception");
}
