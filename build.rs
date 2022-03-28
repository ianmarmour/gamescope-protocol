
extern crate wayland_scanner;

use std::env::var;
use std::path::Path;
use wayland_scanner::*;

#[rustfmt::skip]
type StableProtocol<'a> =    (&'a str,                &'a [(&'a str, &'a str)]);

static STABLE_PROTOCOLS: &[StableProtocol] =
    &[("gamescope-pipewire", &[]), ("gamescope-xwayland", &[]), ("gamescope-input-method", &[])];

fn generate_protocol(
    name: &str,
    protocol_file: &Path,
    out_dir: &Path,
    client: bool,
    server: bool,
    dest_events: &[(&str, &str)],
) {
    println!("cargo:rerun-if-changed={}", protocol_file.display());

    if client {
        generate_code_with_destructor_events(
            &protocol_file,
            out_dir.join(&format!("{}_client_api.rs", name)),
            Side::Client,
            dest_events,
        );
    }

    if server {
        generate_code_with_destructor_events(
            &protocol_file,
            out_dir.join(&format!("{}_server_api.rs", name)),
            Side::Server,
            dest_events,
        );
    }
}

fn main() {
    println!("cargo:rerun-if-changed-env=CARGO_FEATURE_CLIENT");
    println!("cargo:rerun-if-changed-env=CARGO_FEATURE_SERVER");

    let out_dir_str = var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    let client = var("CARGO_FEATURE_CLIENT").ok().is_some();
    let server = var("CARGO_FEATURE_SERVER").ok().is_some();

    for &(name, dest_events) in STABLE_PROTOCOLS {
        let file = format!("{name}.xml", name = name);
        generate_protocol(
            name,
            &Path::new("./gamescope/protocol/").join(&file),
            out_dir,
            client,
            server,
            dest_events,
        );
    }
}