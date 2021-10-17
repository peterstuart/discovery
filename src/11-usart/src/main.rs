#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux11::usart1::RegisterBlock;
#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut itm) = aux11::init();

    let mut buffer: Vec<u16, 32> = Vec::new();

    loop {
        let byte = usart1.rdr.read().rdr().bits();

        if buffer.push(byte).is_err() {
            for byte in "buffer full".bytes() {
                wait_until_write_available(&usart1);
                usart1.tdr.write(|w| w.tdr().bits(byte as u16));
            }
        }

        if byte == 13 {
            iprintln!(&mut itm.stim[0], "echo back");
            for byte in &buffer {
                usart1.tdr.write(|w| w.tdr().bits(*byte));
                wait_until_write_available(&usart1);
            }

            buffer.clear();
            usart1.tdr.write(|w| w.tdr().bits(byte));
        }

        wait_until_read_available(&usart1);
    }
}

fn wait_until_read_available(usart1: &RegisterBlock) {
    while usart1.isr.read().rxne().bit_is_clear() {}
}

fn wait_until_write_available(usart1: &RegisterBlock) {
    while usart1.isr.read().txe().bit_is_clear() {}
}
