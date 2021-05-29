use crate::cli;
use crate::util;

use std::fs;
use reqwest;
use std::path::Path;
use serde::{Serialize, Deserialize};
use termion::{color, style};

// TODO: maybe use a better library for colors so its less messy

/// Data structure returned from the JSON API for the AUR
#[derive(Deserialize, Debug)]
struct SearchResults {
    pub version: i32,
    #[serde(rename = "type")] 
    pub type_: String,
    #[serde(rename = "resultcount")]
    pub result_count: i32,
    pub results: std::vec::Vec<Package>
}

#[derive(Deserialize, Debug)]
struct Package {
    #[serde(rename = "ID")] pub id: i32,
    #[serde(rename = "Name")] pub name: String,
    #[serde(rename = "PackageBaseID")] pub base_id: i32,
    #[serde(rename = "PackageBase")] pub package_base: String,
    #[serde(rename = "Version")] pub version: String,
    #[serde(rename = "Description")] pub description: String,
    #[serde(rename = "URL")] pub url: Option<String>,
    #[serde(rename = "Maintainer")] pub maintainer: Option<String>,

    // not sure if all are needed..
}

pub fn list_packages(conf: cli::ConfigOptions) {
    let aur_cache = util::expand_path(&conf.aur_cache_path);
    let paths = fs::read_dir(aur_cache).unwrap();

    println!("Packages in the AUR cache:");
    for path in paths {
        println!("{}{}{}", color::Fg(color::LightBlue), path.unwrap().path().display(), style::Reset);
    }
}

pub fn search_aur(name: &str) {
    let url = format!("https://aur.archlinux.org/rpc/?v=5&type=search&by=name&arg={}", name);

    let resp = reqwest::blocking::get(url).expect("failed to make request");
    let search_results = resp.json::<SearchResults>().expect("failed to deserialize");
    
    for pkg in search_results.results {
        println!("{}{}{}", color::Fg(color::LightBlue), pkg.name, style::Reset);
        println!("  Maintainer: {}", pkg.maintainer.unwrap_or("no maintainer".into()));
        println!("  Version {}", pkg.version);
        println!("  Description:\n    {}", pkg.description);
        println!();
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
        color::Fg(color::LightGreen), name, style::Reset, color::Fg(color::LightGreen), dir, style::Reset);
    
    util::run_command("git", vec!["clone".into(), url, dir]);
}

pub fn install_package(name: &str, conf: cli::ConfigOptions) {
    let aur_cache = util::expand_path(&conf.aur_cache_path);
    let pkg = format!("{}/{}", aur_cache, name);

    // Clone the package down!
    get_package(name, conf);

    if !util::is_installed("makepkg") {
        eprintln!("{}Error: {}could not locate makepkg and by definition pacman!", color::Fg(color::LightRed), style::Reset);
        std::process::exit(1);
    }

    // Sanity check to make sure PKGBUILD is present

    if !Path::new(&format!("{}/PKGBUILD", pkg)).exists() {
        eprintln!("{}Error: {}No PKGBUILD found, not an AUR package!", color::Fg(color::LightRed), style::Reset);
        std::process::exit(1);
    }

    // All is good
    util::run_command_cd("makepkg", vec!["-si".into()], &pkg);
    println!("Installed package {}'{}'{} successfully!", color::Fg(color::LightGreen), name, style::Reset);
}

pub fn remove_package(name: &str, conf: cli::ConfigOptions) {
    let aur_cache = util::expand_path(&conf.aur_cache_path);
    let pkg = format!("{}/{}", aur_cache, name);

    // Check if installed
    if Path::new(&pkg).exists() {
        println!("Removing {}'{}'{} from the cache", color::Fg(color::LightGreen), name, style::Reset);
        util::run_command_cd("rm", vec!["-rf".into(), pkg.clone()], &aur_cache);
    }
    else {
        eprintln!("{}The package {}'{}'{} is not installed in the cache/system!{}", 
            color::Fg(color::LightRed), color::Fg(color::White), name, color::Fg(color::LightRed), style::Reset);
        std::process::exit(1);
    }

    // All is good
    util::run_command("sudo", vec!["pacman".into(), "-Rs".into(), name.to_string()]);
}