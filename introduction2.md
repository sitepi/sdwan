# Sitepi SDWAN Introduction

## Overview

Sitepi SDWAN is a modern software-defined WAN solution that leverages WireGuard for secure and efficient networking. It's designed to provide enterprise-grade connectivity with simplicity and performance in mind.

## Technical Architecture

### Components

1. **Control Plane**
   - Central management server
   - Configuration distribution
   - Network topology management
   - Authentication and authorization

2. **Data Plane**
   - WireGuard tunnels
   - Dynamic routing
   - Traffic shaping
   - Link monitoring

3. **Client Components**
   - WireGuard interface management
   - Connection monitoring
   - Auto-healing capabilities
   - Local policy enforcement

### Network Flow

```
[Client] <-> [WireGuard Tunnel] <-> [Sitepi Server] <-> [WireGuard Tunnel] <-> [Client]
     ^                                     ^                                        ^
     |                                     |                                        |
[Local Policy]                    [Global Policy & Routes]                   [Local Policy]
```

## Protocol Details

### 1. Connection Establishment
```
Client                     Server
  |                          |
  |-- Authorization    ----->|
  |<-- Session Info   -------|
  |                          |
  |== WireGuard Tunnel ======|
  |                          |
  |-- SSE Connection  ----->|
  |<-- Peer Updates   -------|
```

### 2. Peer Discovery
- Dynamic peer information exchange
- Automatic endpoint updates
- NAT traversal support
- Keep-alive optimization

### 3. Traffic Management
- Smart routing based on network conditions
- QoS policy enforcement
- Bandwidth aggregation
- Application-aware routing

## Deployment Models

### 1. Hub-and-Spoke
```
                [Central Hub]
                     |
        +-----+-----+-----+-----+
        |     |     |     |     |
    [Spoke1] [S2]  [S3]  [S4] [S5]
```

Ideal for:
- Branch office connectivity
- Centralized management
- Cloud access optimization

### 2. Mesh Network
```
    [Node1] -------- [Node2]
      |    \         /    |
      |     \       /     |
      |      [Core]       |
      |     /       \     |
      |    /         \    |
    [Node4] -------- [Node3]
```

Ideal for:
- Peer-to-peer applications
- Distributed computing
- High availability requirements

### 3. Hybrid Model
```
                [Cloud Services]
                      |
                [Core Router]
                   /    \
            [Branch1]  [Branch2]
              /           \
        [Endpoints]    [Endpoints]
```

Ideal for:
- Mixed environment
- Cloud and on-premise integration
- Flexible scaling

## Security Features

1. **Zero Trust Architecture**
   - Per-device authentication
   - Dynamic key management
   - Encrypted control plane
   - Secure peer discovery

2. **Network Isolation**
   - Segment-based routing
   - Policy-based access control
   - Traffic filtering
   - VLAN support

3. **Monitoring & Auditing**
   - Real-time connection status
   - Traffic analytics
   - Security event logging
   - Performance metrics

## Performance Optimization

1. **Smart Routing**
   - Path MTU discovery
   - Latency-based routing
   - Congestion avoidance
   - Load balancing

2. **Link Management**
   - Multi-link aggregation
   - Automatic failover
   - Link quality monitoring
   - Bandwidth optimization

## Integration

### 1. API Support
- RESTful API for management
- Webhook notifications
- Custom script integration
- Monitoring system integration

### 2. Authentication
- Local authentication
- LDAP/AD integration
- OAuth2 support
- Certificate-based auth

## Getting Started

1. **Planning**
   - Network topology design
   - IP addressing scheme
   - Security requirements
   - Performance goals

2. **Implementation**
   - Server deployment
   - Client installation
   - Initial configuration
   - Testing and validation

3. **Management**
   - Monitoring setup
   - Backup configuration
   - Update strategy
   - Support process

## Best Practices

1. **Network Design**
   - Proper subnet planning
   - Redundant connectivity
   - Security zoning
   - Scalability considerations

2. **Operation**
   - Regular monitoring
   - Performance tuning
   - Security updates
   - Documentation

3. **Troubleshooting**
   - Log analysis
   - Network diagnostics
   - Performance testing
   - Issue resolution

## Support Resources

- Documentation: https://docs.sitepi.net
- Community Forum: https://community.sitepi.net
- GitHub Repository: https://github.com/sitepi/sdwan
- Email Support: support@sitepi.net