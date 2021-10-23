#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::{FRAC_PI_8, PI};

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, switch_hal::OutputSwitch, Direction, I16x3};
// this trait provides the `atan2` method
use m::Float;

#[entry]
fn main() -> ! {
    let (leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();
    let mut leds = leds.into_array();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = (y as f32).atan2(x as f32); // in radians

        let dir = if theta >= FRAC_PI_8 && theta <= 3.0 * FRAC_PI_8 {
            Direction::Southeast
        } else if theta > 3.0 * FRAC_PI_8 && theta <= 5.0 * FRAC_PI_8 {
            Direction::East
        } else if theta > 5.0 * FRAC_PI_8 && theta <= 7.0 * FRAC_PI_8 {
            Direction::Northeast
        } else if theta > 7.0 * FRAC_PI_8 || theta <= -7.0 * FRAC_PI_8 {
            Direction::North
        } else if theta > -7.0 * FRAC_PI_8 && theta <= -5.0 * FRAC_PI_8 {
            Direction::Northwest
        } else if theta > -5.0 * FRAC_PI_8 && theta <= -3.0 * FRAC_PI_8 {
            Direction::West
        } else if theta > -3.0 * FRAC_PI_8 && theta <= -1.0 * FRAC_PI_8 {
            Direction::Southwest
        } else {
            Direction::South
        };

        leds.iter_mut().for_each(|led| led.off().unwrap());
        leds[dir as usize].on().unwrap();

        delay.delay_ms(100_u8);
    }
}
