use esp32_test::gpio::{GpioInterrupt, GpioInterruptType, GpioMode, GpioPin, GpioPullMode};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
/**
 * @file gpio_interrupt_test.rs
 * @brief 使用重构后的GPIO模块和中断功能的示例
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
use std::thread;
use std::time::Duration;

// 假设按钮连接到GPIO0（通常是BOOT按钮）
const BUTTON_GPIO_PIN: u32 = 0;
// LED连接到GPIO48
const LED_GPIO_PIN: u32 = 48;

// 使用静态变量来跟踪中断事件
static mut INTERRUPT_COUNT: u32 = 0;
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

// GPIO中断处理函数
unsafe extern "C" fn button_isr_handler(_arg: *mut c_void) {
    // 在中断中设置标志，避免在ISR中执行过多操作
    BUTTON_PRESSED.store(true, Ordering::SeqCst);
    INTERRUPT_COUNT += 1;
}

fn main() {
    // 初始化ESP-IDF
    esp_idf_svc::sys::link_patches();
    // 初始化日志
    esp_idf_svc::log::EspLogger::initialize_default();

    println!("GPIO中断测试示例开始运行!");

    // 安装GPIO中断服务
    GpioInterrupt::install_service(0).expect("安装GPIO中断服务失败");

    // 创建按钮GPIO实例
    let button_gpio = GpioPin::new(BUTTON_GPIO_PIN);

    // 初始化按钮GPIO为输入模式，上拉，下降沿触发中断
    button_gpio
        .init(
            GpioMode::Input,
            GpioPullMode::PullUp,           // 启用上拉电阻，按钮按下时接地
            GpioInterruptType::FallingEdge, // 下降沿触发中断（按下按钮时）
        )
        .expect("按钮GPIO初始化失败");

    // 创建LED GPIO实例
    let led_gpio = GpioPin::new(LED_GPIO_PIN);

    // 初始化LED GPIO为输出模式
    led_gpio
        .init(
            GpioMode::Output,
            GpioPullMode::Floating,
            GpioInterruptType::Disable,
        )
        .expect("LED GPIO初始化失败");

    // 启用按钮中断并注册中断处理函数
    button_gpio.enable_interrupt().expect("启用中断失败");

    // 为按钮GPIO添加中断处理程序
    GpioInterrupt::add_handler(
        BUTTON_GPIO_PIN,
        Some(button_isr_handler),
        std::ptr::null_mut(),
    )
    .expect("添加中断处理程序失败");

    println!(
        "中断已配置。按下按钮（GPIO{})来触发LED（GPIO{})切换。",
        BUTTON_GPIO_PIN, LED_GPIO_PIN
    );

    // 主循环
    let mut led_state = false;
    loop {
        // 检查中断标志
        if BUTTON_PRESSED.load(Ordering::SeqCst) {
            // 复位中断标志
            BUTTON_PRESSED.store(false, Ordering::SeqCst);

            // 切换LED状态
            led_state = !led_state;
            if led_state {
                led_gpio.set_high().expect("设置LED高电平失败");
                println!("LED开启");
            } else {
                led_gpio.set_low().expect("设置LED低电平失败");
                println!("LED关闭");
            }

            // 显示中断计数
            unsafe {
                println!("检测到按钮中断 #{}", INTERRUPT_COUNT);
            }

            // 简单的消抖延迟
            thread::sleep(Duration::from_millis(200));
        }

        // 小延迟避免CPU过度使用
        thread::sleep(Duration::from_millis(10));
    }

    // 注意：此代码实际上永远不会执行，因为上面的循环是无限的
    // 但为了完整性，添加清理代码
    // GpioInterrupt::remove_handler(BUTTON_GPIO_PIN).expect("移除中断处理程序失败");
    // GpioInterrupt::uninstall_service();
}
