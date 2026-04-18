#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/uninstall-service-server.sh [--instance <name>] [--config-dir <path>]

Remove generated craw-chat-server service artifacts and summarize systemd/launchd/windows-service uninstall status.
EOF
}

instance_name="default"
config_dir="/etc/craw-chat/default"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --instance)
      instance_name="$2"
      config_dir="/etc/craw-chat/${instance_name}"
      shift 2
      ;;
    --config-dir)
      config_dir="$2"
      shift 2
      ;;
    -h|--help)
      show_help
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      show_help >&2
      exit 1
      ;;
  esac
done

generated_dir="${config_dir}/generated"
rm -f \
  "${generated_dir}/craw-chat-server.service" \
  "${generated_dir}/com.sdkwork.crawchat.server.plist" \
  "${generated_dir}/CrawChatServer.xml" \
  "${generated_dir}/install-CrawChatServer.ps1" \
  "${generated_dir}/uninstall-CrawChatServer.ps1" \
  "${generated_dir}/service-install-report.json"
echo "Removed generated craw-chat-server service artifacts for instance '${instance_name}'."
echo "systemd target: craw-chat-server.service"
echo "launchd target: com.sdkwork.crawchat.server"
echo "windows service target: CrawChatServer"
