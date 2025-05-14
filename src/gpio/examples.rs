/**
 * @file examples.rs
 * @brief ESP32 GPIO 使用示例
 * @details 提供了常见的 GPIO 使用示例和测试代码
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */

#[cfg(test)]
mod tests {
    use super::super::pin::GpioPin;
    use crate::gpio::{GpioInterruptType, GpioMode, GpioPullMode};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_gpio_basic() {
        // 创建一个GPIO2(通常连接到开发板上的LED)的处理实例
        let led_gpio = GpioPin::new(2);

        // 初始化为输出模式，无上拉/下拉，无中断
        led_gpio
            .init(
                GpioMode::Output,
                GpioPullMode::Floating,
                GpioInterruptType::Disable,
            )
            .expect("GPIO初始化失败");

        // LED闪烁5次
        for _ in 0..5 {
            // 打开LED - 使用高级API
            led_gpio.set_high().expect("设置GPIO高电平失败");
            thread::sleep(Duration::from_millis(500));

            // 关闭LED - 使用高级API
            led_gpio.set_low().expect("设置GPIO低电平失败");
            thread::sleep(Duration::from_millis(500));
        }

        // 测试切换功能
        for _ in 0..5 {
            led_gpio.toggle().expect("切换GPIO电平失败");
            thread::sleep(Duration::from_millis(200));
        }

        // 重置GPIO
        led_gpio.reset().expect("重置GPIO失败");
    }

    // 实际使用中的示例：使用GPIO进行按钮检测
    #[test]
    fn test_gpio_button() {
        // 假设按钮连接到GPIO0，使用内部上拉电阻
        let button_gpio = GpioPin::new(0);

        // 初始化为输入模式，上拉，无中断
        button_gpio
            .init(
                GpioMode::Input,
                GpioPullMode::PullUp,
                GpioInterruptType::Disable,
            )
            .expect("GPIO初始化失败");

        // 简单的按钮消抖
        const DEBOUNCE_TIME: u64 = 20; // 20毫秒
        const SAMPLE_COUNT: u32 = 5; // 采样5次

        // 轮询检测按钮状态
        println!("请按下按钮进行测试...");

        let mut last_state = button_gpio.is_high();

        for _ in 0..30 {
            // 循环30次或直到检测到按钮按下
            let current_state = button_gpio.is_high();

            // 如果状态发生变化（可能是按下或释放）
            if current_state != last_state {
                // 消抖：等待一小段时间并多次采样
                thread::sleep(Duration::from_millis(DEBOUNCE_TIME));

                let mut samples = 0;
                for _ in 0..SAMPLE_COUNT {
                    if button_gpio.is_high() == current_state {
                        samples += 1;
                    }
                    thread::sleep(Duration::from_millis(1));
                }

                // 确认状态变化是稳定的
                if samples >= SAMPLE_COUNT - 1 {
                    if !current_state {
                        // 按钮被按下 (GPIO为低电平)
                        println!("按钮被按下!");
                    } else {
                        println!("按钮被释放!");
                    }
                    last_state = current_state;
                }
            }

            thread::sleep(Duration::from_millis(50));
        }

        // 清理
        button_gpio.reset().expect("重置GPIO失败");
    }
}
