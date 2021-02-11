TARGETIP=192.168.7.1
EXECUTABLE=hello_world
OLDPROGRAMPATH=/tmp/$EXECUTABLE
TARGET_DIR=./target/armv7-unknown-linux-gnueabihf/debug

# Build the target
cargo build

# Kill all previous debugging servers
ssh root@$TARGETIP killall gdbserver

# Kill all previous instances
ssh root@$TARGETIP killall $EXECUTABLE

# Remove old program
ssh root@$TARGETIP rm $OLDPROGRAMPATH

# Transfer the new program
scp $TARGET_DIR/$EXECUTABLE root@$TARGETIP:/tmp

# Begin a new debugging session
ssh -n -f root@$TARGETIP "sh -c 'cd /tmp; nohup gdbserver :5000 $EXECUTABLE > /dev/null 2>&1 &'"