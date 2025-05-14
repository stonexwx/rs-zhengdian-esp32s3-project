use esp_idf_sys::{
    gpio_config,
    // GPIO配置相关
    gpio_config_t,
    gpio_deep_sleep_hold_dis,
    gpio_deep_sleep_hold_en,
    gpio_drive_cap_t,
    gpio_get_level,
    gpio_hold_dis,
    // 其他辅助函数
    gpio_hold_en,
    gpio_install_isr_service,
    // GPIO电平和模式类型
    gpio_int_type_t,
    gpio_intr_disable,
    gpio_intr_enable,
    gpio_isr_handler_add,
    gpio_isr_handler_remove,
    gpio_mode_t,
    gpio_num_t,
    gpio_pull_mode_t,
    gpio_pulldown_dis,
    gpio_pulldown_en,
    gpio_pullup_dis,
    gpio_pullup_en,
    gpio_reset_pin,
    gpio_set_direction,
    gpio_set_drive_capability,
    // GPIO中断相关
    gpio_set_intr_type,
    gpio_set_level,
    gpio_set_pull_mode,
    gpio_uninstall_isr_service,
    gpio_wakeup_disable,
    gpio_wakeup_enable,
    ESP_OK,
};
/**
 * @file gpio_handler.rs
 * @brief ESP32 GPIO 处理程序
 * @details 这个模块提供了对 ESP32-S3 GPIO 引脚的全面操作，包括:
 *          - 初始化和配置
 *          - 输入/输出模式设置
 *          - 电平读取和设置
 *          - 中断配置和处理
 *          - 上拉/下拉电阻控制
 *          - 驱动能力配置
 *          - 睡眠模式配置
 *          - 毛刺过滤器功能
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
use std::ffi::c_void;

/// GPIO操作错误类型
#[derive(Debug)]
pub enum GpioError {
    /// 配置错误
    ConfigError,
    /// 无效的GPIO编号
    InvalidGpio,
    /// 中断设置错误
    InterruptError,
    /// 系统错误
    SystemError,
}

/// GPIO操作结果类型
pub type GpioResult<T> = Result<T, GpioError>;

/// GPIO引脚模式
#[derive(Debug, Clone, Copy)]
pub enum GpioMode {
    /// 禁用(既不是输入也不是输出)
    Disable,
    /// 输入模式
    Input,
    /// 输出模式
    Output,
    /// 开漏输出模式
    OutputOpenDrain,
    /// 输入和输出模式
    InputOutput,
    /// 开漏输入和输出模式
    InputOutputOpenDrain,
}

/// GPIO上拉/下拉模式
#[derive(Debug, Clone, Copy)]
pub enum GpioPullMode {
    /// 仅上拉
    PullUp,
    /// 仅下拉
    PullDown,
    /// 同时上拉下拉
    PullUpDown,
    /// 浮空(无上拉下拉)
    Floating,
}

/// GPIO中断类型
#[derive(Debug, Clone, Copy)]
pub enum GpioInterruptType {
    /// 禁用中断
    Disable,
    /// 上升沿触发
    RisingEdge,
    /// 下降沿触发
    FallingEdge,
    /// 任意边沿触发
    AnyEdge,
    /// 低电平触发
    LowLevel,
    /// 高电平触发
    HighLevel,
}

/// GPIO驱动能力
#[derive(Debug, Clone, Copy)]
pub enum GpioDriveCap {
    /// 弱驱动
    Weak,
    /// 较强驱动
    Stronger,
    /// 中等驱动
    Medium,
    /// 最强驱动
    Strongest,
}

/// GPIO处理结构体
pub struct GpioHandler {
    gpio_num: gpio_num_t,
}

impl GpioHandler {
    /// 创建一个新的GPIO处理实例
    ///
    /// # 参数
    ///
    /// * `pin` - GPIO引脚编号
    ///
    /// # 返回
    ///
    /// 返回一个新的GPIO处理实例
    pub fn new(pin: u32) -> Self {
        GpioHandler {
            gpio_num: pin as gpio_num_t,
        }
    }

    /// 初始化GPIO引脚
    ///
    /// # 参数
    ///
    /// * `mode` - GPIO模式
    /// * `pull_mode` - 上拉/下拉模式
    /// * `intr_type` - 中断类型
    ///
    /// # 返回
    ///
    /// 成功返回Ok(())，失败返回Err(GpioError)
    pub fn init(
        &self,
        mode: GpioMode,
        pull_mode: GpioPullMode,
        intr_type: GpioInterruptType,
    ) -> GpioResult<()> {
        let mut config = gpio_config_t {
            pin_bit_mask: 1 << self.gpio_num,
            mode: self.convert_mode(mode),
            pull_up_en: match pull_mode {
                GpioPullMode::PullUp | GpioPullMode::PullUpDown => 1,
                _ => 0,
            },
            pull_down_en: match pull_mode {
                GpioPullMode::PullDown | GpioPullMode::PullUpDown => 1,
                _ => 0,
            },
            intr_type: self.convert_intr_type(intr_type),
        };

        unsafe {
            if gpio_config(&mut config) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }

        Ok(())
    }

    /// 重置GPIO引脚到默认状态
    pub fn reset(&self) -> GpioResult<()> {
        unsafe {
            if gpio_reset_pin(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 设置GPIO方向模式
    pub fn set_direction(&self, mode: GpioMode) -> GpioResult<()> {
        unsafe {
            if gpio_set_direction(self.gpio_num, self.convert_mode(mode)) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 设置GPIO输出电平
    ///
    /// # 参数
    ///
    /// * `level` - 电平值(0或1)
    pub fn set_level(&self, level: u32) -> GpioResult<()> {
        unsafe {
            if gpio_set_level(self.gpio_num, level) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 获取GPIO输入电平
    ///
    /// # 返回
    ///
    /// 返回GPIO电平(0或1)
    pub fn get_level(&self) -> u32 {
        unsafe { gpio_get_level(self.gpio_num) as u32 }
    }

    /// 设置上拉/下拉模式
    pub fn set_pull_mode(&self, pull_mode: GpioPullMode) -> GpioResult<()> {
        unsafe {
            if gpio_set_pull_mode(self.gpio_num, self.convert_pull_mode(pull_mode)) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 启用上拉电阻
    pub fn enable_pullup(&self) -> GpioResult<()> {
        unsafe {
            if gpio_pullup_en(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 禁用上拉电阻
    pub fn disable_pullup(&self) -> GpioResult<()> {
        unsafe {
            if gpio_pullup_dis(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 启用下拉电阻
    pub fn enable_pulldown(&self) -> GpioResult<()> {
        unsafe {
            if gpio_pulldown_en(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 禁用下拉电阻
    pub fn disable_pulldown(&self) -> GpioResult<()> {
        unsafe {
            if gpio_pulldown_dis(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 设置驱动能力
    pub fn set_drive_capability(&self, drive_cap: GpioDriveCap) -> GpioResult<()> {
        unsafe {
            if gpio_set_drive_capability(self.gpio_num, self.convert_drive_cap(drive_cap)) != ESP_OK
            {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 设置中断类型
    pub fn set_interrupt_type(&self, intr_type: GpioInterruptType) -> GpioResult<()> {
        unsafe {
            if gpio_set_intr_type(self.gpio_num, self.convert_intr_type(intr_type)) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }

    /// 启用中断
    pub fn enable_interrupt(&self) -> GpioResult<()> {
        unsafe {
            if gpio_intr_enable(self.gpio_num) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }

    /// 禁用中断
    pub fn disable_interrupt(&self) -> GpioResult<()> {
        unsafe {
            if gpio_intr_disable(self.gpio_num) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }

    /// 启用GPIO保持功能
    ///
    /// 在深度睡眠或复位时保持GPIO状态
    pub fn enable_hold(&self) -> GpioResult<()> {
        unsafe {
            if gpio_hold_en(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 禁用GPIO保持功能
    pub fn disable_hold(&self) -> GpioResult<()> {
        unsafe {
            if gpio_hold_dis(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 启用GPIO唤醒功能
    pub fn enable_wakeup(&self, intr_type: GpioInterruptType) -> GpioResult<()> {
        unsafe {
            if gpio_wakeup_enable(self.gpio_num, self.convert_intr_type(intr_type)) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    /// 禁用GPIO唤醒功能
    pub fn disable_wakeup(&self) -> GpioResult<()> {
        unsafe {
            if gpio_wakeup_disable(self.gpio_num) != ESP_OK {
                return Err(GpioError::ConfigError);
            }
        }
        Ok(())
    }

    // 辅助方法 - 转换GPIO模式
    fn convert_mode(&self, mode: GpioMode) -> gpio_mode_t {
        match mode {
            GpioMode::Disable => 0,              // GPIO_MODE_DISABLE
            GpioMode::Input => 1,                // GPIO_MODE_INPUT
            GpioMode::Output => 2,               // GPIO_MODE_OUTPUT
            GpioMode::OutputOpenDrain => 3,      // GPIO_MODE_OUTPUT_OD
            GpioMode::InputOutput => 4,          // GPIO_MODE_INPUT_OUTPUT
            GpioMode::InputOutputOpenDrain => 5, // GPIO_MODE_INPUT_OUTPUT_OD
        }
    }

    // 辅助方法 - 转换上拉/下拉模式
    fn convert_pull_mode(&self, pull_mode: GpioPullMode) -> gpio_pull_mode_t {
        match pull_mode {
            GpioPullMode::PullUp => 1,     // GPIO_PULLUP_ONLY
            GpioPullMode::PullDown => 2,   // GPIO_PULLDOWN_ONLY
            GpioPullMode::PullUpDown => 3, // GPIO_PULLUP_PULLDOWN
            GpioPullMode::Floating => 0,   // GPIO_FLOATING
        }
    }

    // 辅助方法 - 转换中断类型
    fn convert_intr_type(&self, intr_type: GpioInterruptType) -> gpio_int_type_t {
        match intr_type {
            GpioInterruptType::Disable => 0,     // GPIO_INTR_DISABLE
            GpioInterruptType::RisingEdge => 2,  // GPIO_INTR_POSEDGE
            GpioInterruptType::FallingEdge => 3, // GPIO_INTR_NEGEDGE
            GpioInterruptType::AnyEdge => 4,     // GPIO_INTR_ANYEDGE
            GpioInterruptType::LowLevel => 1,    // GPIO_INTR_LOW_LEVEL
            GpioInterruptType::HighLevel => 5,   // GPIO_INTR_HIGH_LEVEL
        }
    }

    // 辅助方法 - 转换驱动能力
    fn convert_drive_cap(&self, drive_cap: GpioDriveCap) -> gpio_drive_cap_t {
        match drive_cap {
            GpioDriveCap::Weak => 0,
            GpioDriveCap::Stronger => 1,
            GpioDriveCap::Medium => 2,
            GpioDriveCap::Strongest => 3,
        }
    }
}

/// GPIO模块静态方法
pub struct GpioControl;

impl GpioControl {
    /// 安装GPIO中断服务
    ///
    /// # 参数
    ///
    /// * `intr_alloc_flags` - 中断分配标志
    pub fn install_isr_service(intr_alloc_flags: i32) -> GpioResult<()> {
        unsafe {
            if gpio_install_isr_service(intr_alloc_flags) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }

    /// 卸载GPIO中断服务
    pub fn uninstall_isr_service() {
        unsafe {
            gpio_uninstall_isr_service();
        }
    }

    /// 为指定的GPIO添加ISR处理程序
    ///
    /// # 参数
    ///
    /// * `gpio_num` - GPIO编号
    /// * `isr_handler` - 中断处理函数
    /// * `args` - 传递给处理函数的参数
    pub fn add_isr_handler(
        gpio_num: u32,
        isr_handler: Option<unsafe extern "C" fn(arg: *mut c_void)>,
        args: *mut c_void,
    ) -> GpioResult<()> {
        unsafe {
            if gpio_isr_handler_add(gpio_num as gpio_num_t, isr_handler, args) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }

    /// 移除指定GPIO的ISR处理程序
    ///
    /// # 参数
    ///
    /// * `gpio_num` - GPIO编号
    pub fn remove_isr_handler(gpio_num: u32) -> GpioResult<()> {
        unsafe {
            if gpio_isr_handler_remove(gpio_num as gpio_num_t) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }

    /// 启用所有数字GPIO引脚在深度睡眠期间的保持功能
    pub fn enable_deep_sleep_hold() {
        unsafe {
            gpio_deep_sleep_hold_en();
        }
    }

    /// 禁用所有数字GPIO引脚在深度睡眠期间的保持功能
    pub fn disable_deep_sleep_hold() {
        unsafe {
            gpio_deep_sleep_hold_dis();
        }
    }
}

/// GPIO模块的用法示例
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_gpio_basic() {
        // 创建一个GPIO2(通常连接到开发板上的LED)的处理实例
        let led_gpio = GpioHandler::new(2);

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
            // 打开LED
            led_gpio.set_level(1).expect("设置GPIO电平失败");
            thread::sleep(Duration::from_millis(500));

            // 关闭LED
            led_gpio.set_level(0).expect("设置GPIO电平失败");
            thread::sleep(Duration::from_millis(500));
        }

        // 重置GPIO
        led_gpio.reset().expect("重置GPIO失败");
    }

    // 注意：中断测试在实际项目中需要考虑硬件连接和测试环境
}
