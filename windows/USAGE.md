# SitePi SDWAN - 使用说明

## 首次使用指南

### 问题：403 Forbidden 授权错误

如果您看到这个错误：
```
ERROR Authorization failed: Authorization failed with status: 403 Forbidden
```

这表示您的公钥尚未在 SitePi 网络中注册。

### 解决方案 A：使用 Provision Code（推荐）

1. **访问 SitePi 控制台**
   - 打开浏览器访问：https://sitepi.cn
   - 注册并登录账户
   - 创建一个新网络

2. **获取 Provision Code**
   - 在网络设置中找到 "配置代码" (Provision Code)
   - 复制该代码

3. **使用 Provision Code 运行客户端**
   ```powershell
   .\target\release\sitepi.exe --interface wg0 --provision YOUR_PROVISION_CODE
   ```

   或在配置文件中设置：
   ```json
   {
     "server": "https://sitepi.cn",
     "interface": "wg0",
     "provision_code": "YOUR_PROVISION_CODE",
     "auto_route": true,
     "log_level": "info"
   }
   ```

   然后运行：
   ```powershell
   .\target\release\sitepi.exe --config config.json
   ```

### 解决方案 B：手动注册 Public Key

1. **获取您的 Public Key**
   
   首次运行程序时，日志中会显示：
   ```
   INFO Public key: T9zwNAEyJtN/x9t6bsB+KPEQqKSfhK8XFM3KxxqELHc=
   ```
   
   或查看配置文件：
   ```powershell
   type configs\wg0.conf
   ```

2. **在 SitePi 控制台注册**
   - 登录 https://sitepi.cn
   - 进入您的网络
   - 添加新站点
   - 输入上面获取的 Public Key
   - 保存

3. **重新运行客户端**
   ```powershell
   .\target\release\sitepi.exe --interface wg0
   ```

## 常见使用场景

### 场景 1：开发/测试环境

```powershell
# 使用 provision code，启用调试日志
$env:RUST_LOG="debug"
.\target\release\sitepi.exe --interface wg0 --provision YOUR_CODE
```

### 场景 2：生产环境

1. 创建配置文件 `config.json`：
```json
{
  "server": "https://sitepi.cn",
  "interface": "wg0",
  "provision_code": "YOUR_PROVISION_CODE",
  "auto_route": true,
  "log_level": "info",
  "reconnect_config": {
    "max_attempts": 10,
    "base_delay_ms": 2000
  }
}
```

2. 运行：
```powershell
.\target\release\sitepi.exe --config config.json
```

### 场景 3：自动路由

启用自动路由后，程序会自动管理到其他站点的路由：

```powershell
.\target\release\sitepi.exe --interface wg0 --provision YOUR_CODE --route true
```

或在配置文件中：
```json
{
  "auto_route": true
}
```

## 验证连接

成功连接后，您会看到类似的日志：

```
INFO SitePi SDWAN Client v0.0.9 starting
INFO Server: https://sitepi.cn
INFO Interface: wg0
INFO Public key: ABC123...
INFO Listen port: 51820
INFO Authorizing with server: https://sitepi.cn/authorize
INFO Network: my-network
INFO IP Address: 10.0.0.2
INFO Set adapter IP: 10.0.0.2/24
INFO Connecting to server stream...
INFO Connected, receiving messages...
INFO Update peer: DEF456... | 1.2.3.4:51820 | 10.0.0.3
```

## 检查网络状态

```powershell
# 查看 WireGuard 接口
Get-NetAdapter | Where-Object {$_.Name -like "*wg*"}

# 查看接口 IP
Get-NetIPAddress | Where-Object {$_.InterfaceAlias -like "*wg*"}

# 查看路由（如果启用了 auto_route）
route print | Select-String "10.0.0"

# 测试连接到其他节点
ping 10.0.0.3
```

## 优雅停止

按 `Ctrl+C` 一次即可优雅停止程序。程序会：
1. 停止接收新消息
2. 断开服务器连接
3. 保留 WireGuard 接口和配置

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
- 安装 WireGuard for Windows: https://www.wireguard.com/install/
- 或将 `wireguard.dll` 复制到程序目录

### 问题：连接超时
```
Error: Failed to connect to server: Connection timeout
```
**解决**：
- 检查网络连接
- 检查防火墙设置
- 确认服务器地址正确

### 问题：Provision Code 无效
```
Error: Authorization failed with status: 403 Forbidden
```
**解决**：
- 确认 provision code 正确
- 检查 provision code 是否过期
- 在服务器控制台重新生成

## 获取帮助

```powershell
# 查看所有选项
.\target\release\sitepi.exe --help

# 查看版本
.\target\release\sitepi.exe --version
```

## 更多信息

- 完整文档：`README.windows.md`
- 快速入门：`QUICKSTART.md`
- 问题反馈：https://github.com/sitepi/sdwan/issues
