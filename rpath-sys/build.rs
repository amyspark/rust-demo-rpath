use pkg_config;

fn main() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    let x = pkg_config::Config::new()
        .atleast_version("0.1")
        .probe("rpath_meson")
        .unwrap();
    if x.ld_args.len() > 0 {
        for flags in x.ld_args {
            println!("cargo:rustc-link-arg=-Wl,{}", flags.join(","))
        }
    }
}
