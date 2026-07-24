# rwm
rwm is a window manager written in rust

---

## Dependencies
- Rust toolchain
- Xorg
- Xinit (for display manager)

## Install
```bash
git clone https://github.com/ayxan20145-prog/rwm.git
cd rwm
chmod +x scripts/install.sh
./scripts/install.sh
```

## Uninstall
```bash
chmod +x scripts/uninstall.sh
./scripts/uninstall.sh
```

## Start rwm
```bash
echo "exec rwm" >> ~/.xinitrc
startx
```
