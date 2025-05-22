use esp32_test::drivers::gpio::{GpioHandler, GpioInterruptType, GpioMode, GpioPullMode};
/**
 * @file gpio_led_test.rs
 * @brief 使用封装的GPIO模块控制LED的示例
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
use std::thread;
use std::time::Duration;

// 请根据您的硬件实际情况修改此常量
const LED_GPIO_PIN: u32 = 1;

fn main() {
    // 初始化ESP-IDF
    esp_idf_svc::sys::link_patches();
    // 初始化日志
    esp_idf_svc::log::EspLogger::initialize_default();

    println!("GPIO LED控制示例开始运行!");

    // 创建一个GPIO处理实例
    let led_gpio = GpioHandler::new(LED_GPIO_PIN);

    // 将GPIO初始化为输出模式
    led_gpio
        .init(
            GpioMode::Output,           // 设置为输出模式
            GpioPullMode::Floating,     // 浮空(无上拉下拉)
            GpioInterruptType::Disable, // 禁用中断
        )
        .expect("GPIO初始化失败");

    println!("LED GPIO初始化完成，开始LED闪烁测试...");

    // 快速闪烁5次
    for i in 1..=5 {
        println!("LED闪烁 #{}", i);

        // 打开LED
        led_gpio.set_level(1).expect("设置GPIO高电平失败");
        thread::sleep(Duration::from_millis(200));

        // 关闭LED
        led_gpio.set_level(0).expect("设置GPIO低电平失败");
        thread::sleep(Duration::from_millis(200));
    }

    println!("快速闪烁测试完成，开始呼吸灯效果...");

    // 模拟呼吸灯效果
    // 注意：这需要LED连接的GPIO支持PWM，或者我们自己模拟PWM效果
    // 此处简单模拟呼吸效果
    for _ in 0..3 {
        // 循环3次呼吸效果
        // 渐亮
        for i in 0..10 {
            let on_time = i * 50; // 0 to 450ms
            let off_time = 500 - on_time; // 500 to 50ms

            led_gpio.set_level(1).expect("设置GPIO高电平失败");
            thread::sleep(Duration::from_millis(on_time));

            led_gpio.set_level(0).expect("设置GPIO低电平失败");
            thread::sleep(Duration::from_millis(off_time));
        }

        // 渐暗
        for i in 0..10 {
            let on_time = 450 - i * 50; // 450 to 0ms
            let off_time = 50 + i * 50; // 50 to 500ms

            led_gpio.set_level(1).expect("设置GPIO高电平失败");
            thread::sleep(Duration::from_millis(on_time));

            led_gpio.set_level(0).expect("设置GPIO低电平失败");
            thread::sleep(Duration::from_millis(off_time));
        }
    }

    println!("呼吸灯效果测试完成，开始无限闪烁...");

    // 无限闪烁，直到程序终止
    loop {
        led_gpio.set_level(1).expect("设置GPIO高电平失败");
        thread::sleep(Duration::from_millis(500));

        led_gpio.set_level(0).expect("设置GPIO低电平失败");
        thread::sleep(Duration::from_millis(500));
    }
}
