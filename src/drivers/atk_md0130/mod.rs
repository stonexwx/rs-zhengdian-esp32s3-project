mod lcd;
mod r#type;

pub use lcd::*;
pub use r#type::*;

/// 创建并初始化ATK-MD0130 LCD实例的辅助函数
pub fn create_atk_md0130(
    mosi_pin: i32,
    miso_pin: i32,
    sclk_pin: i32,
    cs_pin: i32,
    dc_pin_num: i32,
    rst_pin_num: i32,
    bl_pin_num: Option<i32>,
) -> Result<ATKMD0130, Box<dyn std::error::Error>> {
    use crate::drivers::gpio::GpioPin;
    use crate::drivers::spi::{SpiBitOrder, SpiBus, SpiDeviceConfig, SpiMaster, SpiMode};

    // 1. 初始化SPI主机
    let mut spi_master =
        SpiMaster::new(SpiBus::Spi2).map_err(|e| format!("Failed to create SpiMaster: {:?}", e))?;
    spi_master
        .initialize(mosi_pin, miso_pin, sclk_pin, 0)
        .map_err(|e| format!("Failed to initialize SpiMaster: {:?}", e))?;

    // 2. 配置SPI设备
    let spi_config = SpiDeviceConfig {
        mode: SpiMode::Mode0,
        clock_speed_hz: 60_000_000,
        cs_pin: Some(cs_pin),
        command_bits: 0,
        address_bits: 0,
        bit_order: SpiBitOrder::MSBFirst,
        queue_size: 7,
    };
    let spi_device = spi_master
        .add_device(&spi_config)
        .map_err(|e| format!("Failed to add SPI device: {:?}", e))?;

    // 3. 初始化GPIO引脚
    let dc_pin = GpioPin::new(dc_pin_num as u32);
    let rst_pin = GpioPin::new(rst_pin_num as u32);
    let bl_pin = bl_pin_num.map(|pin| GpioPin::new(pin as u32));

    // 4. 创建LCD实例
    ATKMD0130::new(spi_master, spi_device, rst_pin, dc_pin, bl_pin).map_err(|e| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        )) as Box<dyn std::error::Error>
    })
}

// 重新导出模块
pub mod prelude {
    pub use super::lcd::*;
    pub use super::r#type::*;
}
