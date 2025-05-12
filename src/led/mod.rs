use esp_idf_svc::sys::{
    gpio_config, gpio_config_t, gpio_dump_io_configuration, gpio_int_type_t_GPIO_INTR_DISABLE,
    gpio_mode_t_GPIO_MODE_OUTPUT, gpio_num_t, gpio_pulldown_t_GPIO_PULLDOWN_DISABLE,
    gpio_pullup_t_GPIO_PULLUP_DISABLE, gpio_set_level, FILE,
};
pub struct Led {
    pin: gpio_num_t,
    is_on: bool,
}

impl Led {
    /// 创建新的LED控制实例
    pub fn new(pin: gpio_num_t) -> Self {
        let mut led = Led { pin, is_on: false };
        led.init();
        led
    }

    /// 初始化GPIO引脚
    fn init(&mut self) {
        unsafe {
            let pin_bit_mask: u64 = 1 << self.pin;

            // 准备GPIO配置
            let io_conf = gpio_config_t {
                pin_bit_mask,
                mode: gpio_mode_t_GPIO_MODE_OUTPUT, // 设置为输出模式
                pull_up_en: gpio_pullup_t_GPIO_PULLUP_DISABLE,
                pull_down_en: gpio_pulldown_t_GPIO_PULLDOWN_DISABLE,
                intr_type: gpio_int_type_t_GPIO_INTR_DISABLE,
            };

            // 配置GPIO
            gpio_config(&io_conf);

            // 初始化为关闭状态
            gpio_set_level(self.pin, 0);
        }
    }

    /// 打开LED
    pub fn on(&mut self) {
        unsafe {
            gpio_set_level(self.pin, 1);
        }
        self.is_on = true;
        println!("LED on pin {} is ON", self.pin);
        log::info!("GPIO {} configuration: mode=OUTPUT", self.pin);
    }

    /// 关闭LED
    pub fn off(&mut self) {
        unsafe {
            gpio_set_level(self.pin, 0);
        }
        self.is_on = false;
        println!("LED on pin {} is OFF", self.pin);
    }

    /// 切换LED状态
    pub fn toggle(&mut self) {
        if self.is_on {
            self.off();
        } else {
            self.on();
        }
    }

    /// 获取LED当前状态
    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

impl Drop for Led {
    fn drop(&mut self) {
        // 确保在删除对象时LED被关闭
        if self.is_on {
            self.off();
        }
    }
}
