#![allow(dead_code)]

//
// aurora - probably the worlds smallest AUR helper
// Copyright (c) bimorphism 2021. Released under the MIT license.
//
mod util;
mod aur;
mod cli;

use std::error::Error;
use std::fs;
use configparser::ini::Ini;

// TODO: remove this since we now parse from config.ini
static AUR_REPO: &str = "https://aur.archlinux.org";
static AUR_CACHE_UNEXPANDED: &str = "~/dl/aur";

// should pacman run with --noconfirm when removing packages?
static PACMAN_NOCONFIRM_REMOVE: bool = false;

// TODO: Package manager specific functionality should probably be moved to aur.rs
// or something.

fn main() -> Result<(), Box<dyn Error>> {
    // Load the config.ini file
    let mut config = Ini::new();
    let map = config.load("config.ini");

    let aur_cache_path = config.get("aur", "aur_cache_path").unwrap();

    let conf_options = cli::ConfigOptions {
        aur_cache_path: aur_cache_path,
        aur_repo: "https://aur.archlinux.org".into(),
        pacman_noconfirm: false
    };

    // CLI parsing
    cli::parse_args(conf_options);
    Ok (())
}
