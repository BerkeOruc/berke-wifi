# berke-wifi

TUI WiFi Manager for Bearch Linux.

## Features

- WiFi scanning via nmcli
- Connect/disconnect functionality
- Signal strength display with ASCII bars
- vim-style navigation (j/k/Enter/Esc/q)

## Installation

```bash
cargo build --release
```

## Usage

```bash
./target/release/berke-wifi
```

## Controls

- `j` / `k` - Move selection up/down
- `c` - Connect to selected network
- `d` - Disconnect
- `r` - Refresh network list
- `q` - Quit