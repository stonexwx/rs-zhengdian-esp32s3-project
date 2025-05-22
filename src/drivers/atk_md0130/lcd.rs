// ATK-MD0130 LCD驱动模块
// ST7789V控制器, 1.3英寸, 240x240像素

use super::r#type::{cmd, madctl, ColorFormat, DisplayRotation, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::drivers::gpio::{GpioInterruptType, GpioMode, GpioPin, GpioPullMode};
use crate::drivers::spi::{
    SpiBitOrder, SpiDevice, SpiDeviceConfig, SpiError, SpiMaster, SpiMode, SpiResult,
};

use esp_idf_svc::sys::{esp_rom_delay_us, ets_delay_us};
use std::thread;
use std::time::Duration;

/// ATK-MD0130 LCD显示器驱动
pub struct ATKMD0130 {
    /// SPI主机控制器
    spi_master: SpiMaster,
    /// SPI设备
    spi_device: SpiDevice,
    /// 复位引脚
    rst_pin: GpioPin,
    /// 数据/命令引脚（高电平=数据，低电平=命令）
    dc_pin: GpioPin,
    /// 背光引脚（高电平=开启，低电平=关闭）
    bl_pin: Option<GpioPin>,
    /// 当前显示方向
    rotation: DisplayRotation,
    /// 当前颜色格式
    color_format: ColorFormat,
    /// 当前窗口X起始位置
    window_x_start: u16,
    /// 当前窗口Y起始位置
    window_y_start: u16,
    /// 当前窗口宽度
    window_width: u16,
    /// 当前窗口高度
    window_height: u16,
}

impl ATKMD0130 {
    /// 创建新的ATK-MD0130实例
    ///
    /// # 参数
    ///
    /// * `spi_master` - SPI主机控制器
    /// * `spi_device` - SPI设备
    /// * `rst_pin` - 复位引脚
    /// * `dc_pin` - 数据/命令引脚
    /// * `bl_pin` - 背光引脚（可选）
    ///
    /// # 返回
    ///
    /// 成功返回LCD实例，失败返回错误
    pub fn new(
        spi_master: SpiMaster,
        spi_device: SpiDevice,
        rst_pin: GpioPin,
        dc_pin: GpioPin,
        bl_pin: Option<GpioPin>,
    ) -> SpiResult<Self> {
        // 初始化DC引脚为输出
        dc_pin
            .init(
                GpioMode::Output,
                GpioPullMode::Floating,
                GpioInterruptType::Disable,
            )
            .map_err(|_| SpiError::InvalidParameter)?;

        // 初始化RST引脚为输出
        rst_pin
            .init(
                GpioMode::Output,
                GpioPullMode::Floating,
                GpioInterruptType::Disable,
            )
            .map_err(|_| SpiError::InvalidParameter)?;

        // 如果有背光引脚，初始化为输出
        if let Some(ref pin) = bl_pin {
            pin.init(
                GpioMode::Output,
                GpioPullMode::Floating,
                GpioInterruptType::Disable,
            )
            .map_err(|_| SpiError::InvalidParameter)?;
        }

        // 创建LCD实例
        let mut lcd = ATKMD0130 {
            spi_master,
            spi_device,
            rst_pin,
            dc_pin,
            bl_pin,
            rotation: DisplayRotation::Portrait,
            color_format: ColorFormat::RGB565,
            window_x_start: 0,
            window_y_start: 0,
            window_width: DISPLAY_WIDTH,
            window_height: DISPLAY_HEIGHT,
        };

        // 初始化显示
        lcd.initialize()?;
        Ok(lcd)
    }

