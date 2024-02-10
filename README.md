# riscv32imc-blink

If you're making your way through the excellent [Rust on ESP](https://esp-rs.github.io/book/)
book, and you're in a hurry, you might have missed that there is already
[an incomplete blink example](https://esp-rs.github.io/book/writing-your-own-application/nostd.html).
So you might write one on your own. At least, that's what I did. It wasn't
until I wrote this README that I discovered the existing one. Oh well.

This example assumes you have a $2 TENSTAR ROBOT ESP32-C3 SuperMini board,
available at <https://www.aliexpress.us/item/3256805781327184.html>. It has a
built-in LED on GPIO 8.

Install with `cargo espflash flash --monitor`.
