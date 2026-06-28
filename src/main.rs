mod assets;

use std::path::Path;

fn main() {
    println!("Railgun v{}", env!("CARGO_PKG_VERSION"));

    let baseq_dir = Path::new("./data/baseq3");

    assets::pk3::load_resources(baseq_dir);
}
