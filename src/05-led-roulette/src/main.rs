#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

const CYCLE_PERIOD_MS: u16 = 250;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let tick_delay: u16 = CYCLE_PERIOD_MS / (leds.len() as u16);
    let cycle: usize = leds.len() * 2;

    let mut tick: usize = 0;

    loop {
        turn_on(&mut leds, tick);
        turn_off(&mut leds, tick);
        delay.delay_ms(tick_delay);
        tick = (tick + 1) % cycle;
    }
}

fn turn_on(leds: &mut LedArray, tick: usize) {
    if tick % 2 != 0 {
        return;
    }

    let index = (tick / 2 + 1) % leds.len();
    leds[index].on().unwrap();
}

fn turn_off(leds: &mut LedArray, tick: usize) {
    if tick % 2 != 1 {
        return;
    }

    let index = tick / 2;
    leds[index].off().unwrap();
}
