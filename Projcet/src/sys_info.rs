use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

pub fn display_universal_system_info() {
    // Detect Linux distribution and architecture and display them together
    let os_info = match detect_linux_distro_and_arch() {
        Ok(info) => info,
        Err(_) => "Unknown OS".to_string(),
    };
    println!("OS: {}", os_info);  // Shortened to "OS:"

    // Detect and display the kernel version
    if let Ok(kernel_version) = detect_kernel_version() {
        // Remove the '200.fc40.x86_64' part from the kernel version for cleaner output
        let cleaned_kernel = kernel_version.split('-').next().unwrap_or(&kernel_version);
        println!("Kernel: {}", cleaned_kernel);
    } else {
        println!("Error detecting Kernel Version.");
    }

    // Detect and display the uptime
    if let Ok(uptime) = detect_uptime() {
        println!("Uptime: {}", uptime);
    } else {
        println!("Error detecting Uptime.");
    }

    // Get and display the hostname
    match sys_info::hostname() {
        Ok(hostname) => println!("Host: {}", hostname),  // Shortened to "Host:"
        Err(e) => eprintln!("Error getting hostname: {}", e),
    }

    // Get and display the total RAM (renamed to Memory)
    match sys_info::mem_info() {
        Ok(mem_info) => println!("Memory: {}MiB / {}MiB", mem_info.free, mem_info.total),
        Err(e) => eprintln!("Error getting memory info: {}", e),
    }

    // Detect and display the display protocol (X11/Wayland)
    if let Ok(display_protocol) = detect_display_protocol() {
        println!("WM: {}", display_protocol);
    } else {
        println!("Error detecting Display Protocol.");
    }

    // Get and display the shell
    if let Ok(shell) = std::env::var("SHELL") {
        println!("Shell: {}", shell);
    } else {
        println!("Error getting Shell.");
    }

    // Get and display the desktop environment (DE)
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        println!("DE: {}", de);
    } else {
        println!("Error getting Desktop Environment.");
    }

    // Get GPU info
    if let Ok(gpu_info) = get_gpu_info() {
        println!("GPU: {}", gpu_info);
    } else {
        println!("Error detecting GPU.");
    }

    // Get CPU info
    if let Ok(cpu_info) = get_cpu_info() {
        println!("CPU: {}", cpu_info);
    } else {
        println!("Error detecting CPU.");
    }

    // Get installed packages
    if let Ok(packages) = detect_installed_packages() {
        println!("Packages: {}", packages);
    } else {
        println!("Error detecting installed packages.");
    }
}

// Function to detect the Linux distribution and architecture
fn detect_linux_distro_and_arch() -> io::Result<String> {
    let distro = detect_linux_distro()?;
    let arch = std::env::consts::ARCH; // Detect system architecture
    Ok(format!("{} {}", distro, arch)) // Format: "Arch Linux x86_64"
}

// Function to detect the Linux distribution
fn detect_linux_distro() -> io::Result<String> {
    let path = "/etc/os-release";
    if Path::new(path).exists() {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("PRETTY_NAME=") {
                // Remove the 'PRETTY_NAME=' prefix and any surrounding quotes
                let distro_name = line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string();
                return Ok(distro_name);
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Could not detect Linux distribution"))
}

// Updated function to detect the kernel version with proper error handling
fn detect_kernel_version() -> io::Result<String> {
    match sys_info::os_release() {
        Ok(kernel_version) => Ok(kernel_version),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("Error detecting kernel version: {}", e))),
    }
}

// Function to detect the uptime
fn detect_uptime() -> io::Result<String> {
    let output = Command::new("uptime")
        .arg("-p")
        .output()?;
    let uptime = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(uptime)
}

// Function to detect the display protocol (X11 or Wayland)
fn detect_display_protocol() -> io::Result<String> {
    if let Ok(wayland) = std::env::var("WAYLAND_DISPLAY") {
        if !wayland.is_empty() {
            return Ok("Wayland".to_string());
        }
    }

    if let Ok(x11) = std::env::var("DISPLAY") {
        if !x11.is_empty() {
            return Ok("X11".to_string());
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "Display protocol not detected"))
}

// Function to get GPU info
fn get_gpu_info() -> io::Result<String> {
    let output = Command::new("lspci")
        .arg("-nn")
        .output()?;
    
    let gpu_info = String::from_utf8_lossy(&output.stdout);
    
    // Find the line that contains 'VGA compatible controller' and clean it up
    for line in gpu_info.lines() {
        if line.contains("VGA compatible controller") {
            // Remove everything before the GPU model name
            let cleaned_line = line.split_whitespace().skip(2).collect::<Vec<&str>>().join(" ");
            return Ok(cleaned_line);
        }
    }
    
    Err(io::Error::new(io::ErrorKind::NotFound, "Could not detect GPU"))
}

// Function to get CPU info
fn get_cpu_info() -> io::Result<String> {
    let output = Command::new("cat")
        .arg("/proc/cpuinfo")
        .output()?;
    
    let cpu_info = String::from_utf8_lossy(&output.stdout);
    let mut model = String::new();
    let mut cores = 0;

    for line in cpu_info.lines() {
        if line.starts_with("model name") {
            model = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("processor") {
            cores += 1;
        }
    }
    
    Ok(format!("{} ({} cores)", model, cores))
}

// Function to detect installed packages
fn detect_installed_packages() -> io::Result<String> {
    // Count RPM packages
    let rpm_count = Command::new("rpm")
        .arg("-qa")
        .output()?
        .stdout.len();

    // Count Flatpak packages
    let flatpak_count = Command::new("flatpak")
        .arg("list")
        .output()?
        .stdout.len();

    Ok(format!("{} (rpm), {} (flatpak)", rpm_count, flatpak_count))
}
