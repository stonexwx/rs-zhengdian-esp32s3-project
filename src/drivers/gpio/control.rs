/**
 * @file control.rs
 * @brief ESP32 GPIO 系统控制功能
 * @details 提供了 GPIO 系统级别的控制功能，如深度睡眠、电源管理等
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
use esp_idf_sys::{gpio_deep_sleep_hold_dis, gpio_deep_sleep_hold_en};

/// GPIO系统控制
pub struct GpioControl;

impl GpioControl {
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

// 可以在此处添加更多系统级的GPIO控制功能，如：
// - 毛刺过滤器配置
// - GPIO矩阵配置
// - 电源管理相关的控制
// - IO_MUX控制等
