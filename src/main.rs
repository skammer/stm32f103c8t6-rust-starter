#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;
extern crate cortex_m_rt;

use cortex_m_rt::{ExceptionFrame, entry, exception};
use hal::prelude::*;
use hal::{
    stm32,
    delay::Delay,
    gpio::{
        gpioc::PCx,
        Output,
        PushPull,
    }
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh).downgrade();

    loop {
        bling_bling(&mut delay, &mut led);
    }
}

fn bling_bling(delay: &mut Delay, led: &mut PCx<Output<PushPull>>) {
    led.set_high();
    delay.delay_ms(100_u16);
    led.set_low();
    delay.delay_ms(100_u16);
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
