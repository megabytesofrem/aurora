use crate::cli;
use crate::util;

use std::fs;
use std::path::Path;
use termion::{color, style};

pub fn list_packages(conf: cli::ConfigOptions) {
    let aur_cache = util::expand_path(&conf.aur_cache_path);
    let paths = fs::read_dir(aur_cache).unwrap();

    println!("Packages in the AUR cache:");
    for path in paths {
        println!("{}{}{}", color::Fg(color::LightYellow), path.unwrap().path().display(), style::Reset);
    }
}

pub fn update_all_packages() {
    // TODO: Update all packages in the package cache.
    // Is there a way to check if a package needs updating or not based on git versioning?
}

fn get_package(name: &str, conf: cli::ConfigOptions) {
    let url = format!("{}/{}.git", &conf.aur_repo, name);
    let aur_cache = util::expand_path(&conf.aur_cache_path);

    let dir = format!("{}/{}", aur_cache, name);
    println!("Cloning package {}'{}'{} into {}'{}'{} using git..", 
        color::Fg(color::Green), name, style::Reset, color::Fg(color::Green), dir, style::Reset);
    
    util::run_command("git", vec!["clone".into(), url, dir]);
}

pub fn install_package(name: &str, conf: cli::ConfigOptions) {
    let aur_cache = util::expand_path(&conf.aur_cache_path);
    let pkg = format!("{}/{}", aur_cache, name);

    // Clone the package down!
    get_package(name, conf);

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

pub fn remove_package(name: &str, conf: cli::ConfigOptions) {
    let aur_cache = util::expand_path(&conf.aur_cache_path);
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