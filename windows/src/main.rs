use ipnet::{IpNet, Ipv4Net};
use rand::Rng;
use std::io::{BufRead};
use std::net::Ipv4Addr;

use std::sync::Arc;
// use ipnet::{Ipv4Net};
use clap::Parser;
use std::sync::Mutex;

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
}


static PEERS: Mutex<Vec<wireguard_nt::SetPeer>> = Mutex::new(Vec::new());

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Cli::parse();

    let _server = args.server;
    let interface = args.interface;
    let provision_code = args.provision.clone(); // Use clone() to create a new copy

    // Use provision code (if provided)
    if let Some(ref provision_code) = args.provision {
        // Use ref to borrow the value
        println!("provision code: {}", provision_code);
        // TODO: Handle the logic for provision code
    }

    // 替换信号处理相关代码
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
                        println!("Reading the existing configuration file: {}.conf", interface);
                        // Read the existing configuration
                        let config_content = std::fs::read_to_string(&config_path)
                            .expect("Failed to read configuration file");
                        // Parse the private key and port from the configuration
                        for line in config_content.lines() {
                            if line.starts_with("PrivateKey") {
                                let parts: Vec<&str> = line.split('=').collect();
                                if parts.len() >= 2 {
                                    let private_key = parts[1].trim();
                                    // Convert the private key from base64
                                    let decoded_key = base64::decode(private_key)
                                        .expect("Invalid base64 private key");
                                    private_bytes.copy_from_slice(&decoded_key);
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
                        let new_private_key = base64::encode(&private_bytes);
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
    println!(" public_key: {}", base64::encode(config.public_key));
    println!("listen_port: {}", config.listen_port);

    // Since the clone method is not found, we will remove the cloning of adapter
    // let adapter2 = Arc::clone(&adapter); // Use the original adapter instead of cloning
                                         // Go to http://demo.wireguard.com/ and see the bandwidth numbers change!
    // println!("Printing peer bandwidth statistics");

    /*
    let done = Arc::new(AtomicBool::new(false));
    let done2 = Arc::clone(&done);

    let thread = std::thread::spawn(move || 'outer: loop {
        let stats = adapter2.get_config();

        for peer in stats.peers {
            let handshake_age = peer
                .last_handshake
                .map(|h| SystemTime::now().duration_since(h).unwrap_or_default());
            let handshake_msg = match handshake_age {
                Some(age) => format!("handshake performed {:.2}s ago", age.as_secs_f32()),
                None => "no active handshake".to_string(),
            };

            println!(
                "  {:?}, {} bytes up, {} bytes down, {handshake_msg}",
                peer.allowed_ips, peer.tx_bytes, peer.rx_bytes
            );
        }
        for _ in 0..10 {
            if done2.load(Ordering::Relaxed) {
                break 'outer;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    });
 */
    // Directly call sync function, no runtime needed
    loop {
        let _ = do_authorize(
            &_server,
            Some(config.public_key),
            Some(config.listen_port),
            provision_code.clone(),
            &adapter,
        );
        std::thread::sleep(std::time::Duration::from_secs(10)); // Sleep for 5 seconds

        if exit.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
    }

    //let mut _buf = [0u8; 32];
    //let _ = std::io::stdin().read(&mut _buf);

    //done.store(true, Ordering::Relaxed);
    //thread.join().unwrap();
    // println!("Exiting!");

    Ok(())
}

// Change async function to sync function
fn do_authorize(
    host: &str,
    pubkey: Option<[u8; 32]>,
    listen_port: Option<u16>,
    provision_code: Option<String>,
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
        request = request.header("PUBKEY", base64::encode(key));
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

        println!(
            "   SESSION: {}",
            x_session.as_ref().unwrap_or(&String::new()));
        println!(
            "  next URL: {}", x_url.as_ref().unwrap_or(&String::new()));
        println!(
            "next PROXY: {}", x_proxy.as_ref().unwrap_or(&String::new()));
        println!(
            "   NETWORK: {}",
            x_network.as_ref().unwrap_or(&String::new()));
        println!(
            "    IPADDR: {}",
            x_ipaddr.as_ref().unwrap_or(&String::new()));

        // Define set_ip as a local closure
        let set_ip = |ipaddr: Option<String>| {
            if let Some(ipaddr) = ipaddr {
                if let Ok(ip_addr) = ipaddr.parse::<Ipv4Addr>() {
                    // Create the network from the parsed IP
                    let ipnet = Ipv4Net::new(ip_addr, 24).unwrap();

                    let interface_config = wireguard_nt::SetInterface {
                        listen_port: None,
                        public_key: None,
                        private_key: None,
                        peers: vec![],
                    };

                    // Directly use ipnet, no additional conversion needed
                    match adapter.set_default_route(&[ipnet.into()], &interface_config) {
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
        do_connect(x_session, x_url, x_proxy, adapter);
        Ok(())
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

fn do_connect(
    x_session: Option<String>,
    x_url: Option<String>,
    x_proxy: Option<String>,
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
                .build()
                .expect("Failed to create client with proxy")
        } else {
            reqwest::blocking::Client::builder()
                .timeout(None)
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
                                handle_message(message, adapter);
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

fn handle_message(message: &str, adapter: &Arc<wireguard_nt::Adapter>) {
    let data: Vec<&str> = message.split_whitespace().collect();
    // println!("Split data: {:?}", data);

    let action = data[0];
    let pubkey = data[1];

    if action == "wg" && data.len() == 6 {
        let mut endpoint = data[3];
        let ip_str = data[4];  // IP地址
        let mut keepalive = data[5];  // 这个参数实际上是prefix/mask

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
            public_key: Some(base64::decode(pubkey).unwrap().try_into().unwrap()),
            preshared_key: None,
            keep_alive: Some(keepalive.parse().unwrap()),
            allowed_ips: ips.clone(),
            endpoint: endpoint.parse().unwrap(),
        };

        println!(" peer: {}", pubkey);

        // 使用 Mutex 安全地修改 PEERS
        let mut peers = PEERS.lock().unwrap();
        if !peers.iter().any(|p| p.public_key == peer.public_key) {
            peers.push(peer);
        } else {
            peers.retain(|p| p.public_key != peer.public_key);
            peers.push(peer);
        }

        // 创建接口配置时克隆 peers
        let interface = wireguard_nt::SetInterface {
            listen_port: None,
            public_key: None,
            private_key: None,
            peers: peers.clone(),
        };
        
        // Set the config our adapter will use
        // This lets it know about the peers and keys
        adapter.set_config(&interface).unwrap();
        
        // Iterate over allowed_ips and add routes
        for ip in &ips { // Use a reference to avoid moving the value
            // Here you would typically call a function to add the route
            // For example: adapter.add_route(ip).unwrap();
            println!(" add route for IP: {}", ip);
        }
    }
}
