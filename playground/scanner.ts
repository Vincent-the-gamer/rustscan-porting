import { close, DataType, load, open } from "ffi-rs"

const library = "librustscan" 

const path = "../target/release/librustscan.dylib"

open({
    library,
    path
})

load({
    library,
    funcName: "scanner_run",
    retType: DataType.String,
    paramsType: [
        DataType.String, // ip: *const c_char
        DataType.I32, // batch_size: u16 (使用I32替代)
        DataType.I32, // timeout_ms: u32 (使用I32替代)
        DataType.U8, // tries: u8
        DataType.Boolean, // greppable: bool
        DataType.I32, // port_start: u16 (使用I32替代)
        DataType.I32, // port_end: u16 (使用I32替代)
        DataType.Boolean, // accessible: bool
        DataType.I32, // excluded_ports_ptr: 用I32传递0作为空指针
        DataType.U64, // excluded_ports_len: usize
        DataType.Boolean, // udp: bool
    ],
    paramsValue: [
        "192.168.1.132", // IP地址
        10, // batch_size
        1000, // timeout_ms
        1, // tries
        true, // greppable
        1, // port_start
        1000, // port_end
        true, // accessible
        0, // excluded_ports_ptr (0作为空指针)
        0, // excluded_ports_len
        false // udp
    ]
})

close(library)