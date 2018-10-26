NUCLEO-L031K6-BSP
=================

This crate implements a BSP for the [ST NUCLEO-L031K6](https://www.st.com/en/evaluation-tools/nucleo-l031k6.html) development board. It is intended to ease development while relieving the programmer of tracking peripheral and pin combinations, as well as providing helper methods for instantiating peripherals.

An example of crate usage can be found in [`src/main.rs`](src/main.rs)

Requirements
------------

Building this (and the dependent crates) requires Rust v1.30 or later.

Setup
------------
1. Install & compile the latest version of [OpenOCD](http://openocd.org/), then `openocd -f interface/stlink-v2-1.cfg -f target/stm32l0.cfg` and open new terminal session for further work. NOTE: Your target `stlink.cfg` file may change depending on what version of OpenOCD you're running. 
2. `rustup update`
3. `rustup target add thumbv6m-none-eabi`
4. `git clone https://github.com/thenewwazoo/nucleo-l031k6-bsp.git`
5. `cd nucleo-l031k6-bsp`
6. `cargo run`
7. Put your hands in the air and yell, "hell yeah!"
