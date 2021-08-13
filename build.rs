use cmake;

fn main() {
    let mut config = cmake::Config::new("unicorn-1.0.3");
    config
        .profile("Release")
        .pic(true)
        .define("BUILD_SHARED_LIBS", "false")
        .build_target("unicorn");
    let mut dst = config.build();
    dst.push("build");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.to_str().expect("link-search UTF-8 error")
    );
    println!("cargo:rustc-link-lib=unicorn");
}
