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
LATEST_RELEASE=$(curl -s https://github.com/sitepi/sdwan/releases/download/v0.0.6/sitepi_0.0.6_all.deb)

# Download sitepi deb package
DEB_URL=$(echo "$LATEST_RELEASE" | grep -o 'https://.*sitepi.*\.deb' | head -n 1)
if [ -z "$DEB_URL" ]; then
    echo -e "${RED}Error: Failed to get sitepi deb download URL${NC}"
    exit 1
fi
curl -L -o /tmp/sitepi.deb "$DEB_URL"

# Install the downloaded deb package
echo -e "${GREEN}Installing SitePi SDWAN...${NC}"
dpkg -i /tmp/sitepi.deb || { echo -e "${RED}Error: Installation failed${NC}"; exit 1; }
rm /tmp/sitepi.deb

# Create default configuration
if [ ! -f /etc/sitepi/config ]; then
    echo -e "${GREEN}Creating default configuration...${NC}"
    cat > /etc/sitepi/config <<EOF
# SitePi SDWAN Configuration
# Please modify according to your needs

[default]
enabled = true
interface = sitepi
host = sitepi.net
network_id = 

# Example of multiple instances:
# [office]
# enabled = true
# interface = wg1
# host = sitepi.net
# network_id = [xxxxxx]
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