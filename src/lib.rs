use futures::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_cli_support::Bindgen;

#[wasm_bindgen]
pub struct Output {
    output: wasm_bindgen_cli_support::Output,
}

#[wasm_bindgen]
pub fn process(wasm_bytes: &[u8], esm: bool) -> js_sys::Promise {
    match _process(wasm_bytes, esm) {
        Ok(output) => output,
        Err(e) => js_sys::Promise::reject(&e),
    }
}

fn _process(wasm_bytes: &[u8], esm: bool) -> Result<js_sys::Promise, JsValue> {
    let output = compile("wasm", wasm_bytes, esm)?;
    let js = format!("const self = {{}}; {}\n return self;", output.output.js());
    let function = js_sys::Function::new_no_args(&js);
    let js_self = function.call0(&JsValue::undefined())?;
    let module = js_sys::Reflect::get(&js_self, &"wasm_bindgen".into())?;
    let wasm_bytes = output.wasm()?;
    let promise = unsafe {
        let view = js_sys::Uint8Array::view(&wasm_bytes);
        module
            .unchecked_ref::<js_sys::Function>()
            .call1(&JsValue::undefined(), &view)?
    };
    let promise = promise.unchecked_into::<js_sys::Promise>();
    let instantiate = wasm_bindgen_futures::JsFuture::from(promise);
    let future = instantiate.map(|_| module);
    Ok(wasm_bindgen_futures::future_to_promise(future))
}

#[wasm_bindgen]
pub fn compile(name: &str, wasm_bytes: &[u8], esm: bool) -> Result<Output, JsValue> {
    _compile(name, wasm_bytes, esm).map_err(|e| JsValue::from(format!("{:?}", e)))
}

fn _compile(name: &str, wasm: &[u8], esm: bool) -> Result<Output, failure::Error> {
    console_error_panic_hook::set_once();

    let module = walrus::ModuleConfig::new()
        .generate_dwarf(false)
        .generate_name_section(true)
        .generate_producers_section(false)
        .on_parse(wasm_webidl_bindings::binary::on_parse)
        .parse(wasm)?;
    let mut bindgen = Bindgen::new();
    if !esm {
        bindgen.no_modules(true)?;
    }
    bindgen.input_module(name, module);
    let output = bindgen.generate_output()?;
    Ok(Output { output })
}

#[wasm_bindgen]
impl Output {
    pub fn js(&self) -> String {
        self.output.js().to_string()
    }

    pub fn wasm(&self) -> Result<Vec<u8>, JsValue> {
        self.output
            .wasm()
            .emit_wasm()
            .map_err(|e| JsValue::from(format!("{:?}", e)))
    }
}
