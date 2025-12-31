use anyhow::{Context, Result};
use ipnet::{IpNet, Ipv4Net};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufRead;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use clap::Parser;
use std::sync::RwLock;

use winapi::shared::ipmib::MIB_IPFORWARDROW;
use winapi::um::iphlpapi::{CreateIpForwardEntry, DeleteIpForwardEntry};

// Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    server: String,
    interface: String,
    provision_code: Option<String>,
    auto_route: bool,
    #[serde(default = "default_log_level")]
    log_level: String,
    #[serde(default)]
    reconnect_config: ReconnectConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReconnectConfig {
    #[serde(default = "default_max_attempts")]
    max_attempts: u32,
    #[serde(default = "default_base_delay")]
    base_delay_ms: u64,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            max_attempts: default_max_attempts(),
            base_delay_ms: default_base_delay(),
        }
    }
}

fn default_max_attempts() -> u32 { 5 }
fn default_base_delay() -> u64 { 1000 }
fn default_log_level() -> String { "info".to_string() }

// Command line arguments
#[derive(Parser)]
#[command(name = "sitepi")]
#[command(about = "SitePi SD-WAN Client (0.0.9)", long_about = None)]
struct Cli {
    /// Server address
    #[arg(short = 's', long = "server")]
    server: Option<String>,

    /// WireGuard interface name
    #[arg(short = 'i', long = "interface")]
    interface: Option<String>,

    /// Provision code
    #[arg(short = 'p', long = "provision")]
    provision: Option<String>,

    /// Route auto load
    #[arg(short = 'r', long = "route")]
    route: Option<bool>,

    /// Configuration file path
    #[arg(short = 'c', long = "config", default_value = "config.json")]
    config_file: PathBuf,

    /// Install as Windows service
    #[arg(long = "install-service")]
    install_service: bool,

    /// Uninstall Windows service
    #[arg(long = "uninstall-service")]
    uninstall_service: bool,
}

// Peer state management
struct PeerManager {
    peers: RwLock<Vec<wireguard_nt::SetPeer>>,
    routes: RwLock<HashMap<String, Vec<Ipv4Net>>>,
}

impl PeerManager {
    fn new() -> Self {
        Self {
            peers: RwLock::new(Vec::new()),
            routes: RwLock::new(HashMap::new()),
        }
    }

    fn add_or_update_peer(&self, peer: wireguard_nt::SetPeer) -> Result<()> {
        let mut peers = self.peers.write().unwrap();
        
        if let Some(existing) = peers.iter_mut().find(|p| p.public_key == peer.public_key) {
            *existing = peer;
            debug!("Updated existing peer");
        } else {
            peers.push(peer);
            debug!("Added new peer");
        }
        
        Ok(())
    }

    fn get_peers(&self) -> Vec<wireguard_nt::SetPeer> {
        self.peers.read().unwrap().clone()
    }

    fn clear_routes(&self, public_key: &str) {
        let mut routes = self.routes.write().unwrap();
        if let Some(old_routes) = routes.remove(public_key) {
            for route in old_routes {
                if let IpNet::V4(dest_net) = IpNet::from(route) {
                    // Best effort cleanup
                    let _ = del_windows_route(dest_net, route.network());
                }
            }
        }
    }

    fn add_route(&self, public_key: String, route: Ipv4Net) {
        let mut routes = self.routes.write().unwrap();
        routes.entry(public_key).or_insert_with(Vec::new).push(route);
    }
}


fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::parse();

    // Load configuration
    let config = load_config(&args)?;

    // Initialize logging
    init_logging(&config.log_level)?;

    info!("SitePi SDWAN Client v0.0.9 starting");
    info!("Server: {}", config.server);
    info!("Interface: {}", config.interface);

    // Handle service installation/uninstallation
    if args.install_service {
        info!("Installing Windows service...");
        // TODO: Implement service installation
        return Ok(());
    }

    if args.uninstall_service {
        info!("Uninstalling Windows service...");
        // TODO: Implement service uninstallation
        return Ok(());
    }

    // Run the client
    run_client(config)
}

