// SPI控制器实现
use crate::drivers::spi::types::*;
use esp_idf_svc::sys;
use std::ptr;
use std::vec::Vec;

/// SPI主机总线编号
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiBus {
    /// SPI1，通常用于Flash访问
    Spi1 = 1,
    /// SPI2，通常可用于用户应用
    Spi2 = 2,
    /// SPI3，通常可用于用户应用 (ESP32-S3有SPI3)
    Spi3 = 3,
}

/// SPI设备句柄结构体
pub struct SpiDevice {
    handle: sys::spi_device_handle_t,
}

/// SPI主机控制器
pub struct SpiMaster {
    host: SpiBus,
    initialized: bool,
    devices: Vec<sys::spi_device_handle_t>, // 跟踪添加到此总线的所有SPI设备
}

impl SpiMaster {
    /// 创建新的SPI主机控制器
    ///
    /// # 参数
    /// * `host` - SPI主机总线
    ///
    /// # 返回
    /// * `SpiResult<Self>` - SPI主机控制器实例
    pub fn new(host: SpiBus) -> SpiResult<Self> {
        let spi = SpiMaster {
            host,
            initialized: false,
            devices: Vec::new(),
        };
        Ok(spi)
    }

    /// 初始化SPI总线
    ///
    /// # 参数
    /// * `mosi_pin` - MOSI引脚编号
    /// * `miso_pin` - MISO引脚编号
    /// * `sclk_pin` - SCLK引脚编号
    /// * `max_transfer_size` - 最大传输大小，0表示默认值
    ///
    /// # 返回
    /// * `SpiResult<()>` - 成功返回Ok(())，失败返回错误
    pub fn initialize(
        &mut self,
        mosi_pin: i32,
        miso_pin: i32,
        sclk_pin: i32,
        max_transfer_size: usize,
    ) -> SpiResult<()> {
        if self.initialized {
            return Ok(());
        }

        // SPI总线配置
        let mut bus_config = sys::spi_bus_config_t::default();
        // 设置MOSI引脚
        bus_config.__bindgen_anon_1.mosi_io_num = mosi_pin;
        // 设置MISO引脚
        bus_config.__bindgen_anon_2.miso_io_num = miso_pin;
        // 设置时钟引脚
        bus_config.sclk_io_num = sclk_pin;
        // 设置WP和HD引脚 (不使用)
        bus_config.__bindgen_anon_3.quadwp_io_num = -1;
        bus_config.__bindgen_anon_4.quadhd_io_num = -1;
        // 设置其他引脚 (不使用)
        bus_config.data4_io_num = -1;
        bus_config.data5_io_num = -1;
        bus_config.data6_io_num = -1;
        bus_config.data7_io_num = -1;
        // 设置最大传输大小
        bus_config.max_transfer_sz = if max_transfer_size > 0 {
            max_transfer_size as i32
        } else {
            0
        };
        // 设置其他标志
        bus_config.flags = 0;
        bus_config.isr_cpu_id = 0; // 默认CPU

        // 初始化SPI总线
        let result = unsafe {
            sys::spi_bus_initialize(
                self.host as sys::spi_host_device_t,
                &bus_config,
                sys::spi_common_dma_t_SPI_DMA_CH_AUTO,
            )
        };

        if result != sys::ESP_OK {
            return Err(SpiError::DriverError(result));
        }

        self.initialized = true;
        Ok(())
    }

