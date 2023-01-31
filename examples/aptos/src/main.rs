use anyhow::Result;
use wasi_experimental_http_wasmtime::{HttpCtx, HttpState};
use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

struct IntegrationTestsCtx {
    pub wasi: WasiCtx,
    pub http: HttpCtx,
}

fn main()  {

    let engine = Engine::default();

    let mut linker = Linker::new(&engine);

    let wasi = WasiCtxBuilder::new()
        .inherit_args()
        .unwrap()
        .arg("compile").unwrap()
        .inherit_env()
        .unwrap()
        .inherit_stdin()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let http = HttpCtx {
        allowed_hosts: Some(vec!["insecure:allow-all".to_string()]),
        max_concurrent_requests: Some(10),
    };

    let mut store = Store::new(&engine, IntegrationTestsCtx { wasi, http });

    wasmtime_wasi::add_to_linker(
        &mut linker,
        |cx: &mut IntegrationTestsCtx| -> &mut WasiCtx { &mut cx.wasi },
    )
    .unwrap();

    // Link `wasi_experimental_http`
    let http = HttpState::new().unwrap();
    http.add_to_linker(&mut linker, |cx: &IntegrationTestsCtx| -> HttpCtx {
        cx.http.clone()
    })
    .unwrap();

    println!("loading wasm file");
    let module = wasmtime::Module::from_file(   
        store.engine(),
        "/Volumes/dev/project/movefuns/move-wasm/target/wasm32-wasi/release/aptos-wasm.wasm",
    )
    .unwrap();

    linker.module(&mut store, "", &module);

    linker
    .get_default(&mut store, "").unwrap()
    .typed::<(),(), _>(&store).unwrap()
    .call(&mut store, ());

    // let instance = linker.instantiate(&mut store, &module).unwrap();

    // let func = instance.get_func(&mut store, "_start").unwrap();

    // func.call(&mut store, &[], &mut []);
}