fn load_config(args: &Cli) -> Result<Config> {
    // Try to load from file first
    let config = if args.config_file.exists() {
        info!("Loading configuration from {:?}", args.config_file);
        let content = std::fs::read_to_string(&args.config_file)
            .context("Failed to read configuration file")?;
        serde_json::from_str::<Config>(&content)
            .context("Failed to parse configuration file")?
    } else {
        Config {
            server: "https://sitepi.cn".to_string(),
            interface: String::new(),
            provision_code: None,
            auto_route: false,
            log_level: default_log_level(),
            reconnect_config: ReconnectConfig::default(),
        }
    };

    // Override with command line arguments
    let config = Config {
        server: args.server.clone().unwrap_or(config.server),
        interface: args.interface.clone().unwrap_or(config.interface),
        provision_code: args.provision.clone().or(config.provision_code),
        auto_route: args.route.unwrap_or(config.auto_route),
        log_level: config.log_level,
        reconnect_config: config.reconnect_config,
    };

    // Validate configuration
    if config.interface.is_empty() {
        anyhow::bail!("Interface name is required. Use --interface or set it in config file.");
    }

    Ok(config)
}

fn init_logging(log_level: &str) -> Result<()> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_ansi(false)
        .init();

    Ok(())
}

fn run_client(config: Config) -> Result<()> {
    // Setup signal handling
    let exit = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let exit_clone = Arc::clone(&exit);

    ctrlc::set_handler(move || {
        if !exit_clone.load(std::sync::atomic::Ordering::Relaxed) {
            info!("Received exit signal, shutting down...");
            exit_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    })?;

    // Load WireGuard DLL
    let wireguard = unsafe { wireguard_nt::load_from_path("wireguard.dll") }
        .context("Failed to load wireguard.dll - ensure WireGuard is installed")?;

    // Get or create adapter
    let adapter = Arc::new(get_or_create_adapter(&wireguard, &config)?);

    // Enable logging
    adapter.set_logging(wireguard_nt::AdapterLoggingLevel::OnWithPrefix);

    // Get adapter configuration
    let adapter_config = adapter.get_config();
    info!("Public key: {}", BASE64.encode(adapter_config.public_key));
    info!("Listen port: {}", adapter_config.listen_port);

    // Create peer manager
    let peer_manager = Arc::new(PeerManager::new());

    // Main connection loop
    let mut attempt = 0;
    let max_attempts = config.reconnect_config.max_attempts;
    let mut base_delay = config.reconnect_config.base_delay_ms;

    loop {
        if exit.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        while attempt < max_attempts {
            if exit.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let jitter = rand::thread_rng().gen_range(800..1200);
            let delay = Duration::from_millis(base_delay * jitter / 1000);
            std::thread::sleep(delay);

            match do_authorize(
                &config,
                Some(adapter_config.public_key),
                Some(adapter_config.listen_port),
                &adapter,
                &peer_manager,
            ) {
                Ok(_) => {
                    info!("Authorization successful");
                    attempt = 0;
                    base_delay = config.reconnect_config.base_delay_ms;
                }
                Err(e) => {
                    error!("Authorization failed: {:#}", e);
                    attempt += 1;
                    base_delay *= 2;
                    
                    // Check if we should exit
                    if exit.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }
                }
            }
        }

        if exit.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        attempt = 0;
        base_delay = config.reconnect_config.base_delay_ms;
    }

    info!("Client shutting down");
    Ok(())
}

