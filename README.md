NUCLEO-L031K6-BSP
=================

This crate implements a BSP for the [ST NUCLEO-L031K6](https://www.st.com/en/evaluation-tools/nucleo-l031k6.html) development board. It is intended to ease development while relieving the programmer of tracking peripheral and pin combinations, as well as providing helper methods for instantiating peripherals.

An example of crate usage can be found in [`src/main.rs`](src/main.rs)

Requirements
------------

Building this (and the dependent crates) requires Rust v1.30 or later.

Setup
------------
### We need to we need to get OpenOCD on our machine. If you already have it, feel free to skip this step. 
* Install or build from source the latest version of [OpenOCD](http://openocd.org/).
### To launch OpenOCD and target your ST NUCLEO-L031K6
* Launch OpenOCD and target your STM32 by executing the following command. `openocd -f interface/stlink-v2-1.cfg -f target/stm32l0.cfg`
* NOTE: In OpenOCD's current tip of master, `stlink-v2-1.cfg` (the config file needed when running OpenOCD from latest stable release, 0.10.0) has been [deprecated](https://github.com/ntfreak/openocd/commit/31c58c139d85c35cc8ebce4196edb2c5eb157c7a#diff-6f768e414112031085566a8f02b014e6), and is now replaced by `stlink.cfg`.
### Now, the fun part, getting Rust up and going!
* Install or build from source Rust v1.30 or greater. NOTE: If you already have rust installed, feel free to run `rustup update` to get the latest stable version.
* We need to add the thumbv6m-none-eabi target
    * `rustup target add thumbv6m-none-eabi`
* Next, let's clone the repo and get it running!
    * `git clone https://github.com/thenewwazoo/nucleo-l031k6-bsp.git`
    * `cd nucleo-l031k6-bsp`
    * `cargo run`

Example Code
------------
The current [`example code`](src/main.rs) initializes the board:
```
let mut p = cortex_m::Peripherals::take().unwrap();
let d = hal::stm32l0x1::Peripherals::take().unwrap();

let mut board = bsp::init::<hal::power::VCoreRange1>(d.PWR, d.FLASH, d.RCC);
```

Starts a system clock: 
```
let ticks = board.rcc.cfgr.context().unwrap().sysclk().0;
board.systick_start(&mut p.SYST, SystClkSource::Core, ticks / 1000);
```

And initializes the D12 and D13 pins:
```
let pins = board.pins(d.GPIOA, d.GPIOB, d.GPIOC);

let mut user_led = board.user_led(pins.d13);
let input_line = pins.d12.into_input::<PullDown>();
```

It then enters a loop with a simple conditional check. If the D12 pin is set to `HIGH`, then D13 is set to `HIGH`, and vice versa. 

To test the example code, flash a board using the setup instructions above, and short the D12 and 3.3v pins on your ST NUCLEO-L031K6.
