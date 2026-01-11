use std::path::PathBuf;

fn main() {
    let source_root = PathBuf::from("SDL_ttf");

    let ctx = sdl_build::BuildContext {
        lib_name: "SDL3_ttf".into(),
        package_name: "sdl3_ttf".into(),
        static_link: std::env::var("CARGO_FEATURE_STATIC_LINK").is_ok(),
        source_root,
    };

    let include_paths = sdl_build::compile::prepare_library(&ctx);

    let config = sdl_build::generate::BindingsConfig {
        wrapper_header: "wrapper.h".into(),
        include_paths,

        allowlist: vec!["^TTF_.*".into()],

        blocklist: vec!["^SDL_.*".into()],

        raw_lines: vec!["use sdl_sys_bindgen::*;".into()],

        ..Default::default()
    };

    sdl_build::generate::generate_bindings(config);
}
