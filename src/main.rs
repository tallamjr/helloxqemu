fn main() {
    match sys_info::os_type() {
        Ok(os) => println!("OS: {}", os),
        Err(e) => println!("Error getting OS info: {}", e),
    }

    match sys_info::os_release() {
        Ok(release) => println!("OS Release: {}", release),
        Err(e) => println!("Error getting OS release info: {}", e),
    }

    match sys_info::cpu_num() {
        Ok(num) => println!("CPU Cores: {}", num),
        Err(e) => println!("Error getting CPU core count: {}", e),
    }

    match sys_info::cpu_speed() {
        Ok(speed) => println!("CPU Speed: {} MHz", speed),
        Err(e) => println!("Error getting CPU speed: {}", e),
    }

    match sys_info::mem_info() {
        Ok(mem) => println!(
            "Total Memory: {} KB, Free Memory: {} KB",
            mem.total, mem.free
        ),
        Err(e) => println!("Error getting memory info: {}", e),
    }
}
