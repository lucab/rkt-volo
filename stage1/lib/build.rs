extern crate serde_codegen;

use std::env;
use std::path::Path;

fn main() {
    // serde codegen for json types
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let tt = vec![
        "appc_pod",
        "runtime_app",
        "runtime_pod",
        "stage1_cli",
    ];
    for i in tt {
        let src = Path::new("src").join("json-types").join(format!("{}.in.rs", i));
        let dst = Path::new(&out_dir).join(format!("{}.rs", i));
        serde_codegen::expand(&src, &dst).unwrap();
    }

}
