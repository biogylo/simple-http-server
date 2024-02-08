set -e
# Load utilities
SCRIPT_DIRECTORY=$(dirname "$0")
source "$SCRIPT_DIRECTORY"/utilities
check_required_environment

# Start script
echo "Syncing assets"
echo_and_run rsync -av "$SCRIPT_DIRECTORY"/../assets/public/ "$REMOTE_USER"@"$REMOTE_HOSTNAME":"$REMOTE_PUBLIC_DIRECTORY"/ --delete
echo "Done!"