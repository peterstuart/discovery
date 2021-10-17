#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let instant = mono_timer.now();

    for byte in "The quick brown fox jumps over the lazy dog.".bytes() {
        while usart1.isr.read().txe().bit_is_clear() {}

        usart1.tdr.write(|w| w.tdr().bits(u16::from(byte)));
    }

    let elapsed = instant.elapsed();

    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    // testing
    loop {}
}
