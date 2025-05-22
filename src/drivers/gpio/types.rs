/**
 * @file types.rs
 * @brief ESP32 GPIO 类型定义
 * @details 包含 GPIO 操作相关的枚举、错误类型等定义
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
use esp_idf_sys::{
    gpio_drive_cap_t, gpio_int_type_t, gpio_mode_t, gpio_pull_mode_t,
};

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

/// 类型转换辅助函数
#[inline]
pub(crate) fn convert_mode(mode: GpioMode) -> gpio_mode_t {
    match mode {
        GpioMode::Disable => 0,      // GPIO_MODE_DISABLE
        GpioMode::Input => 1,        // GPIO_MODE_INPUT
        GpioMode::Output => 2,       // GPIO_MODE_OUTPUT
        GpioMode::OutputOpenDrain => 3, // GPIO_MODE_OUTPUT_OD
        GpioMode::InputOutput => 4,  // GPIO_MODE_INPUT_OUTPUT
        GpioMode::InputOutputOpenDrain => 5, // GPIO_MODE_INPUT_OUTPUT_OD
    }
}

/// 类型转换辅助函数
#[inline]
pub(crate) fn convert_pull_mode(mode: GpioPullMode) -> gpio_pull_mode_t {
    match mode {
        GpioPullMode::PullUp => 1,     // GPIO_PULLUP_ONLY
        GpioPullMode::PullDown => 2,   // GPIO_PULLDOWN_ONLY
        GpioPullMode::PullUpDown => 3, // GPIO_PULLUP_PULLDOWN
        GpioPullMode::Floating => 0,   // GPIO_FLOATING
    }
}

/// 类型转换辅助函数
#[inline]
pub(crate) fn convert_intr_type(intr_type: GpioInterruptType) -> gpio_int_type_t {
    match intr_type {
        GpioInterruptType::Disable => 0,     // GPIO_INTR_DISABLE
        GpioInterruptType::RisingEdge => 2,  // GPIO_INTR_POSEDGE
        GpioInterruptType::FallingEdge => 3, // GPIO_INTR_NEGEDGE
        GpioInterruptType::AnyEdge => 4,     // GPIO_INTR_ANYEDGE
        GpioInterruptType::LowLevel => 1,    // GPIO_INTR_LOW_LEVEL
        GpioInterruptType::HighLevel => 5,   // GPIO_INTR_HIGH_LEVEL
    }
}

/// 类型转换辅助函数
#[inline]
pub(crate) fn convert_drive_cap(cap: GpioDriveCap) -> gpio_drive_cap_t {
    match cap {
        GpioDriveCap::Weak => 0,      // GPIO_DRIVE_CAP_0
        GpioDriveCap::Stronger => 1,  // GPIO_DRIVE_CAP_1
        GpioDriveCap::Medium => 2,    // GPIO_DRIVE_CAP_2
        GpioDriveCap::Strongest => 3, // GPIO_DRIVE_CAP_3
    }
}
