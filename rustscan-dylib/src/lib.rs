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
/// cb: extern "C" fn(port: u16) 回调函数，发现开放端口时调用
#[unsafe(no_mangle)]
pub extern "C" fn scanner_run(
    ip: *const c_char,
    port_start: u16,
    port_end: u16,
    timeout_ms: u32,
    batch_size: u16,
    tries: u8,
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
        true,
        strategy,
        true,
        vec![],
        false,
    );
    // 阻塞运行异步扫描，发现端口时输出
    let _ = async_std::task::block_on(scanner.run_with_callback(move |port| {
        println!("{}:{}", ip_str, port);
    }));
}
