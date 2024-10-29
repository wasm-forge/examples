#[ic_cdk::query]
fn eval(js_code: String) -> String {
    use boa_engine::{Context, Source};

    let mut context = Context::default();

    match context.eval(Source::from_bytes(&js_code)) {
        Ok(res) => {
            return res.to_string(&mut context).unwrap().to_std_string_escaped();
        }
        Err(e) => {
            eprintln!("Uncaught {e}");
        }
    };
    return String::from("");
}

#[ic_cdk::init]
fn init() {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);
}
