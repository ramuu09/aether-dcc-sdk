//! AetherDCC Studio v1.0. The window is the default. `--product-benchmark` prints the release report.

mod studio;

fn main() {
    let benchmark = std::env::args().any(|arg| arg == "--product-benchmark");
    if benchmark {
        let code = aether_dcc_sdk::product_benchmark();
        std::process::exit(code);
    }
    studio::launch();
}
