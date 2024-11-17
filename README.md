# SDWAN Client

A lightweight and efficient Software-Defined Wide Area Network (SD-WAN) client implementation.

## Features

- WireGuard-based
- Intelligent traffic routing
- QoS (Quality of Service) management
- Real-time network monitoring
- Automatic failover
- Multi-link support
- Zero-touch provisioning

## Requirements

- Linux/OpenWrt
- Network interface with root/admin privileges

## Prerequisites

### Ubuntu
```bash
sudo apt update
sudo apt install -y wireguard-tools
```

### OpenWrt
WireGuard is included by default, no additional installation needed.

## Installation

### Ubuntu/Linux
```bash
wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/sitepi_0.1.0_all.deb
```

- The architecture is platform-independent, not limited to amd64(also supports arm64, MIPS, etc.).

#### Installation
```bash
sudo dpkg -i sitepi_0.1.0_all.deb
sudo apt-get install -f  # Install missing dependencies
```

#### Usage
```bash
sudo service sitepi {status|start|stop|restart}
```
or enable auto-start service
```bash
sudo systemctl enable sitepi.service
```
### Edit configuration
```bash
sudo vim /etc/sitepi/config.json
```

### OpenWrt
1. Download the packages
```bash
wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/sitepi_0.1.0_all.ipk
wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/luci-app-sitepi_0.1.0_all.ipk
```

- The architecture is platform-independent. All routers use the same ipk.

2. Install the packages
```bash
opkg install sitepi_0.1.0_all.ipk
opkg install luci-app-sitepi_0.1.0_all.ipk
```

## Configuration

### Command Line
```bash
sitepi -i wg0 [-n network_id]
```

### OpenWrt Web Interface
1. Go to LuCI web interface
2. Navigate to Services -> Sitepi SDWAN
3. Configure:
   - Enable the service
   - Set WireGuard interface name
   - Optionally set server address
   - Optionally set network ID
4. Save & Apply

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
