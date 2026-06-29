#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut red_led = pins.d6.into_output();
    let mut green_led = pins.d7.into_output();
    let mut blue_led = pins.d8.into_output();

    let button = pins.d2.into_pull_up_input();

    let mut count = 0;

    red_led.set_high();
    green_led.set_high();
    blue_led.set_high();

    loop {
        if button.is_low() {
            arduino_hal::delay_ms(20);

            while button.is_low() {}

            count += 1;

            if count % 2 != 0 {
                red_led.set_low();
                green_led.set_low();
                blue_led.set_low();
            } else {
                red_led.set_high();
                green_led.set_high();
                blue_led.set_high();
            }
        }
    }
}
