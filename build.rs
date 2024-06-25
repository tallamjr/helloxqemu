macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    match sys_info::os_type() {
        Ok(os) => p!("OS: {}", os),
        Err(e) => p!("Error getting OS info: {}", e),
    }

    match sys_info::os_release() {
        Ok(release) => p!("OS Release: {}", release),
        Err(e) => p!("Error getting OS release info: {}", e),
    }

    match sys_info::cpu_num() {
        Ok(num) => p!("CPU Cores: {}", num),
        Err(e) => p!("Error getting CPU core count: {}", e),
    }

    match sys_info::cpu_speed() {
        Ok(speed) => p!("CPU Speed: {} MHz", speed),
        Err(e) => p!("Error getting CPU speed: {}", e),
    }

    match sys_info::mem_info() {
        Ok(mem) => p!(
            "Total Memory: {} KB, Free Memory: {} KB",
            mem.total,
            mem.free
        ),
        Err(e) => p!("Error getting memory info: {}", e),
    }
}
