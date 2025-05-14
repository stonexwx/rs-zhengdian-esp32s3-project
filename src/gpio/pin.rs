/**
 * @file pin.rs
 * @brief ESP32 GPIO 引脚基本操作
 * @details 提供了 GPIO 引脚的基本操作，如初始化、设置/获取电平、配置模式等
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
use esp_idf_sys::{
    gpio_config, gpio_config_t, gpio_get_level, gpio_hold_dis, gpio_hold_en, gpio_intr_disable,
    gpio_intr_enable, gpio_num_t, gpio_pulldown_dis, gpio_pulldown_en, gpio_pullup_dis,
    gpio_pullup_en, gpio_reset_pin, gpio_set_direction, gpio_set_drive_capability,
    gpio_set_intr_type, gpio_set_level, gpio_set_pull_mode, gpio_wakeup_disable,
    gpio_wakeup_enable, ESP_OK,
};

use crate::gpio::gpio_handler::{
    GpioDriveCap, GpioError, GpioInterruptType, GpioMode, GpioPullMode, GpioResult,
};

// 类型转换函数
fn convert_mode(mode: GpioMode) -> u32 {
    match mode {
        GpioMode::Disable => 0,      // GPIO_MODE_DISABLE
        GpioMode::Input => 1,        // GPIO_MODE_INPUT
        GpioMode::Output => 2,       // GPIO_MODE_OUTPUT
        GpioMode::OutputOpenDrain => 3, // GPIO_MODE_OUTPUT_OD
        GpioMode::InputOutput => 4,  // GPIO_MODE_INPUT_OUTPUT
        GpioMode::InputOutputOpenDrain => 5, // GPIO_MODE_INPUT_OUTPUT_OD
    }
}

fn convert_pull_mode(mode: GpioPullMode) -> u32 {
    match mode {
        GpioPullMode::PullUp => 1,     // GPIO_PULLUP_ONLY
        GpioPullMode::PullDown => 2,   // GPIO_PULLDOWN_ONLY
        GpioPullMode::PullUpDown => 3, // GPIO_PULLUP_PULLDOWN
        GpioPullMode::Floating => 0,   // GPIO_FLOATING
    }
}

fn convert_intr_type(intr_type: GpioInterruptType) -> u32 {
    match intr_type {
        GpioInterruptType::Disable => 0,     // GPIO_INTR_DISABLE
        GpioInterruptType::RisingEdge => 2,  // GPIO_INTR_POSEDGE
        GpioInterruptType::FallingEdge => 3, // GPIO_INTR_NEGEDGE
        GpioInterruptType::AnyEdge => 4,     // GPIO_INTR_ANYEDGE
        GpioInterruptType::LowLevel => 1,    // GPIO_INTR_LOW_LEVEL
        GpioInterruptType::HighLevel => 5,   // GPIO_INTR_HIGH_LEVEL
    }
}

fn convert_drive_cap(cap: GpioDriveCap) -> u32 {
    match cap {
        GpioDriveCap::Weak => 0,      // GPIO_DRIVE_CAP_0
        GpioDriveCap::Stronger => 1,  // GPIO_DRIVE_CAP_1
        GpioDriveCap::Medium => 2,    // GPIO_DRIVE_CAP_2
        GpioDriveCap::Strongest => 3, // GPIO_DRIVE_CAP_3
    }
}

/// GPIO引脚处理结构体
pub struct GpioPin {
    gpio_num: gpio_num_t,
}

impl GpioPin {
    /// 创建一个新的GPIO引脚处理实例
    ///
    /// # 参数
    ///
    /// * `pin` - GPIO引脚编号
    ///
    /// # 返回
    ///
    /// 返回一个新的GPIO引脚处理实例
    pub fn new(pin: u32) -> Self {
        GpioPin {
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
            mode: convert_mode(mode),
            pull_up_en: match pull_mode {
                GpioPullMode::PullUp | GpioPullMode::PullUpDown => 1,
                _ => 0,
            },
            pull_down_en: match pull_mode {
                GpioPullMode::PullDown | GpioPullMode::PullUpDown => 1,
                _ => 0,
            },
            intr_type: convert_intr_type(intr_type),
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
            if gpio_set_direction(self.gpio_num, convert_mode(mode)) != ESP_OK {
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
            if gpio_set_pull_mode(self.gpio_num, convert_pull_mode(pull_mode)) != ESP_OK {
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
            if gpio_set_drive_capability(self.gpio_num, convert_drive_cap(drive_cap)) != ESP_OK {
                return Err(GpioError::ConfigError);
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
            if gpio_wakeup_enable(self.gpio_num, convert_intr_type(intr_type)) != ESP_OK {
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

    /// 设置中断类型
    pub fn set_interrupt_type(&self, intr_type: GpioInterruptType) -> GpioResult<()> {
        unsafe {
            if gpio_set_intr_type(self.gpio_num, convert_intr_type(intr_type)) != ESP_OK {
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

    /// 获取GPIO编号
    pub fn get_pin_number(&self) -> gpio_num_t {
        self.gpio_num
    }
}

/// 实现便捷的高/低电平切换方法
impl GpioPin {
    /// 设置为高电平
    pub fn set_high(&self) -> GpioResult<()> {
        self.set_level(1)
    }

    /// 设置为低电平
    pub fn set_low(&self) -> GpioResult<()> {
        self.set_level(0)
    }

    /// 切换电平状态（高变低，低变高）
    pub fn toggle(&self) -> GpioResult<()> {
        let current_level = self.get_level();
        self.set_level(1 - current_level)
    }

    /// 检查是否为高电平
    pub fn is_high(&self) -> bool {
        self.get_level() == 1
    }

    /// 检查是否为低电平
    pub fn is_low(&self) -> bool {
        self.get_level() == 0
    }
}
