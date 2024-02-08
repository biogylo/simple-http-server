set -e
# Load utilities
SCRIPT_DIRECTORY=$(dirname "$0")
source "$SCRIPT_DIRECTORY"/utilities
SSH_COMMAND="ssh $REMOTE_USER@$REMOTE_HOSTNAME"

# Start script
echo "Step 1: Cloning repo if missing"
REPOSITORY_TARGET_LOCATION=$REMOTE_PRIVATE_DIRECTORY/simple-http-server
echo_and_run $SSH_COMMAND \
  "/usr/local/bin/git -C $REPOSITORY_TARGET_LOCATION pull || /usr/local/bin/git clone $REPOSITORY_URL $REPOSITORY_TARGET_LOCATION"

echo "Step 2: Build for target"
echo_and_run $SSH_COMMAND \
  "/usr/local/bin/cargo build --release --manifest-path $REPOSITORY_TARGET_LOCATION/Cargo.toml"

echo "Step 3: Move to protected"
echo_and_run $SSH_COMMAND \
  "/bin/mv $REPOSITORY_TARGET_LOCATION/target/simple-http-server $REMOTE_PROTECTED_DIRECTORY/server.bin"

echo "Done!"