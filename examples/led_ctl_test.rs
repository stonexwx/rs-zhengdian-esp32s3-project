use esp32_test::led::Led;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut led = Led::new(1);
    led.on();
    std::thread::sleep(std::time::Duration::from_secs(1));
    led.off();
    std::thread::sleep(std::time::Duration::from_secs(1));
    led.on();
    std::thread::sleep(std::time::Duration::from_secs(1));

    led.toggle();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        led.on();
        std::thread::sleep(std::time::Duration::from_secs(1));
        led.off();
    }
}
