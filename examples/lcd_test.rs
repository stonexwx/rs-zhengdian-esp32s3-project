use esp32_test::drivers::atk_md0130::{self, DisplayRotation};
use esp32_test::drivers::atk_md0130::{color, ATKMD0130};
use std::thread;
use std::time::Duration;

fn main() {
    // 初始化ESP-IDF
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    println!("ATK-MD0130 LCD测试开始运行!");

    // 初始化LCD
    // 注意: 请根据实际硬件连接修改引脚定义
    let mut lcd = atk_md0130::create_atk_md0130(
        11,       // MOSI引脚 IO11
        13,       // MISO引脚 IO13
        12,       // SCK引脚 IO12
        21,       // CS引脚 (自定义)
        40,       // DC引脚 (自定义)
        8,        // RST引脚 (自定义)
        Some(14), // BL引脚 (自定义)
    )
    .expect("初始化LCD失败");

    // LCD基本功能演示
    lcd_demo(&mut lcd);
}

fn lcd_demo(lcd: &mut ATKMD0130) {
    println!("LCD 基本功能演示");

    // 清屏为黑色
    lcd.fill_rect(0, 0, 240, 240, color::BLACK)
        .expect("清屏失败");
    thread::sleep(Duration::from_millis(500));

    // 显示彩色方块
    lcd.fill_rect(0, 0, 80, 80, color::RED).expect("绘制失败");
    lcd.fill_rect(80, 0, 80, 80, color::GREEN)
        .expect("绘制失败");
    lcd.fill_rect(160, 0, 80, 80, color::BLUE)
        .expect("绘制失败");

    lcd.fill_rect(0, 80, 80, 80, color::YELLOW)
        .expect("绘制失败");
    lcd.fill_rect(80, 80, 80, 80, color::CYAN)
        .expect("绘制失败");
    lcd.fill_rect(160, 80, 80, 80, color::MAGENTA)
        .expect("绘制失败");

    lcd.fill_rect(0, 160, 80, 80, color::WHITE)
        .expect("绘制失败");
    lcd.fill_rect(80, 160, 80, 80, color::GRAY)
        .expect("绘制失败");
    lcd.fill_rect(160, 160, 80, 80, color::ORANGE)
        .expect("绘制失败");

    thread::sleep(Duration::from_secs(2));

    // 绘制几何图形
    lcd.fill_rect(0, 0, 240, 240, color::BLACK)
        .expect("清屏失败");

    // 绘制直线
    for i in (0..240).step_by(20) {
        lcd.draw_line(0, i as i16, 240 as i16, (240 - i) as i16, color::WHITE)
            .expect("绘制线失败");
    }
    thread::sleep(Duration::from_secs(1));

    // 绘制矩形
    lcd.fill_rect(0, 0, 240, 240, color::BLACK)
        .expect("清屏失败");
    for i in (10..100).step_by(20) {
        lcd.draw_rect(
            (120 - i) as u16,
            (120 - i) as u16,
            (i * 2) as u16,
            (i * 2) as u16,
            color::GREEN,
        )
        .expect("绘制矩形失败");
    }
    thread::sleep(Duration::from_secs(1));

    // 绘制填充矩形
    lcd.fill_rect(50, 50, 140, 140, color::BLUE)
        .expect("绘制填充矩形失败");
    thread::sleep(Duration::from_secs(1));

    // 绘制圆形
    lcd.fill_rect(0, 0, 240, 240, color::BLACK)
        .expect("清屏失败");
    for r in (20..120).step_by(20) {
        lcd.draw_circle(120, 120, r, color::RED)
            .expect("绘制圆失败");
    }
    thread::sleep(Duration::from_secs(1));

    // 绘制填充圆
    lcd.fill_circle(120, 120, 60, color::YELLOW)
        .expect("绘制填充圆失败");
    thread::sleep(Duration::from_secs(1));

    // 显示方向测试
    lcd.fill_rect(0, 0, 240, 240, color::BLACK)
        .expect("清屏失败");

    // 绘制边框和箭头，指示显示方向
    lcd.draw_rect(0, 0, 240, 240, color::WHITE)
        .expect("绘制边框失败");

    // 从屏幕中心到上边的箭头
    lcd.draw_line(120, 120, 120, 20, color::RED)
        .expect("绘制箭头失败");
    lcd.draw_line(120, 20, 110, 40, color::RED)
        .expect("绘制箭头失败");
    lcd.draw_line(120, 20, 130, 40, color::RED)
        .expect("绘制箭头失败");

    // 显示文本"TOP"在箭头上方（简化为矩形表示）
    lcd.fill_rect(100, 0, 40, 15, color::RED)
        .expect("绘制文本框失败");

    println!("正在切换显示方向...");
    thread::sleep(Duration::from_secs(1));

    // 顺时针旋转90度
    lcd.set_rotation(DisplayRotation::Landscape)
        .expect("设置旋转失败");
    thread::sleep(Duration::from_secs(2));

    // 顺时针旋转180度
    lcd.set_rotation(DisplayRotation::PortraitFlipped)
        .expect("设置旋转失败");
    thread::sleep(Duration::from_secs(2));

    // 顺时针旋转270度
    lcd.set_rotation(DisplayRotation::LandscapeFlipped)
        .expect("设置旋转失败");
    thread::sleep(Duration::from_secs(2));

    // 恢复原始方向
    lcd.set_rotation(DisplayRotation::Portrait)
        .expect("设置旋转失败");
    thread::sleep(Duration::from_secs(1));

    // 背光控制测试
    println!("背光控制测试");
    lcd.fill_rect(0, 0, 240, 240, color::WHITE)
        .expect("清屏失败");
    thread::sleep(Duration::from_secs(1));

    lcd.set_backlight(false).expect("背光控制失败");
    println!("背光已关闭");
    thread::sleep(Duration::from_secs(1));

    lcd.set_backlight(true).expect("背光控制失败");
    println!("背光已打开");
    thread::sleep(Duration::from_secs(1));

    // 最终效果
    lcd.fill_rect(0, 0, 240, 240, color::BLACK)
        .expect("清屏失败");

    // 绘制一些彩色图形作为最终显示
    lcd.fill_circle(60, 60, 50, color::RED).expect("绘制失败");
    lcd.fill_circle(180, 60, 50, color::GREEN)
        .expect("绘制失败");
    lcd.fill_circle(60, 180, 50, color::BLUE).expect("绘制失败");
    lcd.fill_circle(180, 180, 50, color::YELLOW)
        .expect("绘制失败");

    // 在圆的交叉处绘制白色区域
    lcd.fill_rect(60, 60, 120, 120, color::WHITE)
        .expect("绘制失败");

    // 在中间绘制一个黑色圆
    lcd.fill_circle(120, 120, 50, color::BLACK)
        .expect("绘制失败");

    println!("ATK-MD0130 LCD测试完成!");
}
