# rs-zhengdian-esp32s3-project
rust实现正点原子esp32S3开发板ai小智项目,使用[std] Rust编写

## 前期配置
### 开发环境安装
1. 安装`espup`
`cargo install espup`
2. 安装必要的工具链
`espup install`

具体可以参考[ Rust on ESP](https://narukara.github.io/rust-on-esp-book-zh-cn/introduction.html)这本书进行配置

## 目录结构
```
├── build.rs                   # 构建脚本，用于编译时的配置
├── Cargo.lock                 # Cargo锁文件，确保依赖版本一致
├── Cargo.toml                 # Rust包管理配置文件
├── LICENSE                    # 许可证文件
├── README.md                  # 项目说明文档
├── rust-toolchain.toml        # Rust工具链配置
├── sdkconfig.defaults         # ESP-IDF SDK默认配置
├── examples/                  # 示例代码目录
│   └── led_ctl_test.rs        # LED控制测试示例
└── src/                       # 源代码目录
    ├── lib.rs                 # 库入口文件
    ├── main.rs                # 主程序入口
    └── led/                   # LED模块目录
        └── mod.rs             # LED模块定义
```