fn get_or_create_adapter(
    wireguard: &wireguard_nt::Wireguard,
    config: &Config,
) -> Result<wireguard_nt::Adapter> {
    match wireguard_nt::Adapter::open(wireguard, &config.interface) {
        Ok(adapter) => {
            info!("Opened existing adapter: {}", config.interface);
            Ok(adapter)
        }
        Err(_) => {
            info!("Creating new adapter: {}", config.interface);
            let adapter = wireguard_nt::Adapter::create(
                wireguard,
                &config.interface,
                "SitePi SDWAN",
                None,
            )
            .context("Failed to create WireGuard adapter")?;

            // Load or generate keys
            let (private_key, port) = load_or_generate_keys(&config.interface)?;

            let interface = wireguard_nt::SetInterface {
                listen_port: Some(port),
                public_key: None,
                private_key: Some(private_key),
                peers: vec![],
            };

            adapter.set_config(&interface)
                .context("Failed to set adapter configuration")?;

            Ok(adapter)
        }
    }
}

fn load_or_generate_keys(interface_name: &str) -> Result<([u8; 32], u16)> {
    let config_dir = PathBuf::from("configs");
    std::fs::create_dir_all(&config_dir)?;
    
    let config_path = config_dir.join(format!("{}.conf", interface_name));

    if config_path.exists() {
        info!("Loading existing configuration from {:?}", config_path);
        let content = std::fs::read_to_string(&config_path)?;
        
        let mut private_key = None;
        let mut port = None;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("PrivateKey") {
                if let Some(key_str) = line.split('=').nth(1) {
                    let key_str = key_str.trim().to_string() + "=";
                    let decoded = BASE64.decode(&key_str)
                        .context("Failed to decode private key")?;
                    if decoded.len() == 32 {
                        let mut key_bytes = [0u8; 32];
                        key_bytes.copy_from_slice(&decoded);
                        private_key = Some(key_bytes);
                    }
                }
            } else if line.starts_with("ListenPort") {
                if let Some(port_str) = line.split('=').nth(1) {
                    port = Some(port_str.trim().parse()
                        .context("Invalid port number")?);
                }
            }
        }

        match (private_key, port) {
            (Some(key), Some(p)) => {
                info!("Loaded existing keys, port: {}", p);
                return Ok((key, p));
            }
            _ => warn!("Configuration incomplete, generating new keys"),
        }
    }

    // Generate new keys
    info!("Generating new private key and port");
    let private = x25519_dalek::StaticSecret::random();
    let mut private_bytes = [0u8; 32];
    private_bytes.copy_from_slice(private.as_bytes());
    
    let port = rand::thread_rng().gen_range(1024..65535);
    
    let config_content = format!(
        "[Interface]\nPrivateKey = {}\nListenPort = {}\n",
        BASE64.encode(&private_bytes),
        port
    );
    
    std::fs::write(&config_path, config_content)
        .context("Failed to save configuration")?;
    
    info!("Saved new configuration to {:?}", config_path);
    Ok((private_bytes, port))
}