    /// 添加SPI设备
    ///
    /// # 参数
    /// * `config` - SPI设备配置
    ///
    /// # 返回
    /// * `SpiResult<SpiDevice>` - 成功返回设备句柄，失败返回错误
    pub fn add_device(&mut self, config: &SpiDeviceConfig) -> SpiResult<SpiDevice> {
        if !self.initialized {
            return Err(SpiError::InvalidParameter);
        }

        // SPI设备接口配置
        let device_config = sys::spi_device_interface_config_t {
            command_bits: config.command_bits,
            address_bits: config.address_bits,
            dummy_bits: 0,
            mode: config.mode as u8,
            duty_cycle_pos: 0,
            cs_ena_pretrans: 0,
            cs_ena_posttrans: 0,
            clock_speed_hz: config.clock_speed_hz as i32,
            input_delay_ns: 0,
            spics_io_num: config.cs_pin.unwrap_or(-1),
            flags: if config.bit_order == SpiBitOrder::LSBFirst {
                sys::SPI_DEVICE_BIT_LSBFIRST as u32
            } else {
                0
            },
            queue_size: config.queue_size as i32,
            pre_cb: None,
            post_cb: None,
            clock_source: 0, // Default clock source
        };

        // 添加SPI设备
        let mut handle = ptr::null_mut();
        let result = unsafe {
            sys::spi_bus_add_device(
                self.host as sys::spi_host_device_t,
                &device_config,
                &mut handle,
            )
        };

        if result != sys::ESP_OK {
            return Err(SpiError::DriverError(result));
        }

        // 跟踪设备句柄
        self.devices.push(handle);

        // 返回设备句柄
        Ok(SpiDevice { handle })
    }

    /// 释放SPI总线
    pub fn deinitialize(&mut self) -> SpiResult<()> {
        if !self.initialized {
            return Ok(());
        }

        // 释放所有设备
        for &device in &self.devices {
            unsafe {
                sys::spi_bus_remove_device(device);
            }
        }
        self.devices.clear();

        // 释放SPI总线
        let result = unsafe { sys::spi_bus_free(self.host as sys::spi_host_device_t) };

        if result != sys::ESP_OK {
            return Err(SpiError::DriverError(result));
        }

        self.initialized = false;
        Ok(())
    }
}

impl Drop for SpiMaster {
    fn drop(&mut self) {
        // 尝试释放资源
        let _ = self.deinitialize();
    }
}

impl SpiDevice {
    /// 发送并接收数据
    ///
    /// # 参数
    /// * `tx_data` - 发送数据
    /// * `rx_data` - 接收数据缓冲区
    ///
    /// # 返回
    /// * `SpiResult<()>` - 成功返回Ok(())，失败返回错误
    pub fn transfer(&self, tx_data: &[u8], rx_data: &mut [u8]) -> SpiResult<()> {
        let len = tx_data.len().min(rx_data.len());
        if len == 0 {
            return Err(SpiError::InvalidParameter);
        }

        // 创建SPI事务
        let mut transaction = sys::spi_transaction_t::default();
        transaction.flags = 0;
        transaction.cmd = 0;
        transaction.addr = 0;
        transaction.length = (len * 8) as usize; // 以位为单位
        transaction.rxlength = (len * 8) as usize; // 设置接收长度
        transaction.user = ptr::null_mut();

        // 设置发送和接收缓冲区
        transaction.__bindgen_anon_1.tx_buffer = tx_data.as_ptr() as *const _;
        transaction.__bindgen_anon_2.rx_buffer = rx_data.as_mut_ptr() as *mut _;

        // 执行SPI事务
        let result = unsafe { sys::spi_device_transmit(self.handle, &mut transaction) };

        if result != sys::ESP_OK {
            return Err(SpiError::DriverError(result));
        }

        Ok(())
    }

    /// 只发送数据
    ///
    /// # 参数
    /// * `tx_data` - 发送数据
    ///
    /// # 返回
    /// * `SpiResult<()>` - 成功返回Ok(())，失败返回错误
    pub fn write(&self, tx_data: &[u8]) -> SpiResult<()> {
        if tx_data.is_empty() {
            return Err(SpiError::InvalidParameter);
        }

        // 创建SPI事务
        let mut transaction = sys::spi_transaction_t::default();
        transaction.flags = 0;
        transaction.cmd = 0;
        transaction.addr = 0;
        transaction.length = (tx_data.len() * 8) as usize; // 以位为单位
        transaction.rxlength = 0; // 不需要接收数据
        transaction.user = ptr::null_mut();

        // 设置发送缓冲区
        transaction.__bindgen_anon_1.tx_buffer = tx_data.as_ptr() as *const _;
        transaction.__bindgen_anon_2.rx_buffer = ptr::null_mut();

        // 执行SPI事务
        let result = unsafe { sys::spi_device_transmit(self.handle, &mut transaction) };

        if result != sys::ESP_OK {
            return Err(SpiError::DriverError(result));
        }

        Ok(())
    }

