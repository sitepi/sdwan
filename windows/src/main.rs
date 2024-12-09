use ipnet::{IpNet, Ipv4Net};
use rand::Rng;
use std::io::BufRead;
use std::net::Ipv4Addr;

use std::sync::Arc;
// use ipnet::{Ipv4Net};
use base64::Engine;
use clap::Parser;
use std::sync::Mutex;

use winapi::shared::ipmib::MIB_IPFORWARDROW;
use winapi::um::iphlpapi::{
    CreateIpForwardEntry, DeleteIpForwardEntry,
};
// Add command line arguments struct
#[derive(Parser)]
#[command(name = "sitepi")]
#[command(about = "SitePi SD-WAN Client (0.0.8)", long_about = None)]
struct Cli {
    /// Server address
    #[arg(short = 's', long = "server", default_value = "https://sitepi.cn")]
    server: String,

    /// WireGuard interface name
    #[arg(short = 'i', long = "interface", required = true)]
    interface: String,

    /// Provision code
    #[arg(short = 'p', long = "provision")]
    provision: Option<String>,

    /// Route auto load
    #[arg(short = 'r', long = "route")]
    route: Option<bool>,
}

static PEERS: Mutex<Vec<wireguard_nt::SetPeer>> = Mutex::new(Vec::new());

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Cli::parse();

    let _server = args.server;
    let interface = args.interface;
    let provision_code = args.provision.clone(); // Use clone() to create a new copy
    let route = args.route.unwrap_or(false);

    // Use provision code (if provided)
    if let Some(ref provision_code) = args.provision {
        // Use ref to borrow the value
        println!("provision code: {}", provision_code);
        // TODO: Handle the logic for provision code
    }

    // Replace signal handling related code
    let exit = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let exit_clone = Arc::clone(&exit);

    ctrlc::set_handler(move || {
        println!("Received exit signal, shutting down...");
        exit_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        std::process::exit(0);
    })?;

    // Must be run as Administrator because we create network adapters

    // Load the wireguard dll file so that we can call the underlying C functions
    // Unsafe because we are loading an arbitrary dll file
    let wireguard = unsafe { wireguard_nt::load_from_path("wireguard.dll") }
        .expect("Failed to load wireguard dll");

    // tbd: Check if we have administrator privileges

    // Use interface name from command line arguments
    let adapter = Arc::new(match wireguard_nt::Adapter::open(&wireguard, &interface) {
        Ok(adapter) => adapter,
        Err(_) => {
            println!("{} not found, initialize it", interface);
            match wireguard_nt::Adapter::create(&wireguard, &interface, "Demo", None) {
                Ok(adapter) => {
                    let private = x25519_dalek::StaticSecret::random();
                    let mut private_bytes = [0; 32];
                    private_bytes.copy_from_slice(private.as_bytes());
                    let mut port: u16 = 0;

                    // Check if the configuration file exists for the interface
                    let config_path = format!("configs/{}.conf", interface);
                    if std::path::Path::new(&config_path).exists() {
                        println!(
                            "Reading the existing configuration file: {}.conf",
                            interface
                        );
                        // Read the existing configuration
                        let config_content = std::fs::read_to_string(&config_path)
                            .expect("Failed to read configuration file");
                        // Parse the private key and port from the configuration
                        for line in config_content.lines() {
                            if line.starts_with("PrivateKey") {
                                let parts: Vec<&str> = line.split('=').collect();
                                if parts.len() >= 2 {
                                    let private_key = parts[1].trim().to_owned() + "=";

                                    match base64::engine::general_purpose::STANDARD
                                        .decode(private_key)
                                    {
                                        Ok(decoded_key) => {
                                            if decoded_key.len() == 32 {
                                                private_bytes.copy_from_slice(&decoded_key);
                                            } else {
                                                eprintln!("Error: Private key length is incorrect, should be 32 bytes, but got {} bytes", decoded_key.len());
                                                std::process::exit(1);
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Error: Failed to decode private key from base64: {}", e);
                                            std::process::exit(1);
                                        }
                                    }
                                }
                            } else if line.starts_with("ListenPort") {
                                let parts: Vec<&str> = line.split('=').collect();
                                if parts.len() >= 2 {
                                    port = parts[1].trim().parse().expect("Invalid port number");
                                }
                            }
                        }
                    } else {
                        println!("no {}.conf found, create it", interface);
                        // Generate a random port number between 1024 and 65535
                        port = rand::thread_rng().gen_range(1024..65535);
                        let new_private_key =
                            base64::engine::general_purpose::STANDARD.encode(&private_bytes);
                        let new_config = format!(
                            "[Interface]\nPrivateKey = {}\nListenPort = {}\n",
                            new_private_key, port
                        );
                        std::fs::write(&config_path, new_config)
                            .expect("Failed to write configuration file");
                    }

                    let interface = wireguard_nt::SetInterface {
                        listen_port: Some(port),
                        public_key: None,
                        private_key: Some(private_bytes),
                        peers: vec![],
                    };
                    adapter.set_config(&interface).unwrap();
                    adapter
                }
                Err(e) => {
                    eprintln!("Failed to create WireGuard adapter: {}", e);
                    std::process::exit(1);
                }
            }
        }
    });

    assert!(adapter.set_logging(wireguard_nt::AdapterLoggingLevel::OnWithPrefix));

    let config = adapter.get_config();
    println!(
        " public_key: {}",
        base64::engine::general_purpose::STANDARD.encode(config.public_key)
    );
    println!("listen_port: {}", config.listen_port);

    let mut attempt = 0; // Initialize attempt counter
    let max_attempts = 5; // Set maximum attempts
    let mut base_delay = 1; // Base delay in seconds
                            // Directly call sync function, no runtime needed
    loop {
        while attempt < max_attempts {
            let one_shot = rand::thread_rng().gen_range(800..1200);
            std::thread::sleep(std::time::Duration::from_millis(base_delay * one_shot)); // Sleep for base delay

            // If the operation is successful, break the loop
            let _ = do_authorize(
                &_server,
                Some(config.public_key),
                Some(config.listen_port),
                provision_code.clone(),
                route,
                &adapter,
            );

            // Increase the base delay for the next attempt
            base_delay *= 2; // Exponential backoff
            attempt += 1; // Increment attempt counter
        }

        attempt = 0;
        base_delay = 1;

        if exit.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
    }

    Ok(())
}

