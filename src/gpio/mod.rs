pub mod control; // GPIO系统控制功能
pub mod examples;
/**
 * @file mod.rs
 * @brief GPIO模块导出文件
 * @details 导出GPIO处理模块的所有公共接口
 * @author xwx
 * @date 2025-05-13
 * @version 1.0
 */
// GPIO模块按功能拆分为多个子模块
// 保留旧模块用于兼容性（可以在迁移完成后移除）
pub mod gpio_handler;
pub mod interrupt; // GPIO中断处理
pub mod pin; // GPIO引脚基本操作
pub mod types;

// 重新导出常用的类型和结构体，使它们可以直接从gpio模块访问
pub use gpio_handler::{
    GpioDriveCap, GpioError, GpioInterruptType, GpioMode, GpioPullMode, GpioResult,
};

pub use control::GpioControl;
pub use interrupt::{GpioInterrupt, GpioIsr, InterruptArg};
pub use pin::GpioPin;

// 为向后兼容，提供别名
pub use pin::GpioPin as GpioHandler;
