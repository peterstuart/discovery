#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        i2c1.cr2.write(|w| {
            // TODO Broadcast START
            w.start().set_bit();

            // TODO Broadcast the MAGNETOMETER address with the R/W bit set to Write
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });
        while i2c1.isr.read().txis().bit_is_clear() {}

        // TODO Send the address of the register that we want to read: IRA_REG_M
        i2c1.txdr.write(|w| w.txdata().bits(IRA_REG_M));
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        i2c1.cr2.modify(|_, w| {
            // TODO Broadcast RESTART
            w.start().set_bit();

            // TODO Broadcast the MAGNETOMETER address with the R/W bit set to Read
            w.rd_wrn().set_bit();
            w.nbytes().bits(1);
            w.autoend().set_bit()
        });

        // TODO Receive the contents of the register
        while i2c1.isr.read().rxne().bit_is_clear() {}

        i2c1.rxdr.read().rxdata().bits()
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
