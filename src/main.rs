#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    dma::DmaPriority,
    gdma::Gdma,
    i2s::{DataFormat, I2s, I2sTx, I2sWriteDma, Standard},
    peripherals::Peripherals,
    prelude::*,
    Delay, IO,
};
use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // GPIO8 is connected to the built-in LED on the TENSTAR ROBOT ESP32-C3
    // SuperMini board, available at
    // https://www.aliexpress.us/item/3256805781327184.html
    let mut led = io.pins.gpio8.into_open_drain_output();

    loop {
        println!("Light goes on.");
        led.enable_output(true);
        delay.delay_ms(1000u32);
        println!("Light goes off.");
        led.enable_output(false);
        delay.delay_ms(1000u32);
    }
}
