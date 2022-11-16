#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};
use hal::gpio::PullDown;
use nucleo_l031k6_bsp as bsp;
use stm32l0x1_hal as hal;

#[entry]
fn main() -> ! {
    let mut p = cortex_m::Peripherals::take().unwrap();
    let d = hal::stm32l0x1::Peripherals::take().unwrap();

    let mut board = bsp::init::<hal::power::VCoreRange1>(d.PWR, d.FLASH, d.RCC);

    let ticks = board.rcc.cfgr.context().unwrap().sysclk().0;
    board.systick_start(&mut p.SYST, SystClkSource::Core, ticks / 5);

    let pins = board.pins(d.GPIOA, d.GPIOB, d.GPIOC);

    let mut user_led = board.user_led(pins.d13);
    let input_line = pins.d12.into_input::<PullDown>();

    loop {
        if input_line.is_high().unwrap() {
            if user_led.is_set_low().unwrap() {
                user_led.set_high().unwrap();
            } else {
                user_led.set_low().unwrap();
            }
        } else {
            user_led.set_low().unwrap();
        }

        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    //asm::bkpt();
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    //panic!("HardFault at {:#?}", ef);
    panic!("Hardfault");
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    //panic!("Unhandled exception (IRQn = {})", irqn);
    panic!("Unhandled exception");
}
