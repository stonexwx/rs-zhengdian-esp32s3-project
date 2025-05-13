use esp32_test::led::Led;

const GPIO_NUM_1: i32 = 1; // GPIO引脚编号
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut led = Led::new(GPIO_NUM_1);
    led.on();
    std::thread::sleep(std::time::Duration::from_secs(1));
    led.off();
    std::thread::sleep(std::time::Duration::from_secs(1));
    led.on();
    std::thread::sleep(std::time::Duration::from_secs(1));

    

    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
        led.toggle();
    }
}
