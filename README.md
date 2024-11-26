# SitePi SDWAN Client

A lightweight and efficient Software-Defined Wide Area Network (SD-WAN) client implementation.

## Get Network ID
Visit https://sitepi.net/ to register an account, create a network, and obtain your network ID

## Requirements

- Linux/OpenWrt
- Network interface with root/admin privileges

### Ubuntu
```bash
sudo apt update
sudo apt install -y wireguard-tools curl

wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/sitepi_0.1.0_all.deb
sudo dpkg -i sitepi_0.1.0_all.deb

sudo systemctl enable sitepi.service
```

#### Configuration
```bash
sudo vim /etc/sitepi/config.json # edit configuration, binding your network ID

sudo service sitepi {status|start|stop|restart}
```

### OpenWrt
#### Download the packages and install
```bash
cd /tmp
wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/sitepi_0.1.0_all.ipk
wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/luci-app-sitepi_0.1.0_all.ipk

opkg install sitepi_0.1.0_all.ipk
opkg install luci-app-sitepi_0.1.0_all.ipk
```

- The architecture is platform-independent. All routers use the same ipk.

#### Configuration
   1. Go to LuCI web interface
   2. Navigate to Services -> Sitepi SDWAN
   3. Configure:
      - Enable the service
      - Set WireGuard interface name
      - Optionally set server address
      - Optionally set network ID      # binding your network ID
   4. Save & Apply

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
