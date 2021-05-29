# aurora
Probably the worlds smallest AUR helper. Now written in Rust!

## Configuration
Edit `config.ini`. Below are some good defaults if you're not sure what to put.

```ini
[aur]
# Where should aurora store the AUR cache?
# If this folder doesnt exist, it will need to be created first
aur_cache_path = ~/.aurora/cache

# Should pacman use --noconfirm when removing packages?
# Default is "false" 
pacman_noconfirm = false
```

## Usage
`aurora` should *not* be ran as superuser since `makepkg` elevates automatically when needed to!

Install a package:
`aurora -i c-lolcat`

Remove a package:
`aurora -r c-matrix`

List all packages in the package cache
`aurora -l` or `aurora --ls-cache`