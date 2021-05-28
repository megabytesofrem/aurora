# pac
Probably the worlds smallest AUR helper.
Also probably quite buggy.

## Configuration
Edit `pac` and modify the following lines
```py
# Change this if for some reason you use a different AUR mirror
AUR_REPO = "https://aur.archlinux.org"

# The directory for the cache, you most probably do not want my defaults
AUR_CACHE = expanduser("~/dl/aur") # change this if you're sane

# Automatic removing pacman packages with --noconfirm
# Not recommended but its left here as an option if you want
PACMAN_AUTOMATE = False
```

## Usage
Copy/symlink `pac` somewhere so its on your path firs.t

`pac -gi <pkg-name>` will clone a package locally to its package cache directory, and then run
`makepkg -si` on it to install it systemwide.

`pac --rm <pkg-name>` will remove a package both from the package cache and the system.
