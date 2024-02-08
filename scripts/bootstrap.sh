set -e
# Load utilities
SCRIPT_DIRECTORY=$(dirname "$0")
source "$SCRIPT_DIRECTORY"/utilities
check_required_environment


PLATFORM=x86_64-unknown-freebsd
REPOSITORY_TARGET_LOCATION="$REMOTE_PRIVATE_DIRECTORY"/simple-http-server
REMOTE_BUILT_BINARY_LOCATION="$REPOSITORY_TARGET_LOCATION"/target/release/simple-http-server
REMOTE_DEPLOYED_BINARY_LOCATION="$REMOTE_PROTECTED_DIRECTORY"/server_bin
REMOTE_DAEMON_LOCATION="$REMOTE_PROTECTED_DIRECTORY"/daemon.sh
SSH_COMMAND="ssh $REMOTE_USER@$REMOTE_HOSTNAME"

# Start script
echo "Step 1: Refresh git"

echo_and_run $SSH_COMMAND \
  "/usr/local/bin/git -C $REPOSITORY_TARGET_LOCATION pull || /usr/local/bin/git clone $REPOSITORY_URL $REPOSITORY_TARGET_LOCATION"

echo "Step 2: Build for target"
echo_and_run $SSH_COMMAND \
  "/usr/local/bin/cargo build --release --manifest-path $REPOSITORY_TARGET_LOCATION/Cargo.toml"

echo "Step 3: Move to protected"
echo_and_run $SSH_COMMAND \
  "/bin/cp $REMOTE_BUILT_BINARY_LOCATION $REMOTE_DEPLOYED_BINARY_LOCATION"

echo "Step 4: Build daemon script"
echo_and_run $SSH_COMMAND \
  "
  /bin/echo '
#!/bin/bash
$REMOTE_DEPLOYED_BINARY_LOCATION\
  --public-directory $REMOTE_PUBLIC_DIRECTORY \
  --listen-port $SERVER_LISTEN_PORT \
  --log-directory $REMOTE_PRIVATE_DIRECTORY/logs
' | /usr/bin/tee $REMOTE_DAEMON_LOCATION

/bin/chmod 755 $REMOTE_DAEMON_LOCATION
"

echo "Done!"