#![allow(dead_code)]

//
// pac - (probably) the worlds smallest AUR helper
// Copyright (c) bimorphism 2021. Released under the MIT license.
//
mod util;

use std::env;
use std::fs;
use std::path::Path;
use getopts::Options;
use termion::{color, style};

static USAGE: &str = r#"
pac - probably the worlds smallest AUR helper
Usage: pac [options]

Options:
 -h,--help:     Displays usage for pac.
 -s,--search:   Search the AUR for a specific package.
 -u,--update:   Updates all packages in the package cache.
 -i,--install:  Install a package to the system
 -r,--rm:       Remove a package from both the cache and system.
 -l,--ls-cache: List all packages in the cache.
"#;

static AUR_REPO: &str = "https://aur.archlinux.org";
static AUR_CACHE_UNEXPANDED: &str = "~/dl/aur";

// should pacman run with --noconfirm when removing packages?
static PACMAN_NOCONFIRM_REMOVE: bool = false;

// TODO: Package manager specific functionality should probably be moved to aur.rs
// or something.

fn show_usage() {
    println!("{}", USAGE);
}

fn list_packages() {
    let aur_cache = util::expand_path(AUR_CACHE_UNEXPANDED);
    let paths = fs::read_dir(aur_cache).unwrap();

    println!("Packages in the AUR cache:");
    for path in paths {
        println!("{}{}{}", color::Fg(color::LightYellow), path.unwrap().path().display(), style::Reset);
    }
}

fn update_all_packages() {
    // TODO: Update all packages in the package cache.
    // Is there a way to check if a package needs updating or not based on git versioning?
}

fn get_package(name: &str) {
    let url = format!("{}/{}.git", AUR_REPO, name);
    let aur_cache = util::expand_path(AUR_CACHE_UNEXPANDED);

    let dir = format!("{}/{}", aur_cache, name);
    println!("Cloning package {}'{}'{} into {}'{}'{} using git..", 
        color::Fg(color::Green), name, style::Reset, color::Fg(color::Green), dir, style::Reset);
    
    util::run_command("git", vec!["clone".into(), url, dir]);
}

fn install_package(name: &str) {
    let aur_cache = util::expand_path(AUR_CACHE_UNEXPANDED);
    let pkg = format!("{}/{}", aur_cache, name);

    // Clone the package down!
    get_package(name);

    if !util::is_installed("makepkg") {
        eprintln!("{}Error: {}could not locate makepkg and by definition pacman!", color::Fg(color::Red), style::Reset);
        std::process::exit(1);
    }

    // Sanity check to make sure PKGBUILD is present

    if !Path::new(&format!("{}/PKGBUILD", pkg)).exists() {
        eprintln!("{}Error: {}No PKGBUILD found, not an AUR package!", color::Fg(color::Red), style::Reset);
        std::process::exit(1);
    }

    // All is good
    util::run_command_cd("makepkg", vec!["-si".into()], &pkg);
    println!("Installed package {}'{}'{} successfully!", color::Fg(color::Green), name, style::Reset);
}

fn remove_package(name: &str) {
    let aur_cache = util::expand_path(AUR_CACHE_UNEXPANDED);
    let pkg = format!("{}/{}", aur_cache, name);

    // Check if installed
    if Path::new(&pkg).exists() {
        println!("Removing {}'{}'{} from the cache", color::Fg(color::Green), name, style::Reset);
        util::run_command_cd("rm", vec!["-rf".into(), pkg.clone()], &aur_cache);
    }
    else {
        eprintln!("{}The package {}'{}'{} is not installed in the cache/system!{}", 
            color::Fg(color::Red), color::Fg(color::White), name, color::Fg(color::Red), style::Reset);
        std::process::exit(1);
    }

    // All is good
    util::run_command("sudo", vec!["pacman".into(), "-Rs".into(), name.to_string()]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Displays usage for pac");
    opts.optflag("l", "ls", "List all packages in the cache");
    opts.optflag("u", "update", "Updates all packages in the package cache");

    opts.optopt("i", "install", "Install a package to the system", "PKG");
    opts.optopt("r", "remove", "Remove a package from the system", "PKG");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(e) => { panic!("{}", e.to_string()) }
    };

    // Lots of junk..
    if matches.opt_present("h") {
        show_usage();
        return;
    }

    if matches.opt_present("l") {
        list_packages();
        return;
    }

    if matches.opt_present("u") {
        update_all_packages();
        return;
    }

    if !matches.opt_str("i").is_none() {
        // Install package
        install_package(&matches.opt_str("i").unwrap());
        return;
    }

    if !matches.opt_str("r").is_none() {
        // Remove package
        remove_package(&matches.opt_str("r").unwrap());
        return;
    }
}
