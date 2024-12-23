#![no_std]
#![no_main]

use bsp::{board, hal::timer::Blocking};
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins,
        mut gpio2,
        mut gpio4,
        pit,
        ..
    } = board::t41(board::instances());

    let mut delay = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit.0);

    let board_led = gpio2.output(pins.p13);
    board_led.set();

    let green_led = gpio2.output(pins.p6);
    let red_led = gpio4.output(pins.p5);

    loop {
        green_led.toggle();
        delay.block_ms(100);
        red_led.toggle();
        delay.block_ms(100);
    }
}
