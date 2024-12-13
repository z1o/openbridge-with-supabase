# OpenBridge with Supabase

一个基于 Rust 开发的开源项目，用于连使用 Supabase 服务生成 OpenBridge 所需要的配置文件。

## 功能特点

- 使用 Supabase 服务生成 OpenBridge 所需要的配置文件
- 高性能 Rust 实现
- 跨平台支持

## 安装

### 1. 安装系统依赖

在 Ubuntu/Debian 11 及更早版本上：
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev
```

在 Debian 12 (Bookworm) 上：
```bash
echo "deb http://security.debian.org/debian-security bullseye-security main" | sudo tee /etc/apt/sources.list.d/bullseye-security.list
sudo apt update
sudo apt install -y libssl1.1
```

在 macOS 系统上：
```bash
brew install openssl
```

### 2. 安装 Rust 和 Cargo

访问 https://rustup.rs/ 并按照说明安装 Rust。

### 3. 构建和运行

```bash
cargo build --release
export SUPABASE_URL=https://your-supabase-url.supabase.co
export SUPABASE_KEY=your-supabase-key
export OPENBRIDGE_CONFIG_PATH=./openbridge.json
./target/release/openbridge-with-supabase
```

## 贡献指南

欢迎提交 Pull Request 和 Issue！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解更多细节。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件 