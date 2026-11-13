#!/usr/bin/env bash
# ==============================================================================
# WinNotch — WSL Shell Hook (F032)
# Mengirim notifikasi async task selesai ke WinNotch (Axum server localhost:9182).
#
# INSTALL (1 baris di ~/.bashrc):
#   echo 'source /path/to/docs/wsl_hook_setup.sh' >> ~/.bashrc
#
# PAKAI:
#   notch_wrap opencode run "task"
#   notch_wrap npm run build
#   notch_wrap cargo test
#
# CATATAN JARINGAN (WSL 2):
#   Default host 127.0.0.1 bekerja pada Windows 10/11 dengan WSL mirrored
#   networking. Kalau tidak sampai, set IP host Windows:
#     export NOTCH_HOST=$(ip route show default | awk '{print $3}')
#
# Sifat hook: idempotent. Kalau WinNotch mati, curl gagal diam-diam dan
# command yang dibungkus tetap jalan normal (PRD A9 Reliability).
# ==============================================================================

NOTCH_HOST="${NOTCH_HOST:-127.0.0.1}"
NOTCH_PORT="${NOTCH_PORT:-9182}"
# Task di bawah 10 detik tidak dikirim (PRD US4, tidak ada gunanya).
NOTCH_MIN_SECONDS="${NOTCH_MIN_SECONDS:-10}"

notch_notify() {
  local source="$1"
  local duration="$2"
  local exit_code="$3"

  command -v curl >/dev/null 2>&1 || return 0

  curl -s -m 2 -X POST "http://${NOTCH_HOST}:${NOTCH_PORT}/notch/notify" \
    -H 'Content-Type: application/json' \
    --data "{\"source\":\"${source}\",\"duration_seconds\":${duration},\"exit_code\":${exit_code}}" \
    >/dev/null 2>&1 || true
}

notch_wrap() {
  if [ "$#" -eq 0 ]; then
    echo "usage: notch_wrap <command> [args...]" >&2
    return 2
  fi

  # Nama sumber = nama command pertama (contoh: opencode, npm, cargo).
  local source
  source="$(basename -- "$1")"
  source="${source//\"/}"

  local start end duration rc
  rc=0
  start=$(date +%s)

  "$@" || rc=$?

  end=$(date +%s)
  duration=$((end - start))

  if [ "$duration" -ge "$NOTCH_MIN_SECONDS" ]; then
    notch_notify "$source" "$duration" "$rc"
  fi

  return "$rc"
}
