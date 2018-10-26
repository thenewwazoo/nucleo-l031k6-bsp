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
* `openocd -f interface/stlink-v2-1.cfg -f target/stm32l0.cfg`
* NOTE: Your target `stlink.cfg` file may change depending on what version of OpenOCD you're running. 
### Now, the fun part, getting Rust up and going!
* Install or build from source Rust v1.30 or greater. NOTE: If you already have rust installed, feel free to run `rustup update` to get the latest stable version.
* We need to add the thumbv6m-none-eabi target
    * `rustup target add thumbv6m-none-eabi`
* Next, let's clone the repo and get it running!
    * `git clone https://github.com/thenewwazoo/nucleo-l031k6-bsp.git`
    * `cd nucleo-l031k6-bsp`
    * `cargo run`
* Put your hands in the air and yell, "hell yeah!"
