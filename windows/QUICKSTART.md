# SitePi SDWAN Windows 客户端 - 快速入门

## 5 分钟快速开始

### 第 1 步：安装依赖

1. **安装 Rust** (如果未安装)
   ```powershell
   # 下载并运行 rustup-init.exe
   # https://rustup.rs/
   ```

2. **安装 WireGuard**
   - 下载并安装：https://www.wireguard.com/install/
   - 或只需将 `wireguard.dll` 放到程序目录

### 第 2 步：编译程序

```powershell
# 进入 windows 目录
cd windows

# 编译（Release 模式）
cargo build --release

# 或使用构建脚本
.\build.ps1 -Release
```

### 第 3 步：配置

**方式 A：使用命令行**
```powershell
# 以管理员身份运行 PowerShell
.\target\release\sitepi.exe --interface wg0 --server https://sitepi.cn
```

**方式 B：使用配置文件**
```powershell
# 复制配置示例
copy config.example.json config.json

# 编辑配置文件
notepad config.json
```

配置文件内容：
```json
{
  "server": "https://sitepi.cn",
  "interface": "wg0",
  "provision_code": null,
  "auto_route": true,
  "log_level": "info",
  "reconnect_config": {
    "max_attempts": 5,
    "base_delay_ms": 1000
  }
}
```

### 第 4 步：运行

```powershell
# 以管理员身份运行
.\target\release\sitepi.exe --config config.json

# 或使用构建脚本
.\build.ps1 -Release -Run -Interface wg0 -Config config.json
```

## 常用命令

```powershell
# 开发模式编译和运行（更快）
cargo build
cargo run -- --interface wg0

# Release 模式（优化性能）
cargo build --release
.\target\release\sitepi.exe --interface wg0

# 启用调试日志
$env:RUST_LOG="debug"
.\target\release\sitepi.exe --interface wg0

# 使用 provision code
.\target\release\sitepi.exe --interface wg0 --provision YOUR_CODE

# 启用自动路由
.\target\release\sitepi.exe --interface wg0 --route true
```

## 验证运行

运行成功后，你应该看到类似的输出：

```
2025-12-31T10:00:00.123Z  INFO sitepi: SitePi SDWAN Client v0.0.9 starting
2025-12-31T10:00:00.234Z  INFO sitepi: Server: https://sitepi.cn
2025-12-31T10:00:00.345Z  INFO sitepi: Interface: wg0
2025-12-31T10:00:00.456Z  INFO sitepi: Opened existing adapter: wg0
2025-12-31T10:00:00.567Z  INFO sitepi: Public key: ABC123...
2025-12-31T10:00:00.678Z  INFO sitepi: Listen port: 51820
2025-12-31T10:00:01.123Z  INFO sitepi: Authorizing with server...
2025-12-31T10:00:01.456Z  INFO sitepi: Network: my-network
2025-12-31T10:00:01.567Z  INFO sitepi: IP Address: 10.0.0.2
2025-12-31T10:00:01.678Z  INFO sitepi: Set adapter IP: 10.0.0.2/24
2025-12-31T10:00:01.789Z  INFO sitepi: Connecting to server stream...
2025-12-31T10:00:02.123Z  INFO sitepi: Connected, receiving messages...
2025-12-31T10:00:03.456Z  INFO sitepi: Update peer: ABC456... | 1.2.3.4:51820 | 10.0.0.3
2025-12-31T10:00:03.567Z  INFO sitepi: Added route: 192.168.1.0/24 via 10.0.0.3
```

## 检查连接状态

```powershell
# 查看 WireGuard 接口
Get-NetAdapter | Where-Object {$_.Name -like "wg*"}

# 查看路由表
route print

# 查看 WireGuard 配置（如果使用 WireGuard GUI）
# 打开 WireGuard 应用查看状态
```

## 故障排除

### 问题：权限不足
```
Error: Failed to create WireGuard adapter
```
**解决**：以管理员身份运行 PowerShell

### 问题：找不到 wireguard.dll
```
Error: Failed to load wireguard.dll
```
**解决**：
- 安装 WireGuard for Windows
- 或将 wireguard.dll 复制到程序目录
- 或添加 WireGuard 目录到 PATH

### 问题：接口已存在
```
Error: Failed to create adapter - already exists
```
**解决**：删除现有接口或使用不同的接口名

### 问题：连接服务器失败
```
Error: Authorization failed with status: 500
```
**解决**：
- 检查服务器地址是否正确
- 检查网络连接
- 查看服务器日志

## 下一步

1. 📖 阅读完整文档：`README.windows.md`
2. 🔧 调整配置：编辑 `config.json`
3. 🚀 部署为服务：使用 `--install-service`（待实现）
4. 📊 监控性能：使用 `debug` 日志级别

## 需要帮助？

- 查看详细文档：`README.windows.md`
- 检查日志输出
- 提交 Issue：https://github.com/sitepi/sdwan/issues

## 优化建议

- ✅ **生产环境**：使用 Release 模式编译
- ✅ **调试问题**：设置 `RUST_LOG=debug`
- ✅ **自动路由**：启用 `auto_route: true`
- ✅ **稳定连接**：调整 `reconnect_config` 参数
