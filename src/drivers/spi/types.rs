/// SPI模式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiMode {
    /// 时钟空闲时为低电平，在第一个时钟边沿采样数据
    Mode0,
    /// 时钟空闲时为低电平，在第二个时钟边沿采样数据
    Mode1,
    /// 时钟空闲时为高电平，在第一个时钟边沿采样数据
    Mode2,
    /// 时钟空闲时为高电平，在第二个时钟边沿采样数据
    Mode3,
}

impl From<SpiMode> for u32 {
    fn from(mode: SpiMode) -> Self {
        match mode {
            SpiMode::Mode0 => 0,
            SpiMode::Mode1 => 1,
            SpiMode::Mode2 => 2,
            SpiMode::Mode3 => 3,
        }
    }
}

/// SPI数据传输位序
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiBitOrder {
    /// 最高有效位优先
    MSBFirst,
    /// 最低有效位优先
    LSBFirst,
}

impl From<SpiBitOrder> for bool {
    fn from(order: SpiBitOrder) -> Self {
        match order {
            SpiBitOrder::MSBFirst => false,
            SpiBitOrder::LSBFirst => true,
        }
    }
}

/// SPI传输错误类型
#[derive(Debug)]
pub enum SpiError {
    /// 参数错误
    InvalidParameter,
    /// 驱动程序错误
    DriverError(i32),
    /// 总线被占用
    BusBusy,
    /// 超时错误
    Timeout,
}

/// SPI传输结果类型
pub type SpiResult<T> = Result<T, SpiError>;

/// SPI设备配置
#[derive(Debug, Clone)]
pub struct SpiDeviceConfig {
    /// 时钟频率（Hz）
    pub clock_speed_hz: u32,
    /// SPI模式
    pub mode: SpiMode,
    /// SPI位序
    pub bit_order: SpiBitOrder,
    /// 命令长度（位）
    pub command_bits: u8,
    /// 地址长度（位）
    pub address_bits: u8,
    /// 片选引脚编号
    pub cs_pin: Option<i32>,
    /// 队列大小
    pub queue_size: usize,
}

impl Default for SpiDeviceConfig {
    fn default() -> Self {
        Self {
            clock_speed_hz: 1_000_000, // 默认1MHz
            mode: SpiMode::Mode0,
            bit_order: SpiBitOrder::MSBFirst,
            command_bits: 0,
            address_bits: 0,
            cs_pin: None,
            queue_size: 1,
        }
    }
}
