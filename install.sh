#!/bin/bash

# SitePi SDWAN Installer for Ubuntu
# This script will download and install the latest version of SitePi SDWAN

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}Error: Please run as root${NC}"
    exit 1
fi

# Check system requirements
if ! command -v curl >/dev/null 2>&1; then
    echo -e "${YELLOW}Installing curl...${NC}"
    apt-get update && apt-get install -y curl
fi

if ! command -v wg >/dev/null 2>&1; then
    echo -e "${YELLOW}Installing wireguard-tools...${NC}"
    apt-get update && apt-get install -y wireguard-tools
fi

# Create directories
echo -e "${GREEN}Creating directories...${NC}"
install -d -m 755 /etc/sitepi
install -d -m 755 /usr/bin
install -d -m 755 /var/log/sitepi

# Download latest version
echo -e "${GREEN}Downloading SitePi SDWAN...${NC}"
LATEST_RELEASE=$(curl -s https://api.github.com/repos/sitepi-sdwan/sitepi/releases/latest)

# Download sitepi program
SITEPI_URL=$(echo "$LATEST_RELEASE" | grep -o 'https://.*sitepi"' | grep -v 'ubuntu\|install' | sed 's/"$//')
if [ -z "$SITEPI_URL" ]; then
    echo -e "${RED}Error: Failed to get sitepi download URL${NC}"
    exit 1
fi
curl -L -o /usr/bin/sitepi "$SITEPI_URL"
chmod +x /usr/bin/sitepi

# Download sitepi.ubuntu script
UBUNTU_SCRIPT_URL=$(echo "$LATEST_RELEASE" | grep -o 'https://.*sitepi.ubuntu"' | sed 's/"$//')
if [ -z "$UBUNTU_SCRIPT_URL" ]; then
    echo -e "${RED}Error: Failed to get sitepi.ubuntu download URL${NC}"
    exit 1
fi
curl -L -o /usr/bin/sitepi.ubuntu "$UBUNTU_SCRIPT_URL"
chmod +x /usr/bin/sitepi.ubuntu

# Create default configuration
if [ ! -f /etc/sitepi/config ]; then
    echo -e "${GREEN}Creating default configuration...${NC}"
    cat > /etc/sitepi/config <<EOF
# SitePi SDWAN Configuration
# Please modify according to your needs

[default]
enabled = true
interface = wg0
host = sdwan.sitepi.cn
network_id = 

# Example of multiple instances:
# [office]
# enabled = true
# interface = wg1
# host = sdwan.sitepi.cn
# network_id = office
EOF
fi

# Create systemd service
echo -e "${GREEN}Creating systemd service...${NC}"
cat > /etc/systemd/system/sitepi.service <<EOF
[Unit]
Description=SitePi SDWAN Client
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/sitepi.ubuntu start
ExecStop=/usr/bin/sitepi.ubuntu stop
RemainAfterExit=yes
Restart=on-failure
RestartSec=30

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
systemctl daemon-reload

echo -e "${GREEN}Installation completed!${NC}"
echo -e "\nTo configure SitePi SDWAN:"
echo -e "1. Edit configuration: ${YELLOW}nano /etc/sitepi/config${NC}"
echo -e "2. Start service: ${YELLOW}systemctl start sitepi${NC}"
echo -e "3. Enable autostart: ${YELLOW}systemctl enable sitepi${NC}"
echo -e "\nTo manage instances manually:"
echo -e "- Start instance: ${YELLOW}sitepi.ubuntu start [instance]${NC}"
echo -e "- Stop instance: ${YELLOW}sitepi.ubuntu stop [instance]${NC}"
echo -e "- Check status: ${YELLOW}sitepi.ubuntu status [instance]${NC}"
echo -e "- View logs: ${YELLOW}tail -f /var/log/sitepi-*.log${NC}" 