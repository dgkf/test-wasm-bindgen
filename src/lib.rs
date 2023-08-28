use wasm_bindgen::{prelude::*, convert::{WasmAbi, IntoWasmAbi, FromWasmAbi}, describe::WasmDescribe};

#[repr(C)]
pub struct WasmResultAbi {
    ok: String,
    err: bool
}

unsafe impl WasmAbi for WasmResultAbi {}

pub struct WasmResult {
    res: Result<String, ()>
}

impl WasmDescribe for WasmResult {
    fn describe() {}
}

impl IntoWasmAbi for WasmResult {
    type Abi = WasmResultAbi;

    fn into_abi(self) -> Self::Abi {
        match self.res {
            Ok(ok) => WasmResultAbi { ok, err: false },
            Err(_) => WasmResultAbi { ok: "".to_string(), err: true },
        }
    }
}

impl FromWasmAbi for WasmResult {
    type Abi = WasmResultAbi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        if js.err {
            WasmResult { res: Err(()) }
        } else {
            WasmResult { res: Ok(js.ok) }
        }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> WasmResult {
    if name == "abc" {
        WasmResult { res: Ok(name.to_string()) }
    } else {
        WasmResult { res: Err(()) }
    }
}
