use cmake;

fn main() {
    let mut config = cmake::Config::new("unicorn-1.0.3");
    config
        .profile("Release")
        .pic(true)
        .define("UNICORN_BUILD_SAMPLES","OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build_target("unicorn");
    let mut dst = config.build();
    dst.push("build");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.to_str().expect("link-search UTF-8 error")
    );
    println!("cargo:rustc-link-lib=unicorn");
}