fn do_authorize(
    config: &Config,
    pubkey: Option<[u8; 32]>,
    listen_port: Option<u16>,
    adapter: &Arc<wireguard_nt::Adapter>,
    peer_manager: &Arc<PeerManager>,
) -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let url = format!("{}/authorize", config.server);
    info!("Authorizing with server: {}", url);

    let mut request = client.post(&url).header("User-Agent", "sitepi/0.0.9");

    if let Some(key) = pubkey {
        request = request.header("PUBKEY", BASE64.encode(key));
    }
    if let Some(port) = listen_port {
        request = request.header("LISTEN-PORT", port.to_string());
    }
    if let Some(ref code) = config.provision_code {
        request = request.header("PROVISION-CODE", code);
    }

    let response = request.send()
        .context("Failed to send authorization request")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().unwrap_or_default();
        
        if status.as_u16() == 403 {
            error!("Authorization denied. This may be due to:");
            error!("  1. Public key not registered with the network");
            error!("  2. Missing or invalid provision code");
            error!("  3. Network access restrictions");
            error!("");
            error!("Solutions:");
            error!("  - Use --provision option with a valid provision code");
            error!("  - Register your public key on the server: https://sitepi.cn");
            error!("  - Public Key: {}", BASE64.encode(pubkey.unwrap_or([0u8; 32])));
            if !error_body.is_empty() {
                error!("Server response: {}", error_body);
            }
        }
        
        anyhow::bail!("Authorization failed with status: {} - {}", status, error_body);
    }

    let headers = response.headers();
    let session = headers.get("x-session")
        .and_then(|h| h.to_str().ok())
        .map(String::from);
    let network = headers.get("x-network")
        .and_then(|h| h.to_str().ok())
        .map(String::from);
    let ipaddr = headers.get("x-ipaddr")
        .and_then(|h| h.to_str().ok())
        .map(String::from);
    let url = headers.get("x-url")
        .and_then(|h| h.to_str().ok())
        .map(String::from);
    let proxy = headers.get("x-proxy")
        .and_then(|h| h.to_str().ok())
        .map(String::from);

    info!("Network: {}", network.as_deref().unwrap_or("N/A"));
    info!("IP Address: {}", ipaddr.as_deref().unwrap_or("N/A"));
    info!("Connect URL: {}", url.as_deref().unwrap_or("N/A"));
    if proxy.is_some() {
        info!("Proxy: {}", proxy.as_deref().unwrap_or("N/A"));
    }

    // Set IP address
    if let Some(ref ip_str) = ipaddr {
        set_adapter_ip(adapter, ip_str, peer_manager)?;
    }

    // Connect to server
    let mut attempt = 0;
    let max_attempts = 3;
    let mut delay = 1000u64;

    while attempt < max_attempts {
        std::thread::sleep(Duration::from_millis(delay + rand::thread_rng().gen_range(0..200)));
        
        match do_connect(
            session.clone(),
            url.clone(),
            proxy.clone(),
            config.auto_route,
            adapter,
            peer_manager,
        ) {
            Ok(_) => {
                info!("Connection established successfully");
                return Ok(());
            }
            Err(e) => {
                warn!("Connection attempt {} failed: {:#}", attempt + 1, e);
                attempt += 1;
                delay *= 2;
            }
        }
    }

    anyhow::bail!("Failed to establish connection after {} attempts", max_attempts)
}

fn set_adapter_ip(
    adapter: &Arc<wireguard_nt::Adapter>,
    ip_str: &str,
    peer_manager: &Arc<PeerManager>,
) -> Result<()> {
    let ip_addr = ip_str.parse::<Ipv4Addr>()
        .context("Invalid IP address")?;
    
    let ipnet = Ipv4Net::new(ip_addr, 24)
        .context("Failed to create IP network")?;

    let peers = peer_manager.get_peers();
    let interface = wireguard_nt::SetInterface {
        listen_port: None,
        public_key: None,
        private_key: None,
        peers,
    };

    adapter.set_default_route(&[ipnet.into()], &interface)
        .context("Failed to set adapter IP address")?;

    adapter.up()
        .context("Failed to bring adapter up")?;

    info!("Set adapter IP: {}/24", ip_addr);
    Ok(())
}

