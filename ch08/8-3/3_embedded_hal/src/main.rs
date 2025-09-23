//#![no_std]
//#![no_main]
//
//extern crate panic_halt;
//
//use cortex_m_rt::entry;
//use stm32f4xx_hal::uart::{Serial, config::Config, config::StopBits};
//use stm32f4xx_hal::{pac, prelude::*};
//
//#[entry]
//fn main() -> ! {
//    if let Some(dp) = pac::Peripherals::take() {
//        let rcc = dp.RCC.constrain();
//        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
//        let gpioc = dp.GPIOC.split();
//
//        let config = Config::default()
//            .baudrate(115200.bps())
//            .wordlength_8()
//            .parity_none()
//            .stopbits(StopBits::STOP1);
//
//        let pins = (gpioc.pc10.into_alternate(), gpioc.pc11.into_alternate());
//        let mut uart4 = Serial::new(dp.UART4, pins, config, &clocks).unwrap();
//
//        let _ = uart4.write(b'H');
//        let _ = uart4.write(b'e');
//        let _ = uart4.write(b'l');
//        let _ = uart4.write(b'l');
//        let _ = uart4.write(b'o');
//        let _ = uart4.flush();
//    }
//
//    loop {}
//}

#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f4xx_hal::{i2c::I2c, pac, prelude::*};

#[entry]
fn main() -> ! {
    if let Some(dp) = pac::Peripherals::take() {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let gpiob = dp.GPIOB.split();

        let pins = (
            gpiob.pb8.into_alternate().set_open_drain(),
            gpiob.pb9.into_alternate().set_open_drain(),
        );
        let mut i2c = I2c::new(dp.I2C1, pins, 400.kHz(), &clocks);
        let _ = i2c.write(0u8, &[0]);
    }

    loop {}
}
