// 导入 rustscan 的核心模块
use rustscan::input::{PortRange, ScanOrder};
use rustscan::port_strategy::PortStrategy;
use rustscan::scanner::Scanner;
use std::ffi::CStr;
use std::net::IpAddr;
use std::os::raw::c_char;
use std::time::Duration;

/// C ABI 导出：扫描指定 IP 和端口范围，发现开放端口时通过回调实时输出
/// ip: C 字符串，如 "127.0.0.1"
/// port_start, port_end: 端口范围
/// timeout_ms: 超时时间（毫秒）
/// batch_size: 并发批量数
/// tries: 每端口尝试次数
/// excluded_ports_ptr: 指向排除端口数组的指针
/// excluded_ports_len: 排除端口数组的长度
#[unsafe(no_mangle)]
pub extern "C" fn scanner_run(
    ip: *const c_char,
    batch_size: u16,
    timeout_ms: u32,
    tries: u8,
    greppable: bool,
    port_start: u16,
    port_end: u16,
    accessible: bool,
    excluded_ports_ptr: *const u16,
    excluded_ports_len: usize,
    udp: bool,
) {
    // 安全转换 C 字符串
    let c_str = unsafe { CStr::from_ptr(ip) };
    let ip_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return,
    };
    let ip_addr: IpAddr = match ip_str.parse() {
        Ok(addr) => addr,
        Err(_) => return,
    };

    // 安全转换排除端口数组
    let excluded_ports = if excluded_ports_ptr.is_null() || excluded_ports_len == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(excluded_ports_ptr, excluded_ports_len).to_vec() }
    };

    let addrs = vec![ip_addr];
    let range = PortRange {
        start: port_start,
        end: port_end,
    };
    let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Serial);
    let scanner = Scanner::new(
        &addrs,
        batch_size,
        Duration::from_millis(timeout_ms as u64),
        tries,
        greppable,
        strategy,
        accessible,
        excluded_ports,
        udp,
    );
    // 阻塞运行异步扫描，发现端口时输出
    let _ = async_std::task::block_on(scanner.run_with_callback(move |port| {
        println!("{}:{}", ip_str, port);
    }));
}