fn do_connect(
    session: Option<String>,
    url: Option<String>,
    proxy: Option<String>,
    auto_route: bool,
    adapter: &Arc<wireguard_nt::Adapter>,
    peer_manager: &Arc<PeerManager>,
) -> Result<()> {
    let session = session.ok_or_else(|| anyhow::anyhow!("Missing session"))?;
    let url = url.ok_or_else(|| anyhow::anyhow!("Missing URL"))?;

    info!("Connecting to server stream...");

    let client = if let Some(ref proxy_url) = proxy {
        let proxy = reqwest::Proxy::all(proxy_url)
            .context("Invalid proxy URL")?;
        reqwest::blocking::Client::builder()
            .proxy(proxy)
            .timeout(None)
            .tcp_keepalive(Some(Duration::from_secs(24)))
            .build()?
    } else {
        reqwest::blocking::Client::builder()
            .timeout(None)
            .tcp_keepalive(Some(Duration::from_secs(24)))
            .build()?
    };

    let response = client
        .get(&url)
        .header("User-agent", "sitepi/0.0.9")
        .header("X-Session", session)
        .send()
        .context("Failed to connect to server")?;

    if !response.status().is_success() {
        anyhow::bail!("Connection failed with status: {}", response.status());
    }

    info!("Connected, receiving messages...");

    let mut reader = std::io::BufReader::new(response);
    let mut line = String::new();
    let mut message_count = 0u64;

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                info!("Connection closed by server (received {} messages)", message_count);
                break;
            }
            Ok(_) => {
                message_count += 1;
                let message = line.trim();
                if !message.is_empty() {
                    if let Err(e) = handle_message(message, auto_route, adapter, peer_manager) {
                        error!("Failed to handle message: {:#}", e);
                    }
                }
            }
            Err(e) => {
                error!("Read error: {:#}", e);
                anyhow::bail!("Connection lost: {}", e);
            }
        }
    }

    Ok(())
}


fn handle_message(
    message: &str,
    auto_route: bool,
    adapter: &Arc<wireguard_nt::Adapter>,
    peer_manager: &Arc<PeerManager>,
) -> Result<()> {
    let parts: Vec<&str> = message.split_whitespace().collect();

    if parts.len() < 2 {
        debug!("Ignoring invalid message: {}", message);
        return Ok(());
    }

    let action = parts[0];
    let public_key_str = parts[1];

    if action == "wg" && parts.len() == 6 {
        let endpoint_str = parts[3];
        let ip_str = parts[4];
        let keepalive_str = parts[5];

        // Parse public key
        let public_key = BASE64.decode(public_key_str)
            .context("Failed to decode public key")?;
        let public_key: [u8; 32] = public_key.try_into()
            .map_err(|_| anyhow::anyhow!("Invalid public key length"))?;

        // Parse endpoint
        let endpoint = if endpoint_str == "x" || endpoint_str.is_empty() {
            "0.0.0.0:0".parse().unwrap()
        } else {
            endpoint_str.parse()
                .context("Invalid endpoint address")?
        };

        // Parse keepalive
        let keepalive: u16 = if keepalive_str == "x" || keepalive_str.is_empty() {
            0
        } else {
            keepalive_str.parse()
                .context("Invalid keepalive value")?
        };

        // Parse allowed IPs
        let allowed_ips: Vec<IpNet> = if ip_str != "x" && !ip_str.is_empty() {
            ip_str.split(',')
                .filter_map(|s| {
                    s.trim().parse::<std::net::IpAddr>()
                        .ok()
                        .map(|addr| IpNet::new(addr, 32).unwrap())
                })
                .collect()
        } else {
            vec![]
        };

        let peer = wireguard_nt::SetPeer {
            public_key: Some(public_key),
            preshared_key: None,
            keep_alive: Some(keepalive),
            allowed_ips: allowed_ips.clone(),
            endpoint,
        };

        let ip_display = if !allowed_ips.is_empty() {
            allowed_ips[0].addr().to_string()
        } else {
            "[none]".to_string()
        };

        info!("Update peer: {} | {} | {}", 
            &public_key_str[..16], endpoint, ip_display);

        // Update peer
        peer_manager.add_or_update_peer(peer)?;

        // Apply configuration
        let peers = peer_manager.get_peers();
        let interface = wireguard_nt::SetInterface {
            listen_port: None,
            public_key: None,
            private_key: None,
            peers,
        };

        adapter.set_config(&interface)
            .context("Failed to update adapter configuration")?;

        // Handle routing
        if auto_route && allowed_ips.len() > 1 {
            handle_peer_routes(&allowed_ips, public_key_str, adapter, peer_manager)?;
        }
    }

    Ok(())
}

