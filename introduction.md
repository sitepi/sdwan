# Sitepi SDWAN Introduction

## What is SD-WAN?

Software-Defined Wide Area Network (SD-WAN) is a virtual WAN architecture that allows enterprises to leverage any combination of transport services – including MPLS, LTE and broadband internet services – to securely connect users to applications.

## Key Features

### 1. Zero Trust Security
- WireGuard-based encryption
- Dynamic key rotation
- Peer authentication
- No direct internet exposure

### 2. Smart Routing
- Automatic path selection
- Multi-link aggregation
- QoS-aware routing
- Traffic optimization

### 3. High Availability
- Seamless failover
- Load balancing
- Link health monitoring
- Automatic recovery

### 4. Easy Management
- Web interface
- Command-line tools
- REST API
- Real-time monitoring

## Use Cases

### 1. Branch Office Connectivity
Connect multiple office locations securely:
- Automatic VPN setup
- Centralized management
- Traffic optimization
- Bandwidth aggregation

### 2. Cloud Access
Optimize access to cloud services:
- Direct internet breakout
- Cloud on-ramp
- Application-aware routing
- Secure cloud connectivity

### 3. Remote Work
Enable secure remote work:
- Zero-trust access
- Split tunneling
- Quality of service
- Multiple device support

### 4. IoT/Edge Computing
Connect and manage IoT devices:
- Secure device connectivity
- Edge computing support
- Low-latency routing
- Bandwidth optimization

## Network Architecture

```
                    [Cloud Services]
                           ↑
                           |
                    [Sitepi Server]
                     ↙     ↑    ↘
                    ↙      |     ↘
               [Site A]  [Site B]  [Site C]
                 ↙↘       ↙↘       ↙↘
            [Dev] [Dev] [Dev] [Dev] [Dev] [Dev]
```

## Getting Started

1. Choose your deployment model:
   - Cloud-hosted
   - Self-hosted
   - Hybrid

2. Install the client:
   - On Linux servers
   - On OpenWrt routers
   - On edge devices

3. Configure your network:
   - Set up the server
   - Deploy clients
   - Define routing policies

4. Monitor and manage:
   - Use web interface
   - Check connection status
   - Optimize performance

## Best Practices

1. Security
   - Regular key rotation
   - Access control
   - Network segmentation
   - Audit logging

2. Performance
   - Link monitoring
   - QoS configuration
   - Path optimization
   - Bandwidth management

3. Reliability
   - Redundant links
   - Automatic failover
   - Health checks
   - Backup configurations

4. Management
   - Documentation
   - Monitoring
   - Regular updates
   - Incident response

## Support

For more information and support:
- Documentation: https://docs.sitepi.cn
- Community: https://community.sitepi.cn
- Email: support@sitepi.cn