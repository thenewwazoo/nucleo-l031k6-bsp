#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate embedded_hal;
extern crate l031k6_nucleo_bsp as bsp;
extern crate panic_abort;
extern crate stm32l0x1;
extern crate stm32l0x1_hal as hal;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use embedded_hal::digital::StatefulOutputPin;
use embedded_hal::prelude::*;
use hal::time::Bps;
use rt::ExceptionFrame;

#[entry]
fn main() -> ! {
    let mut p = cortex_m::Peripherals::take().unwrap();
    let d = hal::stm32l0x1::Peripherals::take().unwrap();

    let mut board = bsp::init::<hal::power::VddHigh>(d.PWR, d.FLASH, d.RCC);

    let ticks = board.rcc.cfgr.context().unwrap().sysclk().0;
    board.systick_start(&mut p.SYST, SystClkSource::Core, ticks);

    let pins = board.pins(d.GPIOA, d.GPIOB, d.GPIOC);

    let mut user_led = board.user_led(pins.d13);

    let (_vcp_tx, _vcp_rx) = board.vcp_usart_pins(
        d.USART2,
        pins.a7,
        Bps(9600),
        hal::rcc::clocking::USARTClkSource::HSI16,
    );

    let _led_i2c = board.i2c1(d.I2C1, (pins.d5, pins.d4));

    loop {
        if user_led.is_set_high() {
            user_led.set_low();
        } else {
            user_led.set_high();
        }

        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    // Do nothing here, simply unblock the main loop from its wfi
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