fn handle_peer_routes(
    allowed_ips: &[IpNet],
    public_key: &str,
    adapter: &Arc<wireguard_nt::Adapter>,
    peer_manager: &Arc<PeerManager>,
) -> Result<()> {
    // Clear old routes for this peer
    peer_manager.clear_routes(public_key);

    // The first IP is the peer's own IP (gateway)
    let gateway = match allowed_ips[0] {
        IpNet::V4(net) => net.addr(),
        IpNet::V6(_) => {
            debug!("IPv6 gateway not supported for routing");
            return Ok(());
        }
    };

    // Get interface index
    let if_index = get_interface_index(adapter)?;

    // Add routes for remaining IPs
    for dest_net in &allowed_ips[1..] {
        match dest_net {
            IpNet::V4(dest) => {
                debug!("Adding route: {} via {} (if={})", dest, gateway, if_index);
                
                match add_windows_route(*dest, gateway, if_index) {
                    Ok(()) => {
                        info!("Added route: {} via {}", dest, gateway);
                        peer_manager.add_route(public_key.to_string(), *dest);
                    }
                    Err(code) => {
                        // Error 5010 means route already exists
                        if code == 5010 {
                            debug!("Route already exists: {}", dest);
                        } else {
                            warn!("Failed to add route {}: error {}", dest, code);
                        }
                    }
                }
            }
            IpNet::V6(_) => {
                debug!("Skipping IPv6 route");
            }
        }
    }

    Ok(())
}

fn get_interface_index(_adapter: &wireguard_nt::Adapter) -> Result<u32> {
    // Try to get the interface index from the adapter's LUID
    // This is a simplified approach - in production you might need more robust handling
    
    // For now, return 0 to let Windows determine the interface automatically
    // A more complete implementation would use ConvertInterfaceLuidToIndex
    Ok(0)
}


// Windows route management functions
fn add_windows_route(dest_net: Ipv4Net, next_hop: Ipv4Addr, if_index: u32) -> Result<(), i32> {
    let mut route_entry = MIB_IPFORWARDROW {
        dwForwardDest: u32::from(dest_net.network()),
        dwForwardMask: u32::from(dest_net.netmask()),
        dwForwardNextHop: u32::from(next_hop),
        dwForwardIfIndex: if_index,
        dwForwardMetric1: 30, // Lower metric for preference
        dwForwardMetric2: 0,
        dwForwardMetric3: 0,
        dwForwardMetric4: 0,
        dwForwardMetric5: 0,
        dwForwardAge: 0,
        dwForwardNextHopAS: 0,
        dwForwardPolicy: 0,
        ForwardType: 4,  // Remote route
        ForwardProto: 3, // Static route
    };

    let result = unsafe { CreateIpForwardEntry(&mut route_entry) };
    
    if result == 0 {
        Ok(())
    } else {
        Err(result as i32)
    }
}

fn del_windows_route(dest_net: Ipv4Net, next_hop: Ipv4Addr) -> Result<(), i32> {
    let mut route_entry = MIB_IPFORWARDROW {
        dwForwardDest: u32::from(dest_net.network()),
        dwForwardMask: u32::from(dest_net.netmask()),
        dwForwardNextHop: u32::from(next_hop),
        dwForwardIfIndex: 0,
        dwForwardMetric1: 0,
        dwForwardMetric2: 0,
        dwForwardMetric3: 0,
        dwForwardMetric4: 0,
        dwForwardMetric5: 0,
        dwForwardAge: 0,
        dwForwardNextHopAS: 0,
        dwForwardPolicy: 0,
        ForwardType: 4,
        ForwardProto: 3,
    };

    let result = unsafe { DeleteIpForwardEntry(&mut route_entry) };
    
    if result == 0 {
        Ok(())
    } else {
        Err(result as i32)
    }
}
