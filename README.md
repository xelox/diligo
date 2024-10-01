# Diligo

Diligo is a very simple CLI application that helps keeping track of how you spend time on your computer.
Diligo is not designed to be the most feature ritch or the most cross platform tool out there.
I wrote Diligo mainly to help myself measure my own productivity, on my own machine.

## Building From Source
Required: `cargo`
clone this repository
navigate inside the `./cli` directory and the `./service` directory, and in both run:
```bash
cargo install --path .
```
this will compile the binaries and install them in your `~/.cargo/bin` directory
To enable the diligo daemon modify and copy the `./service/diligo_daemon_example.service` file into `/etc/systemd/system/`.
Then enable the service by running:
```bash
sudo systemctl daemon-reload
sudo systemctl enable diligo_daemon
sudo systemctl start diligo_daemon
```
If you don't want to create a service, you may simply run the `diligo_daemon` in other ways.

## Usage
To print the current task/state and the total time spent on it since the daemon started run:
```bash
diligo total
```

To print the current task/state and the total time spent on it since the last time the task was changed run:
```bash
diligo session
```

To change the task run:
```bash
diligo set [new task name]
```
*note:* it's ok for the task name to contain white space or special characters/emojies. anything after "set " will be the new task.

To toggle the current task run:
```bash
diligo set [task name]
```
If the current task it's already the new task, then it will "toggle" it by changing to the default task " Arch" (I use arch btw.)

## How I use diligo
Although diligo is a console application, I mostly use it as a custom component to [waybar](https://github.com/Alexays/Waybar).
Here is how it looks:
![Waybar](https://github.com/xelox/diligo/blob/main/waybar.png?raw=true)
The Text in the middle is the actual point.
I also have a keybind that executes a diligo toggle command, this way it's easy to set the current task to what I really want to track: Time spend coding.

## Other possible usages
Another way to use diligo is by integrating it into nvim lualine, or any other status bar of any other application that is customizable enough to allow it.

## Missing but wanted features
1. Detecting *idle* activity.
2. Periodically saving in a *sqlite database*.
3. Printing a nice report to console.
