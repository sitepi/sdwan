# SitePi SDWAN 客户端

一个轻量级且高效的软件定义广域网 (SD-WAN) 客户端实现。

## 获取networkid
访问 [https://sitepi.net](https://sitepi.net) 并注册账号，创建网络, 获取networkid

## 系统要求

- Linux/OpenWrt
- 具有 root/管理员权限的网络接口

### Ubuntu
```bash
sudo apt update
sudo apt install -y wireguard-tools curl

wget https://github.com/sitepi/sdwan/releases/download/v0.0.2/sitepi_0.0.2_all.deb
sudo dpkg -i sitepi_0.0.2_all.deb

sudo systemctl enable sitepi.service
```

#### 配置
```bash
sudo vim /etc/sitepi/config.json # edit configuration, binding your network ID

sudo service sitepi {status|start|stop|restart}
```

### OpenWrt
#### 下载并安装
```bash
cd /tmp
wget https://github.com/sitepi/sdwan/releases/download/v0.0.2/sitepi_0.0.2_all.ipk
wget https://github.com/sitepi/sdwan/releases/download/v0.0.2/luci-app-sitepi_0.0.2_all.ipk

opkg install sitepi_0.0.2_all.ipk
opkg install luci-app-sitepi_0.0.2_all.ipk
```

- 安装文件架构是平台无关的。所有路由器使用相同的ipk。

#### 配置
   1. Go to LuCI web interface
   2. Navigate to Services -> Sitepi SDWAN
   3. Configure:
      - Enable the service
      - Set WireGuard interface name
      - Optionally set server address
      - Optionally set network ID      # binding your network ID
   4. Save & Apply

## 功能

- WireGuard-based
- Intelligent traffic routing
- QoS (Quality of Service) management
- Real-time network monitoring
- Automatic failover
- Multi-link support
- Zero-touch provisioning

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
