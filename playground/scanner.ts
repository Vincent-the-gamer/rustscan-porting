import { close, DataType, load, open } from "ffi-rs"

const library = "librustscan" 

const path = "../target/release/librustscan.dylib"

open({
    library,
    path
})

const result = load({
    library,
    funcName: "scanner_run",
    retType: DataType.String,
    paramsType: [DataType.String, DataType.U64, DataType.U64, DataType.U64, DataType.U64, DataType.U8],
    paramsValue: ["192.168.1.132", 1, 100, 1000, 10, 3]
})

try {
    const ports = JSON.parse(result)
    console.log("Open ports:", ports)
} catch (e) {
    console.error("Failed to parse result:", e)
    console.log("Raw result:", result)
}

close(library)