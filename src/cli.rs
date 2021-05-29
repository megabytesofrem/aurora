use crate::aur;

// CLI parsing code for aurora
use std::env;
use getopts::Options;

static USAGE: &str = r#"
aurora - probably the worlds smallest AUR helper
Usage: aurora [options]

Options:
 -h,--help:     Displays usage for aurora.
 -s,--search:   Search the AUR for a specific package.
 -u,--update:   Updates all packages in the package cache.
 -i,--install:  Install a package to the system
 -r,--rm:       Remove a package from both the cache and system.
 -l,--ls-cache: List all packages in the cache.
"#;

pub struct ConfigOptions {
    pub aur_cache_path: String,
    pub aur_repo: String,
    pub pacman_noconfirm: bool
}

fn show_usage() {
    println!("{}", USAGE);
}

pub fn parse_args(conf: ConfigOptions) {
    let args: Vec<String> = env::args().collect();
    let _program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Displays usage for aurora");
    opts.optflag("l", "ls", "List all packages in the cache");
    opts.optflag("u", "update", "Updates all packages in the package cache");

    opts.optopt("s", "search", "Search the AUR f or a specific package", "PKG");
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
        aur::list_packages(conf);
        return;
    }

    if matches.opt_present("u") {
        aur::update_all_packages();
        return;
    }

    if !matches.opt_str("s").is_none() {
        aur::search_aur(&matches.opt_str("s").unwrap());
    }

    if !matches.opt_str("i").is_none() {
        // Install package
        aur::install_package(&matches.opt_str("i").unwrap(), conf);
        return;
    }

    if !matches.opt_str("r").is_none() {
        // Remove package
        aur::remove_package(&matches.opt_str("r").unwrap(), conf);
        return;
    }
}