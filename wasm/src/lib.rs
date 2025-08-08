use js_sys::{Array, JsString};
use rustscan::input::{PortRange, ScanOrder};
use rustscan::port_strategy::PortStrategy;
use rustscan::scanner::Scanner;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub struct WasmScanner {
    inner: Scanner,
}

#[wasm_bindgen]
impl WasmScanner {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ips: Array,
        batch_size: u16,
        timeout_ms: u64,
        tries: u8,
        greppable: bool,
        start_port: u16,
        end_port: u16,
        order: &str,
        accessible: bool,
        exclude_ports: Array,
        udp: bool,
    ) -> Result<WasmScanner, JsValue> {
        let ips: Result<Vec<IpAddr>, _> = ips
            .iter()
            .map(|ip| {
                let s = ip
                    .as_string()
                    .ok_or_else(|| JsValue::from_str("Invalid IP string"))?;
                IpAddr::from_str(&s).map_err(|_| JsValue::from_str("Invalid IP format"))
            })
            .collect();
        let ips = ips?;

        let exclude_ports: Result<Vec<u16>, _> = exclude_ports
            .iter()
            .map(|p| {
                p.as_f64()
                    .map(|v| v as u16)
                    .ok_or_else(|| JsValue::from_str("Invalid port"))
            })
            .collect();
        let exclude_ports = exclude_ports?;

        let port_range = PortRange {
            start: start_port,
            end: end_port,
        };
        let scan_order = match order.to_lowercase().as_str() {
            "serial" => ScanOrder::Serial,
            "random" => ScanOrder::Random,
            _ => return Err(JsValue::from_str("Invalid scan order")),
        };
        let strategy = PortStrategy::pick(&Some(port_range), None, scan_order);

        let scanner = Scanner::new(
            &ips,
            batch_size,
            Duration::from_millis(timeout_ms),
            tries,
            greppable,
            strategy,
            accessible,
            exclude_ports,
            udp,
        );
        Ok(WasmScanner { inner: scanner })
    }

    #[wasm_bindgen]
    pub fn run(&self) -> js_sys::Promise {
        let scanner = self.inner.clone();
        future_to_promise(async move {
            let results = scanner.run().await;
            let arr = Array::new();
            for socket in results {
                arr.push(&JsString::from(socket.to_string()));
            }
            Ok(arr.into())
        })
    }
}
