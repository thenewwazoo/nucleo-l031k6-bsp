//! STM L031K6-Nucleo board support
//!
//! This crate provides convenience methods for working with the L031K6-Nucleo development board,
//! and is intended to serve as a kind of example for how a board support crate might work. Under
//! the hood, it ties pin function together with peripheral instantiation. Peripherals take as
//! arguments, and subsequently own, the pins they're attached to. This speeds development
//! considerably, and reduces a lot of boilerplate setup code.
//!
//! You can find a basic working example in [`main.rs`].

#![no_std]
#![allow(unused_imports)]

extern crate cortex_m;
extern crate stm32l0x1;
extern crate stm32l0x1_hal as hal;

use cortex_m::peripheral::syst::SystClkSource;
use hal::common::Constrain;
use hal::flash::{self, *};
use hal::gpio::{self, *};
use hal::i2c::{self, *};
use hal::power::{self, *};
use hal::rcc::clocking::*;
use hal::rcc::{self, *};
use hal::serial::{self, *};
use hal::time::{self, *};
use stm32l0x1::*;

/// A configured user LED
pub type Led = gpio::PB3<Output<PushPull, Floating>>;

/// A representation of connectors CN3 and CN4
pub struct Pins {
    // CN3
    /// D1 - MCO, I2C1_SCL, LPTIM1_OUT, USART2_TX, TIM21_CH2, COMP1_OUT
    pub d1: PA9<Analog>,
    /// D0 - TIM21_CH1, I2C1_SDA, RTC_REFIN, USART2_RX, TIM2_CH3, COMP1_OUT
    pub d0: PA10<Analog>,
    /// D2 - SPI1_MOSI, EVENTOUT, USART2_RTS_DE, COMP2_OUT
    pub d2: PA12<Analog>,
    /// D3 - EVENTOUT, SPI1_MISO, TIM2_CH2, USART2_RTS_DE, TIM2_CH3
    pub d3: PB0<Analog>,
    /// D4 - USART2_RX, I2C1_SDA, LPTIM1_IN2, TIM2_CH4, LPUART1_RX
    pub d4: PB7<Analog>,
    /// D5 - USART2_TX, I2C1_SCL, LPTIM1_ETR, TIM2_CH3, LPUART1_TX
    pub d5: PB6<Analog>,
    /// D6 - USART2_CK, SPI1_MOSI, LPTIM1_IN1, LPUART1_RTS_DE , TIM2_CH4
    pub d6: PB1<Analog>,
    /// D7 - OSC32_IN
    pub d7: PC14<Analog>,
    /// D8 - OSC32_OUT
    pub d8: PC15<Analog>,
    /// D9 - MCO, LPTIM1_IN1, EVENTOUT, USART2_CK, TIM2_CH1
    pub d9: PA8<Analog>,
    /// D10 - SPI1_MISO, LPTIM1_OUT, EVENTOUT, USART2_CTS, TIM21_CH2, COMP1_OUT
    pub d10: PA11<Analog>,
    /// D11 - SPI1_MOSI, LPTIM1_IN1, I2C1_SMBA, TIM21_CH1
    pub d11: PB5<Analog>,
    /// D12 - SPI1_MISO, EVENTOUT
    pub d12: PB4<Analog>,

    // CN4
    /// D13 - SPI1_SCK, TIM2_CH2, EVENTOUT
    pub d13: PB3<Analog>,
    /// A0 - USART2_RX, LPTIM1_IN1, TIM2_CH1, USART2_CTS, TIM2_ETR, LPUART1_RX, COMP1_OUT
    pub a0: PA0<Analog>,
    /// A1 - EVENTOUT, LPTIM1_IN2, TIM2_CH2, I2C1_SMBA, USART2_RTS_DE, TIM21_ETR, LPUART1_TX
    pub a1: PA1<Analog>,
    /// A2 - TIM21_CH2, TIM2_CH4, USART2_RX, LPUART1_RX
    pub a2: PA3<Analog>,
    /// A3 - SPI1_NSS, LPTIM1_IN1, LPTIM1_ETR, I2C1_SCL, USART2_CK, TIM2_ETR, LPUART1_TX, COMP2_OUT
    pub a3: PA4<Analog>,
    /// A4 - SPI1_SCK, LPTIM1_IN2, TIM2_ETR, TIM2_CH1
    pub a4: PA5<Analog>,
    /// A5 - SPI1_MISO, LPTIM1_ETR, LPUART1_CTS, EVENTOUT, COMP1_OUT
    pub a5: PA6<Analog>,
    /// A6 - SPI1_MOSI, LPTIM1_OUT, USART2_CTS, TIM21_ETR, EVENTOUT, COMP2_OUT
    pub a6: PA7<Analog>,
    /// A7 - TIM21_CH1, TIM2_CH3, USART2_TX, LPUART1_TX, COMP2_OUT
    pub a7: PA2<Analog>,
}

/// The L031K6-Nucleo
pub struct Board<VDD, VCORE, RTC> {
    /// The constrained Power peripheral
    pub pwr: Power<VDD, VCORE, RTC>,
    /// The constrained Flash peripheral
    pub flash: Flash,
    /// The constrained Rcc peripheral
    pub rcc: Rcc,
}

