# SitePi SDWAN Client

[中文版](README.zh-CN.md)

A lightweight and efficient Software-Defined Wide Area Network (SD-WAN) client implementation.

## Create a Network
Visit https://sitepi.net/ to register an account, create a network, and input your site(public key) to bind to the network

## Provisioning Code
Also, you can input the provisioning code of the network when installing the site program, and the site will be automatically bound to the network

## Requirements

- Windows/Linux/OpenWrt
- Network interface with root/admin privileges  

### Ubuntu
```bash
sudo apt update
sudo apt install -y wireguard-tools curl

wget https://github.com/sitepi/sdwan/releases/download/v0.0.6/sitepi_0.0.6_all.deb
sudo dpkg -i sitepi_0.0.6_all.deb

sudo systemctl enable sitepi.service
```

#### Configuration
```bash
sudo vim /etc/sitepi/config.json # optional edit configuration, binding provisioning code

sudo service sitepi {status|start|stop|restart}
```

### OpenWrt
#### Download the packages and install
```bash
cd /tmp
wget https://github.com/sitepi/sdwan/releases/download/v0.0.6/sitepi_0.0.6_all.ipk
wget https://github.com/sitepi/sdwan/releases/download/v0.0.6/luci-app-sitepi_0.0.6_all.ipk

opkg install sitepi_0.0.6_all.ipk
opkg install luci-app-sitepi_0.0.6_all.ipk
```

- The architecture is platform-independent. All routers use the same ipk.

#### Configuration
   1. Go to LuCI web interface
   2. Navigate to Network -> Interfaces
      - Add a new interface (type: WireGuard, name: your choice), press 'Create Interface'
      - General Settings (Generate new key pair)
      - Firewall Settings: (specify the interface to wan)
      - press 'Save & Apply'
   3. Navigate to Services -> Sitepi SDWAN
   4. Configure:
      - Enable the SitePi service
      - Set the WireGuard interface name (the WireGuard interface you just created)
      - Optionally set server address
      - Optionally set provisioning code      # binding provisioning code
   5. Save & Apply

## Features

- WireGuard-based
- Intelligent traffic routing
- QoS (Quality of Service) management
- Real-time network monitoring
- Automatic failover
- Multi-link support
- Zero-touch provisioning

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