    /// 初始化显示器
    fn initialize(&mut self) -> SpiResult<()> {
        // 硬件复位
        self.hardware_reset();

        // 退出睡眠模式
        self.write_command(cmd::SLPOUT)?;
        thread::sleep(Duration::from_millis(120));

        // 设置颜色格式 (16-bit 65K 颜色)
        self.set_color_format(ColorFormat::RGB565)?;

        // 设置显示方向
        self.set_rotation(DisplayRotation::Portrait)?;

        // 打开显示反相
        self.write_command(cmd::INVON)?;

        // 设置伽马校正
        self.write_command(cmd::GMCTRP1)?;
        self.write_data(&[
            0x0f, 0x22, 0x1C, 0x1B, 0x08, 0x0F, 0x48, 0xB8, 0x34, 0x05, 0x0C, 0x09, 0x0F, 0x07,
            0x00,
        ])?;

        self.write_command(cmd::GMCTRN1)?;
        self.write_data(&[
            0x0F, 0x23, 0x1C, 0x1B, 0x09, 0x10, 0x48, 0xB8, 0x34, 0x05, 0x0C, 0x09, 0x0F, 0x07,
            0x00,
        ])?;

        thread::sleep(Duration::from_millis(10));

        // 开启显示
        self.write_command(cmd::DISPON)?;
        thread::sleep(Duration::from_millis(120));

        // 如果有背光，打开背光
        if let Some(ref pin) = self.bl_pin {
            pin.set_high().map_err(|_| SpiError::DriverError(-1))?;
        }

        // 清屏为黑色
        self.fill_rect(0, 0, self.window_width, self.window_height, 0x0000)?;

        Ok(())
    }

    /// 硬件复位
    fn hardware_reset(&mut self) {
        self.rst_pin.set_high().unwrap_or(());
        thread::sleep(Duration::from_millis(10));
        self.rst_pin.set_low().unwrap_or(());
        thread::sleep(Duration::from_millis(10));
        self.rst_pin.set_high().unwrap_or(());
        thread::sleep(Duration::from_millis(120));
    }

    /// 写命令
    fn write_command(&mut self, cmd: u8) -> SpiResult<()> {
        self.dc_pin
            .set_low()
            .map_err(|_| SpiError::DriverError(-1))?;
        self.spi_device.write(&[cmd])
    }

    /// 写数据
    fn write_data(&mut self, data: &[u8]) -> SpiResult<()> {
        self.dc_pin
            .set_high()
            .map_err(|_| SpiError::DriverError(-1))?;
        self.spi_device.write(data)
    }

    /// 写16位数据
    fn write_data_u16(&mut self, data: u16) -> SpiResult<()> {
        let data_bytes = [(data >> 8) as u8, data as u8];
        self.write_data(&data_bytes)
    }

    /// 设置地址窗口
    fn set_address_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> SpiResult<()> {
        // 设置列地址
        self.write_command(cmd::CASET)?;
        self.write_data_u16(x0)?;
        self.write_data_u16(x1)?;

        // 设置行地址
        self.write_command(cmd::RASET)?;
        self.write_data_u16(y0)?;
        self.write_data_u16(y1)?;

        // 准备写入内存
        self.write_command(cmd::RAMWR)?;

        Ok(())
    }

    /// 设置显示方向
    pub fn set_rotation(&mut self, rotation: DisplayRotation) -> SpiResult<()> {
        let rotation_value = match rotation {
            DisplayRotation::Portrait => madctl::MX | madctl::BGR, // 0度旋转
            DisplayRotation::Landscape => madctl::MV | madctl::BGR, // 90度旋转
            DisplayRotation::PortraitFlipped => madctl::MY | madctl::BGR, // 180度旋转
            DisplayRotation::LandscapeFlipped => madctl::MV | madctl::MY | madctl::BGR, // 270度旋转
        };

        self.write_command(cmd::MADCTL)?;
        self.write_data(&[rotation_value])?;

        // 更新宽度和高度
        match rotation {
            DisplayRotation::Portrait | DisplayRotation::PortraitFlipped => {
                self.window_width = DISPLAY_WIDTH;
                self.window_height = DISPLAY_HEIGHT;
            }
            DisplayRotation::Landscape | DisplayRotation::LandscapeFlipped => {
                self.window_width = DISPLAY_HEIGHT;
                self.window_height = DISPLAY_WIDTH;
            }
        }

        self.rotation = rotation;
        Ok(())
    }