/// Initialize the MCU and the board
pub fn init<VCORE>(pwr: PWR, flash: FLASH, rcc: RCC) -> Board<VddHigh, VCORE, RtcDis>
where
    VCORE: Vos + FreqLimit + Latency,
{
    let pwr: Power<VddHigh, VCoreRange2, RtcDis> = pwr.constrain();
    let mut pwr = pwr.into_vcore_range::<VCORE>();
    let mut flash = flash.constrain();
    let mut rcc = rcc.constrain();

    {
        let cfgr = rcc.cfgr.config().unwrap();

        cfgr.msi.enable();
        cfgr.msi.set_freq(clocking::MsiFreq::Hz_2_097_000);
        cfgr.hsi16.enable();
        cfgr.hclk_fclk = Hertz(2_097_000);
        cfgr.pclk1 = Hertz(2_097_000);
        cfgr.pclk2 = Hertz(2_097_000);
        cfgr.sysclk_src = clocking::SysClkSource::MSI;
    }

    rcc.freeze(&mut flash, &mut pwr);

    Board { pwr, flash, rcc }
}

impl<VDD, VCORE, RTC> Board<VDD, VCORE, RTC> {
    /// Obtain Pins for this board in their post-reset state
    pub fn pins(&mut self, gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC) -> Pins {
        let gpioa = gpio::A::new(gpioa, &mut self.rcc.iop);
        let gpiob = gpio::B::new(gpiob, &mut self.rcc.iop);
        let gpioc = gpio::C::new(gpioc, &mut self.rcc.iop);

        Pins {
            d1: gpioa.PA9,
            d0: gpioa.PA10,
            d2: gpioa.PA12,
            d3: gpiob.PB0,
            d4: gpiob.PB7,
            d5: gpiob.PB6,
            d6: gpiob.PB1,
            d7: gpioc.PC14,
            d8: gpioc.PC15,
            d9: gpioa.PA8,
            d10: gpioa.PA11,
            d11: gpiob.PB5,
            d12: gpiob.PB4,

            d13: gpiob.PB3,
            a0: gpioa.PA0,
            a1: gpioa.PA1,
            a2: gpioa.PA3,
            a3: gpioa.PA4,
            a4: gpioa.PA5,
            a5: gpioa.PA6,
            a6: gpioa.PA7,
            a7: gpioa.PA2,
        }
    }

    /// Set the up SysTick exception to be called every `ticks` CPU cycles
    pub fn systick_start(&mut self, syst: &mut SYST, src: SystClkSource, ticks: u32) {
        syst.set_clock_source(src);
        syst.set_reload(ticks);
        syst.clear_current();
        syst.enable_counter();
        syst.enable_interrupt();
    }

    /// Configure pin D13 to be used to drive the user led LD3
    pub fn user_led<T>(&mut self, d13: PB3<T>) -> Led {
        d13.into_output::<PushPull, Floating>()
    }

    /// Initialize the VCP UART (pass-through the STLink USB) and return Tx and Rx pins
    pub fn vcp_usart<T>(
        &mut self,
        usart: USART2,
        tx_pin_a7: PA2<T>,
        clk_src: USARTClkSource,
    ) -> Serial<USART2, (PA2<AF::AF4>, PA15<AF::AF4>)> {
        // safe because we moved GPIOB when we created the Pins that gives us the PB3
        let gpioa = gpio::A::new(
            unsafe { stm32l0x1::Peripherals::steal() }.GPIOA,
            &mut self.rcc.iop,
        );
        let rx_pin = gpioa.PA15;

        let vcp_tx: PA2<AF::AF4> = tx_pin_a7
            .into_output::<PushPull, Floating>()
            .into_alt_fun::<AF::AF4>();
        vcp_tx.set_pin_speed(PinSpeed::VeryHigh);

        let vcp_rx: PA15<AF::AF4> = rx_pin
            .into_output::<PushPull, Floating>()
            .into_alt_fun::<AF::AF4>();
        vcp_rx.set_pin_speed(PinSpeed::VeryHigh);

        usart2::rs232(
            usart,
            (vcp_tx, vcp_rx),
            Bps(115200),
            clk_src,
            self.rcc.cfgr.context().unwrap(),
            &mut self.rcc.apb1,
            &mut self.rcc.ccipr,
        )
    }

    /// Initialize I2C1 and return the peripheral
    pub fn i2c1<C, A>(
        &mut self,
        i2c1: I2C1,
        pins: (PB6<C>, PB7<A>),
    ) -> I2c<I2C1, (PB6<AF::AF1>, PB7<AF::AF1>)> {
        let i2c_sda = pins
            .1
            .into_output::<OpenDrain, PullUp>()
            .into_alt_fun::<AF::AF1>();
        i2c_sda.set_pin_speed(PinSpeed::VeryHigh);

        let i2c_scl = pins
            .0
            .into_output::<OpenDrain, PullUp>()
            .into_alt_fun::<AF::AF1>();
        i2c_scl.set_pin_speed(PinSpeed::VeryHigh);

        i2c::I2c::i2c1(
            i2c1,
            (i2c_scl, i2c_sda),
            //Hertz(100_000),
            i2c::I2cClkSrc::HSI16,
            //&clk_ctx,
            0x00303D5B, // timing
            &mut self.rcc.apb1,
            &mut self.rcc.ccipr,
        )
    }
}
