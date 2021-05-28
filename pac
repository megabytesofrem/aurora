#!/usr/bin/env python

"""
pac - (probably) the worlds smallest AUR helper
"""
import glob
import sys
import getopt

from os.path import expanduser
from subprocess import check_output

USAGE = """
pac - (probably) the worlds smallest AUR helper
Usage: pac [opts]

Installing a package: pac -gi <pkg-name>
Removing a package: pac --rm <pkg-name>

Options:
 -h or --help:      displays USAGE for pac
 -g or --get:       clones a package from the AUR to the cache
 -p or --pull:      pulls any changes into the existing cache
 -i or --install:   install a package to the system
 -r or --rm:        removes a package from both the cache and system
 -l or --ls-cache:  lists all packages in the cache
"""

AUR_REPO = "https://aur.archlinux.org"
AUR_CACHE = expanduser("~/dl/aur") # change this if you're sane

# Automatic removing pacman packages with --noconfirm
# Not recommended but its left here as an option if you want
PACMAN_AUTOMATE = False

clone_package = False
pull_changes = False

def rm_package_cache(name: str):
    # Remove a package from the local AUR cache
    check_output(['rm', '-rf', f"{AUR_CACHE}/{name}"], cwd=AUR_CACHE)

def get_package(name: str):
    url = f"{AUR_REPO}/{name}.git"

    # Clone the package to AUR_DIRECTORY
    if not pull_changes:
        check_output(['git', 'clone', url, f"{AUR_CACHE}/{name}"])
    else:
        print("pulling changes into local package cache..")
        check_output(['git', 'pull', url, f"{AUR_CACHE}/{name}"])

def install_package(name: str):
    # Run makepkg -si on it
    check_output(['makepkg', '-si'], cwd=f"{AUR_CACHE}/{name}")
    
    print("finished installing package into system!")

def rm_package(name: str):
    print(f"removing '{name}' from the local cache at '{AUR_CACHE}'")
    rm_package_cache(name)

    print(f"removing '{name}' from the system using pacman")

    out = ""
    if PACMAN_AUTOMATE:
        out = check_output(['sudo', 'pacman', '--noconfirm', '-Rs', name])
    else:
        out = check_output(['sudo', 'pacman', '-Rs', name])

def ls_packages():
    pkgs = glob.glob(f"{AUR_CACHE}/*")

    for pkg in pkgs:
        print(pkg)

def parse():
    global clone_package, pull_changes

    try:
        opts, args = getopt.getopt(
            sys.argv[1:],
            'hgpi:rl',
            ["help", "get=", "install=", "rm=", "ls-cache"]
        )

        for o, a in opts:
            if o in ("-h", "--help"):
                print(USAGE)
            if o in ("-g", "--get"):
                clone_package = True
                #get_package(a)
            if o in ("-p", "--pull"):
                clone_package = False
                pull_changes = True
            if o in ("-i", "--install"):
                if clone_package or pull_changes:
                    get_package(a)
                    install_package(a)
                else:
                    print("since -g wasnt specified, no need to clone sources")
                    install_package(a)
                #install_package(a)
            if o in ("-r", "--rm"):
                rm_package(a)
            if o in ("-l", "--ls-cache"):
                ls_packages()
    except getopt.GetoptError:
        print("invalid option, see --help for usage")

parse()