    /// 只接收数据
    ///
    /// # 参数
    /// * `rx_data` - 接收数据缓冲区
    ///
    /// # 返回
    /// * `SpiResult<()>` - 成功返回Ok(())，失败返回错误
    pub fn read(&self, rx_data: &mut [u8]) -> SpiResult<()> {
        if rx_data.is_empty() {
            return Err(SpiError::InvalidParameter);
        }

        // 创建SPI事务，发送全为0的数据
        let mut transaction = sys::spi_transaction_t::default();
        transaction.flags = 0;
        transaction.cmd = 0;
        transaction.addr = 0;
        transaction.length = (rx_data.len() * 8) as usize; // 以位为单位
        transaction.rxlength = (rx_data.len() * 8) as usize; // 接收长度
        transaction.user = ptr::null_mut();

        // 设置发送和接收缓冲区
        transaction.__bindgen_anon_1.tx_buffer = ptr::null(); // 将自动发送0
        transaction.__bindgen_anon_2.rx_buffer = rx_data.as_mut_ptr() as *mut _;

        // 执行SPI事务
        let result = unsafe { sys::spi_device_transmit(self.handle, &mut transaction) };

        if result != sys::ESP_OK {
            return Err(SpiError::DriverError(result));
        }

        Ok(())
    }

    /// 带命令和地址的写数据
    ///
    /// # 参数
    /// * `cmd` - 命令
    /// * `addr` - 地址
    /// * `tx_data` - 发送数据
    ///
    /// # 返回
    /// * `SpiResult<()>` - 成功返回Ok(())，失败返回错误
    pub fn write_with_cmd_addr(&self, cmd: u16, addr: u32, tx_data: &[u8]) -> SpiResult<()> {
        // 创建SPI事务
        let mut transaction = sys::spi_transaction_t::default();
        transaction.flags = if cmd > 0 {
            sys::SPI_TRANS_VARIABLE_CMD as u32
        } else {
            0
        } | if addr > 0 {
            sys::SPI_TRANS_VARIABLE_ADDR as u32
        } else {
            0
        };
        transaction.cmd = cmd;
        transaction.addr = addr as u64; // 注意: addr类型应该是u64
        transaction.length = (tx_data.len() * 8) as usize; // 以位为单位
        transaction.rxlength = 0; // 不需要接收数据
        transaction.user = ptr::null_mut();

        // 设置发送缓冲区
        if !tx_data.is_empty() {
            transaction.__bindgen_anon_1.tx_buffer = tx_data.as_ptr() as *const _;
        } else {
            transaction.__bindgen_anon_1.tx_buffer = ptr::null();
        }
        transaction.__bindgen_anon_2.rx_buffer = ptr::null_mut();

        // 执行SPI事务
        let result = unsafe { sys::spi_device_transmit(self.handle, &mut transaction) };

        if result != sys::ESP_OK {
            return Err(SpiError::DriverError(result));
        }

        Ok(())
    }
}

/// SPI3总线（ESP32-S3特有）初始化辅助函数
#[cfg(any(target_arch = "xtensa", feature = "esp32s3"))]
pub fn initialize_spi3(
    mosi_pin: i32,
    miso_pin: i32,
    sclk_pin: i32,
    max_transfer_sz: usize,
) -> SpiResult<SpiMaster> {
    let mut spi = SpiMaster::new(SpiBus::Spi3)?;
    spi.initialize(mosi_pin, miso_pin, sclk_pin, max_transfer_sz)?;
    Ok(spi)
}

/// 简单工厂函数，创建SPI2总线实例
pub fn initialize_spi2(
    mosi_pin: i32,
    miso_pin: i32,
    sclk_pin: i32,
    max_transfer_sz: usize,
) -> SpiResult<SpiMaster> {
    let mut spi = SpiMaster::new(SpiBus::Spi2)?;
    spi.initialize(mosi_pin, miso_pin, sclk_pin, max_transfer_sz)?;
    Ok(spi)
}
