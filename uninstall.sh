#!/usr/bin/env bash
set -euo pipefail

# Paths
BIN_DST="$HOME/.local/bin/term_planner"
UNIT_DIR="$HOME/.config/systemd/user"
SERVICE="$UNIT_DIR/term_planner-notify.service"
TIMER="$UNIT_DIR/term_planner-notify.timer"

echo "⏹️  Parando e desabilitando timer…"
systemctl --user disable --now term_planner-notify.timer || true

echo "🔄  Recarregando systemd user daemon…"
systemctl --user daemon-reload

echo "🗑️  Removendo systemd units…"
rm -f "$SERVICE" "$TIMER"

echo "🚮  Removendo binário…"
rm -f "$BIN_DST"

echo "✅  Uninstall completo!"
