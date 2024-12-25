#![no_std]
#![no_main]

use bsp::{board, hal::timer::Blocking};
use hal::adc;
use imxrt_hal as hal;
use imxrt_log as logging;
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins,
        usb,
        mut gpio2,
        mut adc1,
        pit,
        ..
    } = board::t41(board::instances());
    let mut delay = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit.0);
    let mut poller = logging::log::usbd(usb, logging::Interrupts::Disabled).unwrap();

    // 0 - 3.3V measured by 10bit adc via 5:1 voltage divider
    const VOLTAGE_SCALE: f32 = ((3.3) / 1024.0) * 5.0;

    let board_led = gpio2.output(pins.p13);
    board_led.clear();
    let mut a1 = adc::AnalogInput::new(pins.p15);
    board_led.set();

    loop {
        let voltage: u16 = adc1.read_blocking(&mut a1);
        let val: f32 = voltage as f32 * VOLTAGE_SCALE;

        log::info!("V: {val}");
        poller.poll();
        delay.block_ms(100);
    }
}
