/// ATK-MD0130 ST7789V 1.3英寸LCD显示模块类型定义

/// 显示区域的颜色格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    /// RGB565 格式 (16 bits per pixel)
    RGB565,
    /// RGB888 格式 (24 bits per pixel)
    RGB888,
}

/// 显示方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayRotation {
    /// 0度旋转
    Portrait,
    /// 顺时针90度旋转
    Landscape,
    /// 顺时针180度旋转
    PortraitFlipped,
    /// 顺时针270度旋转
    LandscapeFlipped,
}

/// 基本颜色常量 (RGB565格式)
pub mod color {
    pub const BLACK: u16 = 0x0000;
    pub const BLUE: u16 = 0x001F;
    pub const RED: u16 = 0xF800;
    pub const GREEN: u16 = 0x07E0;
    pub const CYAN: u16 = 0x07FF;
    pub const MAGENTA: u16 = 0xF81F;
    pub const YELLOW: u16 = 0xFFE0;
    pub const WHITE: u16 = 0xFFFF;
    pub const GRAY: u16 = 0x8430;
    pub const LIGHT_GRAY: u16 = 0xC618;
    pub const DARK_GRAY: u16 = 0x4208;
    pub const NAVY: u16 = 0x000F;
    pub const DARK_GREEN: u16 = 0x03E0;
    pub const DARK_CYAN: u16 = 0x03EF;
    pub const MAROON: u16 = 0x7800;
    pub const PURPLE: u16 = 0x780F;
    pub const OLIVE: u16 = 0x7BE0;
    pub const ORANGE: u16 = 0xFD20;
    pub const PINK: u16 = 0xFE19;
    pub const BROWN: u16 = 0xA145;
}

/// LCD显示命令
pub mod cmd {
    // 系统命令
    pub const NOP: u8 = 0x00; // 空操作
    pub const SWRESET: u8 = 0x01; // 软件复位
    pub const RDDID: u8 = 0x04; // 读取显示器ID
    pub const RDDST: u8 = 0x09; // 读取显示器状态
    pub const SLPIN: u8 = 0x10; // 进入睡眠模式
    pub const SLPOUT: u8 = 0x11; // 退出睡眠模式
    pub const PTLON: u8 = 0x12; // 部分显示模式开启
    pub const NORON: u8 = 0x13; // 普通显示模式开启

    // 电源控制
    pub const INVOFF: u8 = 0x20; // 关闭反相显示
    pub const INVON: u8 = 0x21; // 开启反相显示
    pub const DISPOFF: u8 = 0x28; // 关闭显示
    pub const DISPON: u8 = 0x29; // 开启显示
    pub const CASET: u8 = 0x2A; // 列地址设置
    pub const RASET: u8 = 0x2B; // 行地址设置
    pub const RAMWR: u8 = 0x2C; // 内存写入
    pub const RAMRD: u8 = 0x2E; // 内存读取

    // 接口控制
    pub const MADCTL: u8 = 0x36; // 存储器访问控制
    pub const COLMOD: u8 = 0x3A; // 接口像素格式

    // 显示控制
    pub const FRMCTR1: u8 = 0xB1; // 帧速率控制（普通模式/全彩色）
    pub const FRMCTR2: u8 = 0xB2; // 帧速率控制（空闲模式/8色）
    pub const FRMCTR3: u8 = 0xB3; // 帧速率控制（局部模式/全彩色）
    pub const INVCTR: u8 = 0xB4; // 显示反相控制
    pub const PWCTR1: u8 = 0xC0; // 电源控制1
    pub const PWCTR2: u8 = 0xC1; // 电源控制2
    pub const PWCTR3: u8 = 0xC2; // 电源控制3
    pub const PWCTR4: u8 = 0xC3; // 电源控制4
    pub const PWCTR5: u8 = 0xC4; // 电源控制5
    pub const VMCTR1: u8 = 0xC5; // VCOM控制1
    pub const GMCTRP1: u8 = 0xE0; // 正极性伽马校正
    pub const GMCTRN1: u8 = 0xE1; // 负极性伽马校正
}

/// 存储器访问控制位定义
pub mod madctl {
    /// 行地址顺序（0=从上到下，1=从下到上）
    pub const MY: u8 = 0x80;
    /// 列地址顺序（0=从左到右，1=从右到左）
    pub const MX: u8 = 0x40;
    /// 行/列交换（0=正常，1=交换）
    pub const MV: u8 = 0x20;
    /// 垂直刷新顺序（0=从上到下，1=从下到上）
    pub const ML: u8 = 0x10;
    /// RGB/BGR顺序（0=RGB，1=BGR）
    pub const RGB: u8 = 0x00;
    /// BGR/RGB顺序（0=RGB，1=BGR）
    pub const BGR: u8 = 0x08;
    /// 水平刷新顺序（0=从左到右，1=从右到左）
    pub const MH: u8 = 0x04;
}

/// ATK-MD0130显示器物理尺寸
pub const DISPLAY_WIDTH: u16 = 240;
pub const DISPLAY_HEIGHT: u16 = 240;