    /// 设置颜色格式
    pub fn set_color_format(&mut self, format: ColorFormat) -> SpiResult<()> {
        let format_value = match format {
            ColorFormat::RGB565 => 0x55, // 16位/像素
            ColorFormat::RGB888 => 0x66, // 18位/像素 (24位数据，但控制器使用18位)
        };

        self.write_command(cmd::COLMOD)?;
        self.write_data(&[format_value])?;

        self.color_format = format;
        Ok(())
    }

    /// 设置背光亮度（如果支持）
    pub fn set_backlight(&mut self, on: bool) -> SpiResult<()> {
        if let Some(ref pin) = self.bl_pin {
            if on {
                pin.set_high().map_err(|_| SpiError::DriverError(-1))?;
            } else {
                pin.set_low().map_err(|_| SpiError::DriverError(-1))?;
            }
        }
        Ok(())
    }

    /// 绘制像素
    pub fn draw_pixel(&mut self, x: u16, y: u16, color: u16) -> SpiResult<()> {
        if x >= self.window_width || y >= self.window_height {
            return Ok(());
        }

        self.set_address_window(x, y, x, y)?;
        self.write_data_u16(color)
    }

    /// 填充矩形区域
    pub fn fill_rect(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        color: u16,
    ) -> SpiResult<()> {
        if x >= self.window_width || y >= self.window_height {
            return Ok(());
        }

        let x1 = (x + width - 1).min(self.window_width - 1);
        let y1 = (y + height - 1).min(self.window_height - 1);

        self.set_address_window(x, y, x1, y1)?;

        // 计算需要填充的像素数量
        let num_pixels = (x1 - x + 1) as usize * (y1 - y + 1) as usize;

        // 为了提高效率，批量发送颜色数据
        self.dc_pin
            .set_high()
            .map_err(|_| SpiError::DriverError(-1))?;

        // 每次发送的像素数量
        const CHUNK_SIZE: usize = 64;
        let mut color_buffer = [0u8; CHUNK_SIZE * 2];

        // 填充颜色缓冲区
        for i in 0..CHUNK_SIZE {
            color_buffer[i * 2] = (color >> 8) as u8;
            color_buffer[i * 2 + 1] = color as u8;
        }

        // 分块发送数据
        let mut remaining = num_pixels;
        while remaining > 0 {
            let chunk = remaining.min(CHUNK_SIZE);
            self.spi_device.write(&color_buffer[0..chunk * 2])?;
            remaining -= chunk;
        }

        Ok(())
    }

    /// 绘制水平线
    pub fn draw_hline(&mut self, x: u16, y: u16, width: u16, color: u16) -> SpiResult<()> {
        self.fill_rect(x, y, width, 1, color)
    }

    /// 绘制垂直线
    pub fn draw_vline(&mut self, x: u16, y: u16, height: u16, color: u16) -> SpiResult<()> {
        self.fill_rect(x, y, 1, height, color)
    }

