#!/usr/bin/env bash
set -euo pipefail

# 1) Build release
echo "📦 Building term_planner..."
cargo build --release

# 2) Install binary
BIN_SRC="target/release/term_planner"
BIN_DST="$HOME/.local/bin/term_planner"
echo "🚚 Installing binary to $BIN_DST"
install -Dm755 "$BIN_SRC" "$BIN_DST"

# 3) Install systemd units
UNIT_DIR="$HOME/.config/systemd/user"
echo "📄 Copying systemd units to $UNIT_DIR"
mkdir -p "$UNIT_DIR"
install -m644 systemd/term_planner-notify.service "$UNIT_DIR/"
install -m644 systemd/term_planner-notify.timer   "$UNIT_DIR/"

# 4) Reload and enable
echo "🔄 Reloading systemd user daemon"
systemctl --user daemon-reload

echo "⏰ Enabling and restarting timer"
systemctl --user enable --now term_planner-notify.timer

echo "✅ Installation complete!"
