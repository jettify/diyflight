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
        pit,
        ..
    } = board::t41(board::instances());
    let board_led = gpio2.output(pins.p13);

    let mut delay = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit.0);

    loop {
        board_led.toggle();
        delay.block_ms(100);
    }
}
