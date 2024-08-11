# rustyblink
### A bare metal rust template for the STM32 Microcontroller series. 

I got myself a NUCELO board withe the STM32C0xx microcontroller from EMbedded World earlier this year and I had some fun playing around with it in good old `Embedded C`. But when I was checking on the rust community, I realised that a lot had passed me by. Rust supports embedded systems now. While the support is fairly new and the ground is still shifting a lot, I think the project is at a point where I can start to seriously consider the language for products moving forward especially for projects that may use earliy adopter micros like the stm32f0 and nRF52xx micros.

At the time of posting this, the `STM32C0` series is not mauturely supported yet so I felt it might be helpful to put this out there.

 
> ⚠️ **Warning:** This has **only** been tested on the `STM32C031C6Tx` 

depends on 
- `cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }`
- `cortex-m-rt = "0.7.3"`
- `panic-halt = "0.2.0"`
- `rtt-target = "0.5.0"`

you must install `probe-rs` that will come with `cargo-embed` from this [link](https://probe.rs/docs/getting-started/installation/) to build and flash your program.
This application supports debugging with `arm-gdb`

## Using any other ARM microcontroller
Change the following:
1. change `target = "thumbv6m-none-eabi"` to the respective ARM target in `.cargo/config.toml`
2. change `"rust-analyzer.cargo.target": "thumbv6m-none-eabi"` in `.vscode/settings.json` for rust analyzer to help you toe the very thin line that rust forces you renegades to tread. 
3. change the linker script `memory.x` to suite your microcontroller
4. change the chip name `STM32C031C6Tx` in `Embed.toml` to the name of your chip according to the `cargo embed --chip` argument
5. of course be mindful of the registers used in this example

## Future updates
Use [svd2rust](https://docs.rs/svd2rust/latest/svd2rust/) to generate a peripheral access crate for the stm32c0 in case the heroes at the stm32 rust project won't have gotten to it first. ;-)  
That will make it easier to access all the peripherals and handlers withought having to break too many rust rules. 
