# SitePi SDWAN 客户端

一个轻量级且高效的软件定义广域网 (SD-WAN) 客户端实现。

## 管理
访问 [https://sitepi.cn](https://sitepi.cn) 并注册账号，创建网络, 输入站点的 public key 即可绑定到网络

## 网络配置代码
也可以在安装站点程序时, 输入网络的 配置代码, 站点会自动绑定到网络

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
sudo vim /etc/sitepi/config.json # optional edit configuration, binding provisioning code

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
   1. 访问 LuCI 网页界面
   2. 导航到 服务 -> Sitepi SDWAN
   3. 配置：
      - 启用服务
      - 设置 WireGuard 接口名称
      - 可选设置服务器地址
      - 可选设置配置代码      # 绑定配置代码
   4. 保存并应用

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
