# `istat`: an i3 (or sway) status_command

I used to use [`i3blocks`](https://github.com/vivien/i3blocks) for `i3`'s `status_command`, but I found that having all
my configuration in separate scripts was getting a little cumbersome.

That, and also I could never find a good block for volume control that wasn't janky or slow.

So, I decided to write my own `status_command` generator, and what better language to write it in than Rust!

- [`istat`: an i3 (or sway) status\_command](#istat-an-i3-or-sway-status_command)
  - [Features](#features)
    - [Screenshots](#screenshots)
  - [Install](#install)
  - [Usage](#usage)
    - [Setting it up](#setting-it-up)
    - [Interacting with `istat`](#interacting-with-istat)
      - [Signals](#signals)
      - [Custom IPC events](#custom-ipc-events)
  - [Development](#development)


## Features

* ⚡ completely single threaded (less resource usage)
  * 🔎 it's a status command, it shouldn't be heavy
* ⏩ powerline theming and customisability
* 🎮 ipc control
  * 🖱️ send click events via a command
  * ♻️ refresh items with a command
  * 📜 custom events for some integrations (e.g., controlling PulseAudio/PipeWire, etc)
  * 🤯 runtime updates - no restart required
* 🖇️ many different bar items (continue reading for screenshots)

Each bar item is configurable, see [the sample config](./sample_config.toml) for options.

### Screenshots

Here's an image of a bar in i3:

![screenshot of i3bar](./.github/assets/full.png)

And another one which `powerline` mode set:

![screenshot of i3bar with powerline](./.github/assets/full-powerline.png)

This table contains screenshots of some bar items:

| item        | description                                                                       | screenshots                                                                                                                                                                                                                                                                                      |
| ----------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `battery`   | Percentage, charging, etc. Supports multiple batteries.                           | ![](./.github/assets/battery_at_5.png) ![](./.github/assets/battery_at_20.png) ![](./.github/assets/battery_at_40.png) ![](./.github/assets/battery_at_60.png) ![](./.github/assets/battery_at_100.png) ![](./.github/assets/battery_charging.png) ![](./.github/assets/battery_full.png)        |
| `cpu`       | Usage expressed as a percentage                                                   | ![](./.github/assets/cpu_at_0.png) ![](./.github/assets/cpu_at_50.png) ![](./.github/assets/cpu_at_67.png) ![](./.github/assets/cpu_at_100.png)                                                                                                                                                  |
| `disk`      | Usage, shows free disk space. Supports multiple mount points.                     | ![](./.github/assets/disk_default.png)                                                                                                                                                                                                                                                           |
| `dunst`     | Displays "do not disturb" status (if it's paused or not)                          | ![](./.github/assets/dunst_on.png) ![off (invisible)](./.github/assets/dunst_off.png)                                                                                                                                                                                                            |
| `kbd`       | Displays CapsLock/Numlock/etc states                                              | ![](./.github/assets/kbd_all_off.png) ![](./.github/assets/kbd_all_on.png) ![](./.github/assets/kbd_caps_on.png) ![](./.github/assets/kbd_num_on.png)                                                                                                                                            |
| `krb`       | Checks if a valid kerberos token exists (like `klist -s`)                         | ![](./.github/assets/krb_off.png) ![](./.github/assets/krb_on.png)                                                                                                                                                                                                                               |
| `mem`       | Display free memory as bytes or as a percentage                                   | ![](./.github/assets/mem_at_100.png) ![](./.github/assets/mem_at_75.png) ![](./.github/assets/mem_free_50.png) ![](./.github/assets/mem_free_100.png)                                                                                                                                            |
| `net_usage` | Upload and download statistics                                                    | ![](./.github/assets/net_usage_no_traffic.png) ![](./.github/assets/net_usage_threshold_1.png) ![](./.github/assets/net_usage_threshold_2.png) ![](./.github/assets/net_usage_threshold_3.png) ![](./.github/assets/net_usage_threshold_4.png) ![](./.github/assets/net_usage_threshold_max.png) |
| `nic`       | Network interface status - connection state and ip addresses                      | ![](./.github/assets/nic_default.png)                                                                                                                                                                                                                                                            |
| `pulse`     | Input/output volume status, full control and current speaker type (jack, bt, etc) | ![](./.github/assets/pulse_default.png)                                                                                                                                                                                                                                                          |
| `script`    | Run arbitrary scripts and show their output                                       | ![](./.github/assets/script_default.png)                                                                                                                                                                                                                                                         |
| `sensors`   | Temperature sensors                                                               | ![](./.github/assets/sensors_at_50.png) ![](./.github/assets/sensors_at_70.png) ![](./.github/assets/sensors_at_80.png) ![](./.github/assets/sensors_at_100.png)                                                                                                                                 |
| `time`      | Displays the current date and/or time                                             | ![](./.github/assets/time_default.png)                                                                                                                                                                                                                                                           |



## Install

With Rust (via cargo):

```sh
cargo install istat
# Make sure to look at the `sample_config.toml` file for configuration options
```

Via the AUR (Arch Linux):

```sh
paru -S istat
```

## Usage

### Setting it up

First, create a config file for `istat`. View [the sample config](./sample_config.toml) for what's available.
This file should be placed in:

* `$XDG_CONFIG_HOME/istat/<here>`, or
* `$HOME/.config/istat/<here>`

Even though the [sample configuration file](./sample_config.toml) is a TOML file, YAML and JSON are also supported.

Then, update your i3/sway config to use `istat` as the `status_command`:

```
bar {
        status_command istat
        # ... other config
}
```

### Interacting with `istat`

`istat` offers multiple ways of interacting with it:

* standard click events from i3/sway
* real-time signals
* it's own ipc

#### Signals

Consider the following bar item which outputs the state of the CapsLock and NumLock keys:

```toml
type = "kbd"
show = ["caps_lock", "num_lock"]
interval = "30s"
```

It refreshes every 30 seconds, or every time the bar item receives a click event. That's alright, but we can do better with signals.
Adding `signal = 8` to the config, and removing `interval` we get:

```toml
type = "kbd"
show = ["caps_lock", "num_lock"]
signal = 8
```

Now, whenever `istat` receives the `SIGRTMIN+8` signal, the bar item will be refreshed.
Pair this with the following config in i3/sway, and you'll have a bar item that reflects your keys all the time:

```
bindsym --release Caps_Lock exec --no-startup-id pkill -RTMIN+8 istat
bindsym --release Num_Lock  exec --no-startup-id pkill -RTMIN+8 istat
```

Linux offers many realtime signals, to see which your machine supports the `istat-signals` command is provided:

```bash
$ istat-signals
{"max":30,"min":0,"sigrtmax":64,"sigrtmin":34}
```

The same signal can be configured for multiple bar items, so you many can be refreshed with the same signal!

#### Custom IPC events

The command `istat-ipc` is provided to interface with `istat`. It supports:

* fetching the name and index of all the currently running bar items
* refreshing all bar items at once
* sending `click` events to each bar item
* sending custom events to bar items
  * some bar items (like `pulse`) expose an advanced API which can be accessed with these events

**Refresh all bar items at once**:

```bash
istat-ipc refresh-all
```

**Send a click event to a bar item - without actually clicking it!**:

```bash
# emulate a left click on the disk item:
istat-ipc click disk left
```

**Control PulseAudio/Pipewire via custom IPC events**:

```bash
# see all the custom events that pulse has to offer:
istat-ipc custom pulse

# Some examples:

# turn the output (speakers) volume up
istat-ipc custom pulse volume-down sink
# turn the input (microphone) volume down
istat-ipc custom pulse volume-up   source
# mute or unmute the output
istat-ipc custom pulse mute-toggle sink
```

## Development

See the [justfile](./justfile)!

Also give [IDEAS.md](./IDEAS.md) a read too.
