extern crate winres;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    if cfg!(target_os = "windows") || std::env::var("TARGET").unwrap_or_default().contains("windows") {
        let mut res = winres::WindowsResource::new();
        
        if !cfg!(target_os = "windows") {
            // 设置工具链路径
            res.set_ar_path("/usr/bin/x86_64-w64-mingw32-ar");
            res.set_windres_path("/usr/bin/x86_64-w64-mingw32-windres");
            println!("cargo:warning=Set windres path to x86_64-w64-mingw32-windres");
        }
        
        // 设置版本信息
        res.set("FileVersion", "0.0.8.0")
           .set("FileDescription", "Site Pi Client")
           .set("ProductName", "Site Pi")
           .set("OriginalFilename", "sitepi.exe")
           .set("LegalCopyright", "Copyright © 2024")
           .set("CompanyName", "Site Pi");

        // 编译资源
        match res.compile() {
            Ok(_) => println!("cargo:warning=Resource compilation successful"),
            Err(e) => {
                eprintln!("cargo:warning=Resource compilation failed: {}", e);
                std::process::exit(1);
            }
        }
    }
}