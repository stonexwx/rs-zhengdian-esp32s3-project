// filepath: /Volumes/code/rust_project/esp32-test/src/drivers/spi/mod.rs

mod types;
mod controller;

pub use types::*;
pub use controller::*;

/// 导出SPI相关的接口和类型
pub mod prelude {
    pub use super::types::*;
    pub use super::controller::*;
}
