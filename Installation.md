# Running code on the DE10-Nano with Linux

This project expects the user to have VSCode installed.

## 0. Windows setup

On windows I highly recommend using WSL (Windows Subsystem for Linux) to run a linux environment on Windows.

## 1. Install rust

For a complete guide, go to the [official website](https://www.rust-lang.org/tools/install).

The short version is just use the following command and accept everything.

> ⚠️ Make sure to select `y` when it asks you whether or not to add to path.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Install proper target

Simply use the following command:
```
rustup toolchain add armv7-unknown-linux-gnueabihf
```

## 3. Install proper linker:

Using APT (there are equivalent for other distros):
```
sudo apt install gcc-arm-linux-gnueabihf
```

## 4. Install proper debugger

Using APT (there are equivalent for other distros):
```
sudo apt install gdb-multiarch
```

## 5. Add cargo config file to 

1. Create a `.cargo` folder in the root of your crate (same folder as your `Cargo.toml`)
2. Create a `config` file (⚠️ no file extension) with the following contents:
```toml
[build]
target = "armv7-unknown-linux-gnueabihf"

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

## 6. Debugging

First we'll see the different steps required to debug then we'll write a simple shell script to do it all automatically

### 6.1. Uploading the file

We can do this using SSH and ´scp` (file transfer over SSH):
```
    scp ./target/armv7-unknown-linux-gnueabihf/debug/{exec_name} root@{target_ip}:/tmp/{exec_name}
```

> ⚠️ Note that the workspace folder must exit on the board.

### 6.2 Launching a GDB server for this executable

We simply run this command in an SSH session on the board:
```
    gdbserver :5000 /tmp/{exec_name}
```

### 6.3 Debugging using GDB:

First, we'll need open gdb:
```
    gdb-multiarch
```

Then we'll need to input the following commands inside of the GDB prompt
```
    file ./target/armv7-unknown-linux-gnueabihf/debug/{exec_name}
    target remote {target_ip}:5000
```

Then here we can begin a new debugging session using normal GDB commands such as:
- `continue`
- `breakpoint` (`b`)
- `backtrace`
- [More here](https://gist.github.com/rkubik/b96c23bd8ed58333de37f2b8cd052c30)

### 7. Editor setup and debugger support

0. Install VSCode
1. Install the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension
2. Install the [GDB Debugger - Beyond](https://marketplace.visualstudio.com/items?itemName=coolchyni.beyond-debug) extension
3. Create a folder `.vscode` in your project folder
5. Create a file `launch.json` in the `.vscode` folder with the following content:
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "by-gdb",
            "request": "launch",
            "name": "Launch(remote)",
            "program": "./target/armv7-unknown-linux-gnueabihf/debug/THE_PROCESS_NAME",
            "cwd": "${workspaceRoot}",
            "preLaunchTask": "prepdebug",
            "debuggerPath": "gdb-multiarch",
            "remote": {
                "enabled": true,
                "address": "THE_IP_YOU_WISH_TO_USE:5000",
                "mode": "remote",
                "execfile": "./target/armv7-unknown-linux-gnueabihf/debug/THE_PROCESS_NAME"
            }
        }
    ]
}
```
5. Create a file `tasks.json` in the `.vscode` folder with the following content:
```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "prepdebug",
            "command": "sh",
            "args": [
                "./scripts/prep_debug.sh"
            ],
            "type": "shell"
        }
    ]
}
```
6. Create a folder `scripts` in your project folder
7. Create a file `prep_debug.sh` in the `scripts` folder with the following content:
```sh
TARGETIP=THE_IP_YOU_WISH_TO_USE
EXECUTABLE=THE_PROCESS_NAME
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
```
8. ???
9. Profit

You can now debug your code using the GUI integrated into VSCode.