// Change async function to sync function
fn do_authorize(
    host: &str,
    pubkey: Option<[u8; 32]>,
    listen_port: Option<u16>,
    provision_code: Option<String>,
    route: bool,
    adapter: &Arc<wireguard_nt::Adapter>,
) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = if host.starts_with("http") {
        format!("{}/authorize", host)
    } else {
        format!("{}/authorize", host)
    };

    println!("== Authorizing..");

    let mut request = client.post(url).header("User-Agent", "sitepi");

    // If there is a public key, add it to the request header
    if let Some(key) = pubkey {
        request = request.header(
            "PUBKEY",
            base64::engine::general_purpose::STANDARD.encode(key),
        );
    }

    // If there is a listen port, add it to the request header
    if let Some(port) = listen_port {
        request = request.header("LISTEN-PORT", port.to_string());
    }

    // If there is a provision code, add it to the request header
    if let Some(code) = provision_code {
        request = request.header("PROVISION-CODE", code);
    }

    let response = request.send()?;

    if response.status().is_success() {
        let x_session = response
            .headers()
            .get("x-session")
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let x_network = response
            .headers()
            .get("x-network")
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let x_ipaddr = response
            .headers()
            .get("x-ipaddr")
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let x_url = response
            .headers()
            .get("x-url")
            .and_then(|h| h.to_str().ok())
            .map(String::from);
        let x_proxy = response
            .headers()
            .get("x-proxy")
            .and_then(|h| h.to_str().ok())
            .map(String::from);

        println!("  next URL: {}", x_url.as_ref().unwrap_or(&String::new()));
        println!("next PROXY: {}", x_proxy.as_ref().unwrap_or(&String::new()));
        println!(
            "   NETWORK: {}",
            x_network.as_ref().unwrap_or(&String::new())
        );
        println!(
            "    IPADDR: {}",
            x_ipaddr.as_ref().unwrap_or(&String::new())
        );

        // Define set_ip as a local closure
        let set_ip = |ipaddr: Option<String>| {
            if let Some(ipaddr) = ipaddr {
                if let Ok(ip_addr) = ipaddr.parse::<Ipv4Addr>() {
                    // Create the network from the parsed IP
                    let ipnet = Ipv4Net::new(ip_addr, 24).unwrap();

                    let configs = wireguard_nt::SetInterface {
                        listen_port: None,
                        public_key: None,
                        private_key: None,
                        peers: vec![],
                    };

                    // Directly use ipnet, no additional conversion needed
                    match adapter.set_default_route(&[ipnet.into()], &configs) {
                        Ok(()) => {
                            // println!(" set address success: {}", ipaddr);
                        }
                        Err(err) => panic!("Failed to set address: {}", err),
                    }

                    assert!(adapter.up().is_ok());
                }
            }
        };

        set_ip(x_ipaddr);
        do_connect(x_session, x_url, x_proxy, route, adapter);
        Ok(())
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

fn do_connect(
    x_session: Option<String>,
    x_url: Option<String>,
    x_proxy: Option<String>,
    route: bool,
    adapter: &Arc<wireguard_nt::Adapter>,
) {
    // Add adapter parameter

    if let Some(url) = x_url {
        println!("== connect to {}", url);
        let client = if let Some(proxy) = x_proxy {
            let proxy = reqwest::Proxy::all(proxy).expect("Invalid proxy URL");
            reqwest::blocking::Client::builder()
                .proxy(proxy)
                .timeout(None)
                .tcp_keepalive(Some(std::time::Duration::from_secs(24)))
                .build()
                .expect("Failed to create client with proxy")
        } else {
            reqwest::blocking::Client::builder()
                .timeout(None)
                .tcp_keepalive(Some(std::time::Duration::from_secs(24)))
                .build()
                .expect("Failed to create client")
        };

        let mut request = client.get(&url).header("User-Agent", "sitepi");

        if let Some(session) = x_session {
            request = request.header("X-Session", session);
        }

        // Send request and handle streaming response
        match request.send() {
            Ok(response) => {
                if response.status().is_success() {
                    // Change: declare reader as mutable
                    let mut reader = std::io::BufReader::new(response);
                    let mut line = String::new();

                    // Continuously read lines from the stream
                    loop {
                        line.clear();
                        match reader.read_line(&mut line) {
                            Ok(n) if n > 0 => {
                                // println!("Read a line: {:?}", line);
                                let message = line.trim_end();
                                // Process each line (remove trailing newline) and handle it
                                //if let Some(message) = line.trim_end().is_empty().then(|| line.trim_end()) {
                                handle_message(message, route, adapter);
                                //}
                            }
                            Ok(_) => {
                                println!("Connection closed");
                                break;
                            }
                            Err(e) => {
                                println!("Read error: {:?}", e);
                                break;
                            }
                        }
                    }
                } else {
                    println!("Connection failed: {:?}", response.status());
                }
            }
            Err(err) => {
                println!("Request error: {:?}", err);
            }
        }
    } else {
        println!(" Invalid URL: {:?}", x_url);
    }
}

fn handle_message(message: &str, route: bool, adapter: &Arc<wireguard_nt::Adapter>) {
    let data: Vec<&str> = message.split_whitespace().collect();
    // println!("Split data: {:?}", data);

    // Extract action and public key from the data
    let action = data[0];
    let pubkey = data[1];

    // Check if the action is "wg" and the data length is 6
    if action == "wg" && data.len() == 6 {
        // Initialize endpoint, IP address, and keepalive
        let mut endpoint = data[3];
        let ip_str = data[4]; // IP address
        let mut keepalive = data[5];

        if endpoint == "x" {
            endpoint = "127.0.0.1:11820";
            keepalive = "0";
        }

        // If the IP is not 'x', create an IpNet
        // let ip_net = IpNet::new(ip_str.parse().unwrap(), 32).unwrap();
        let ips = if ip_str != "x" {
            vec![IpNet::new(ip_str.parse().unwrap(), 32).unwrap()]
        } else {
            vec![]
        };

        let peer = wireguard_nt::SetPeer {
            public_key: Some(
                base64::engine::general_purpose::STANDARD
                    .decode(pubkey)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
            preshared_key: None,
            keep_alive: Some(keepalive.parse().unwrap()),
            allowed_ips: ips.clone(),
            endpoint: endpoint.parse().unwrap(),
        };

        println!(" peer: {}", pubkey);

        // Safely modify PEERS using Mutex
        let mut peers = PEERS.lock().unwrap();
        if !peers.iter().any(|p| p.public_key == peer.public_key) {
            peers.push(peer);
        } else {
            peers.retain(|p| p.public_key != peer.public_key);
            peers.push(peer);
        }

        // Clone peers when creating the interface configuration
        let interface = wireguard_nt::SetInterface {
            listen_port: None,
            public_key: None,
            private_key: None,
            peers: peers.clone(),
        };

        // Set the config our adapter will use
        // This lets it know about the peers and keys
        adapter.set_config(&interface).unwrap();

        if route && ips.len() > 1 {
            // The first in ips is the peer IP
            let peer_ip = ips[0];

            // Extract the IPv4 address from peer_ip IpNet
            if let IpNet::V4(peer_net) = peer_ip {
                let peer_addr = peer_net.addr();  // Convert to Ipv4Addr

                // Iterate over allowed_ips and add routes
                for dest in &ips {
                    if dest != &peer_ip {
                        match dest {
                            IpNet::V4(dest_net) => {
                                println!(" add route for IP: {} via {}", dest_net, peer_addr);
                                match add_windows_route(*dest_net, peer_addr) {
                                    Ok(()) => println!("Successfully added route for IP: {}", dest_net),
                                    Err(error_code) => println!(
                                        "Failed to add route for IP: {} with error code: {}",
                                        dest_net, error_code
                                    ),
                                }
                            }
                            IpNet::V6(_) => continue, // Skip IPv6
                        }
                    }
                }
            }
        }
    }
}

// Add new function
fn add_windows_route(dest_net: Ipv4Net, next_hop: Ipv4Addr) -> Result<(), i32> {
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
        dwForwardPolicy: 0,     // Default policy
        ForwardType: 4,         // 4 represents a remote route
        ForwardProto: 3,        // 3 represents a static route
    };

    // Add the route
    let result = unsafe { CreateIpForwardEntry(&mut route_entry) };
    if result != 0 {
        Err(result as i32)
    } else {
        Ok(())
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
        dwForwardPolicy: 0,     // Default policy
        ForwardType: 4,         // 4 represents a remote route
        ForwardProto: 3,        // 3 represents a static route
    };

    // Delete the route
    let result = unsafe { DeleteIpForwardEntry(&mut route_entry) };
    if result != 0 {
        Err(result as i32)
    } else {
        Ok(())
    }
}
