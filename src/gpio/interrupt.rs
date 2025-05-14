use esp_idf_sys::{
    gpio_install_isr_service, gpio_isr_handler_add, gpio_isr_handler_remove, gpio_num_t,
    gpio_uninstall_isr_service, ESP_OK,
};
/**
 * @file interrupt.rs
 * @brief ESP32 GPIO 中断处理
 * @details 提供了 GPIO 中断相关的配置和处理功能
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
use std::ffi::c_void;

use crate::gpio::{GpioError, GpioResult};

/// GPIO中断处理器
pub struct GpioInterrupt;

impl GpioInterrupt {
    /// 安装GPIO中断服务
    ///
    /// # 参数
    ///
    /// * `intr_alloc_flags` - 中断分配标志
    pub fn install_service(intr_alloc_flags: i32) -> GpioResult<()> {
        unsafe {
            if gpio_install_isr_service(intr_alloc_flags) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }

    /// 卸载GPIO中断服务
    pub fn uninstall_service() {
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
    pub fn add_handler(
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
    pub fn remove_handler(gpio_num: u32) -> GpioResult<()> {
        unsafe {
            if gpio_isr_handler_remove(gpio_num as gpio_num_t) != ESP_OK {
                return Err(GpioError::InterruptError);
            }
        }
        Ok(())
    }
}

// 中断函数类型定义，方便使用
pub type GpioIsr = Option<unsafe extern "C" fn(arg: *mut c_void)>;

/// 中断参数包装器，用于在中断处理中保存和传递数据
pub struct InterruptArg<T> {
    data: Box<T>,
}

impl<T> InterruptArg<T> {
    /// 创建一个新的中断参数包装器
    pub fn new(data: T) -> Self {
        InterruptArg {
            data: Box::new(data),
        }
    }

    /// 获取包装器内部数据的裸指针，用于传递给中断处理函数
    pub fn as_ptr(&self) -> *mut c_void {
        Box::into_raw(Box::new(&*self.data)) as *mut c_void
    }

    /// 从裸指针中恢复数据引用
    ///
    /// # 安全性
    ///
    /// 此函数不安全，因为它需要确保指针是有效的并且来自于`as_ptr`方法
    pub unsafe fn from_ptr<'a>(ptr: *mut c_void) -> &'a T {
        let boxed = Box::from_raw(ptr as *mut &T);
        &**boxed
    }
}
