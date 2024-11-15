# SDWAN Client

A lightweight and efficient Software-Defined Wide Area Network (SD-WAN) client implementation.

## Features

- Secure VPN connectivity
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
curl https://github.com/sitepi/sdwan/releases/download/v0.1.0/sitepi -o sitepi
chmod +x sitepi
sudo mv sitepi /usr/bin/sitepi

sudo sitepi -i wg0
```

### OpenWrt

#### Method 1: Install from package
1. Download the packages
```bash
wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/sitepi_0.1.0-1_all.ipk
wget https://github.com/sitepi/sdwan/releases/download/v0.1.0/luci-app-sitepi_1.0.0-1_all.ipk
```

2. Install the packages
```bash
opkg install sitepi_0.1.0-1_all.ipk
opkg install luci-app-sitepi_1.0.0-1_all.ipk
```

#### Method 2: Build from source
1. Add feed to OpenWrt
```bash
echo "src-git sitepi https://github.com/sitepi/sdwan.git" >> feeds.conf.default
./scripts/feeds update -a
./scripts/feeds install -a
```

2. Configure and build
```bash
make menuconfig
# Go to Network -> sitepi
# Go to LuCI -> Applications -> luci-app-sitepi
make package/sitepi/compile V=s
make package/luci-app-sitepi/compile V=s
```

The compiled packages will be in `bin/packages/ARCH/base/`.

## Configuration

### Command Line
```bash
sitepi -h sitepi.cn -i wg0 [-n network_id]
```

### OpenWrt Web Interface
1. Go to LuCI web interface
2. Navigate to Services -> Sitepi SDWAN
3. Configure:
   - Enable the service
   - Set server address
   - Set WireGuard interface name
   - Optionally set network ID
4. Save & Apply

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