    /// 绘制线段 (使用Bresenham算法)
    pub fn draw_line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: u16) -> SpiResult<()> {
        // 计算Δx和Δy
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

        // 计算步进方向
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        // 计算误差
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        // 当前位置
        let mut x = x0;
        let mut y = y0;

        loop {
            // 如果坐标在显示区域内，绘制像素
            if x >= 0 && x < self.window_width as i16 && y >= 0 && y < self.window_height as i16 {
                self.draw_pixel(x as u16, y as u16, color)?;
            }

            // 到达终点，结束
            if x == x1 && y == y1 {
                break;
            }

            // 更新误差和位置
            err2 = err;
            if err2 > -dx {
                err -= dy;
                x += sx;
            }
            if err2 < dy {
                err += dx;
                y += sy;
            }
        }

        Ok(())
    }

    /// 绘制空心矩形
    pub fn draw_rect(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        color: u16,
    ) -> SpiResult<()> {
        self.draw_hline(x, y, width, color)?;
        self.draw_hline(x, y + height - 1, width, color)?;
        self.draw_vline(x, y, height, color)?;
        self.draw_vline(x + width - 1, y, height, color)?;

        Ok(())
    }

    /// 绘制空心圆
    pub fn draw_circle(
        &mut self,
        x_center: u16,
        y_center: u16,
        radius: u16,
        color: u16,
    ) -> SpiResult<()> {
        let mut x = radius;
        let mut y: i16 = 0;
        let mut err: i16 = 0;

        while x >= y as u16 {
            self.draw_pixel(x_center + x, y_center + y as u16, color)?;
            self.draw_pixel(x_center + y as u16, y_center + x, color)?;
            self.draw_pixel(x_center - y as u16, y_center + x, color)?;
            self.draw_pixel(x_center - x, y_center + y as u16, color)?;
            self.draw_pixel(x_center - x, y_center - y as u16, color)?;
            self.draw_pixel(x_center - y as u16, y_center - x, color)?;
            self.draw_pixel(x_center + y as u16, y_center - x, color)?;
            self.draw_pixel(x_center + x, y_center - y as u16, color)?;

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x as i16 + 1;
            }
        }

        Ok(())
    }

    /// 绘制填充圆
    pub fn fill_circle(
        &mut self,
        x_center: u16,
        y_center: u16,
        radius: u16,
        color: u16,
    ) -> SpiResult<()> {
        let mut x = radius;
        let mut y: i16 = 0;
        let mut err: i16 = 0;

        while x >= y as u16 {
            self.draw_hline(x_center - x, y_center + y as u16, 2 * x + 1, color)?;
            self.draw_hline(x_center - y as u16, y_center + x, 2 * y as u16 + 1, color)?;
            self.draw_hline(x_center - x, y_center - y as u16, 2 * x + 1, color)?;
            self.draw_hline(x_center - y as u16, y_center - x, 2 * y as u16 + 1, color)?;

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x as i16 + 1;
            }
        }

        Ok(())
    }

    /// 显示图像数据 (RGB565格式)
    pub fn draw_image(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        image_data: &[u16],
    ) -> SpiResult<()> {
        if x >= self.window_width || y >= self.window_height || width == 0 || height == 0 {
            return Ok(());
        }

        // 确保坐标在屏幕范围内
        let x_end = (x + width - 1).min(self.window_width - 1);
        let y_end = (y + height - 1).min(self.window_height - 1);
        let actual_width = x_end - x + 1;
        let actual_height = y_end - y + 1;

        // 设置地址窗口
        self.set_address_window(x, y, x_end, y_end)?;

        // 转换为字节数组并发送
        let num_pixels = actual_width as usize * actual_height as usize;
        if num_pixels > image_data.len() {
            return Err(SpiError::InvalidParameter);
        }

        // 预备数据
        let mut data_buffer = Vec::with_capacity(num_pixels * 2);
        for color in image_data.iter().take(num_pixels) {
            data_buffer.push((*color >> 8) as u8);
            data_buffer.push(*color as u8);
        }

        // 发送数据
        self.dc_pin
            .set_high()
            .map_err(|_| SpiError::DriverError(-1))?;
        self.spi_device.write(&data_buffer)
    }
}

// 工厂方法，方便创建ATK-MD0130实例
pub fn create_atk_md0130(
    mosi_pin: i32,
    miso_pin: i32,
    sclk_pin: i32,
    cs_pin: i32,
    dc_pin: u32,
    rst_pin: u32,
    bl_pin: Option<u32>,
) -> SpiResult<ATKMD0130> {
    // 创建SPI总线和设备
    let mut spi_master = SpiMaster::new(crate::drivers::spi::SpiBus::Spi2)?;
    spi_master.initialize(mosi_pin, miso_pin, sclk_pin, 0)?;

    // 创建SPI设备
    let config = SpiDeviceConfig {
        clock_speed_hz: 40_000_000, // 40MHz
        mode: SpiMode::Mode0,
        bit_order: SpiBitOrder::MSBFirst,
        command_bits: 0,
        address_bits: 0,
        cs_pin: Some(cs_pin),
        queue_size: 1,
    };
    let spi_device = spi_master.add_device(&config)?;

    // 创建GPIO引脚
    let dc = GpioPin::new(dc_pin);
    let rst = GpioPin::new(rst_pin);
    let bl = bl_pin.map(GpioPin::new);

    // 创建LCD实例
    ATKMD0130::new(spi_master, spi_device, rst, dc, bl)
}
