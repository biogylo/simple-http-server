set -e
# Load utilities
SCRIPT_DIRECTORY=$(dirname "$0")
source "$SCRIPT_DIRECTORY"/utilities
check_required_environment

# Start script
echo "Remoting into server"
echo_and_run ssh "$REMOTE_USER"@"$REMOTE_HOSTNAME" "$@"
echo "Done!"