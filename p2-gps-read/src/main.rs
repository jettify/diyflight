#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::{Config as UsartConfig, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart, Config};
use embassy_time::{with_timeout, Duration};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    let usart_config = UsartConfig::default();
    let gps_usart = Uart::new(
        p.USART1,
        p.PB7,
        p.PB6,
        Irqs,
        p.DMA2_CH7,
        p.DMA2_CH5,
        usart_config,
    )
    .unwrap();

    let mut dma_buf = [0u8; 128];
    let (_tx, gps_usart_rx) = gps_usart.split();
    read_gnss(gps_usart_rx.into_ring_buffered(&mut dma_buf), led).await;
}

pub async fn read_gnss(mut uart_rx: impl embedded_io_async::Read, mut led: Output<'_>) -> ! {
    const ID: &str = "gnss_reader";

    let mut buffer = [0u8; 128];
    let mut parse_buffer = [0u8; 128];
    let buf = ublox::FixedLinearBuffer::new(&mut parse_buffer);
    let mut parser = ublox::Parser::new(buf);

    info!("{}: Entering main loop", ID);
    'parsing: loop {
        led.toggle();
        // Read serial data, but with a timeout
        let bytes = match with_timeout(Duration::from_secs(1), uart_rx.read(&mut buffer)).await {
            Ok(Ok(bytes)) => bytes,
            Ok(Err(_)) => {
                error!("{}: Failed to read serial data", ID);
                continue 'parsing;
            }
            Err(_) => {
                warn!("{}: Timeout while reading serial data", ID);
                continue 'parsing;
            }
        };

        let sub_buffer = &buffer[..bytes];
        let mut consumed = parser.consume(sub_buffer);

        while let Some(parsed) = consumed.next() {
            match parsed {
                Ok(packet) => match packet {
                    ublox::PacketRef::NavPvt(pvt) => {
                        info!(
                            "satellites: {} lat_raw: {} lon_raw: {} altitude: {}",
                            pvt.num_satellites(),
                            pvt.latitude_raw(),
                            pvt.longitude_raw(),
                            pvt.height_msl()
                        );
                    }
                    _ => error!("{}: Unknown GNSS packet type", ID),
                },
                Err(_) => error!("{}: Error parsing GNSS packet", ID),
            }
        }
    }
}
