use serde_json::json;

use crate::i3::X11Test;
use crate::util::Test;

// TODO: use these fake_root mocks in actual tests
//  ie: re-use these in `spawn` tests and check json

macro_rules! screenshot {
    // batch case (many fake_root mocks)
    (
        $test_name:ident,
        $item_json:expr
        $(,
            [
                $(
                    $case_name:ident => {
                        $(bin $bname:expr => $bdata:expr,)*
                        $(r $fname:expr => $fdata:expr,)*
                    }$(,)?
                )+
            ]
        )+
    ) => {
        $(
            $(
                paste::paste! {
                    screenshot!(
                        [<$test_name _ $case_name>],
                        $item_json,
                        bins = {
                            $($bname => $bdata)*
                        },
                        roots = {
                            $($fname => $fdata)*
                        }
                    );
                }
            )*
        )?
    };

    // single case
    (
        $test_name:ident,
        $item_json:expr,
        bins = {
            $($bname:expr => $bdata:expr$(,)?)*
        },
        roots = {
            $($fname:expr => $fdata:expr$(,)?)*
        }
    ) => {
        x_test!(
            $test_name,
            {
                // disable separator
                let mut obj = $item_json;
                let map = obj.as_object_mut().unwrap();
                map.insert("separator".into(), false.into());

                // insert item afterwards for some artificial padding
                // done this way because some nerd fonts clip if it's the last item
                json!({ "items": [obj, { "type": "raw", "full_text": "" }] })
            },
            |_test: &mut Test| {
                $(
                    _test.add_bin($bname, $bdata);
                )*
                $(
                    _test.add_fake_file($fname, $fdata);
                )*
            },
            |x_test: X11Test| {
                x_test.screenshot("bar-0");
            }
        );
    };
}

// battery ---------------------------------------------------------------------

screenshot! {
    battery,
    json!({
        "type": "battery",
        "interval": "1s",
        "batteries": [
            "/sys/class/power_supply/BAT0",
            "/sys/class/power_supply/BAT0"
        ],
    }),
    [
        at_100 => {
            r "/sys/class/power_supply/BAT0/charge_now" => "100",
            r "/sys/class/power_supply/BAT0/charge_full" => "100",
            r "/sys/class/power_supply/BAT0/status" => "Discharging",
        },
        at_60 => {
            r "/sys/class/power_supply/BAT0/charge_now" => "60",
            r "/sys/class/power_supply/BAT0/charge_full" => "100",
            r "/sys/class/power_supply/BAT0/status" => "Discharging",
        },
        at_40 => {
            r "/sys/class/power_supply/BAT0/charge_now" => "40",
            r "/sys/class/power_supply/BAT0/charge_full" => "100",
            r "/sys/class/power_supply/BAT0/status" => "Discharging",
        },
        at_20 => {
            r "/sys/class/power_supply/BAT0/charge_now" => "20",
            r "/sys/class/power_supply/BAT0/charge_full" => "100",
            r "/sys/class/power_supply/BAT0/status" => "Discharging",
        },
        at_5 => {
            r "/sys/class/power_supply/BAT0/charge_now" => "5",
            r "/sys/class/power_supply/BAT0/charge_full" => "100",
            r "/sys/class/power_supply/BAT0/status" => "Discharging",
        },
        charging => {
            r "/sys/class/power_supply/BAT0/charge_now" => "10",
            r "/sys/class/power_supply/BAT0/charge_full" => "100",
            r "/sys/class/power_supply/BAT0/status" => "Charging",
        }
        full => {
            r "/sys/class/power_supply/BAT0/charge_now" => "100",
            r "/sys/class/power_supply/BAT0/charge_full" => "100",
            r "/sys/class/power_supply/BAT0/status" => "Full",
        }
    ]
}

// cpu -------------------------------------------------------------------------

screenshot! {
    cpu,
    json!({
        "type": "cpu",
        "interval": "1s",
    }),
    // /proc/stat's values are
    // cpu_id user nice system idle iowait irq softirq steal guest guest_nice
    // for sysinfo's calculations:
    // see: https://github.com/GuillaumeGomez/sysinfo/blob/2fa03b052e92f4d8ce90e57c548b1732f848dbbd/src/linux/cpu.rs
    [
        at_0 => {
            r "/proc/stat" => "cpu  0 0 0 0 0 0 0 0 0 0",
        },
        at_50 => {
            r "/proc/stat" => "cpu  1 0 0 1 0 0 0 0 0 0",
        },
        at_67 => {
            r "/proc/stat" => "cpu  2 0 0 1 0 0 0 0 0 0",
        },
        at_100 => {
            r "/proc/stat" => "cpu  1 0 0 0 0 0 0 0 0 0",
        },
    ]
}

// disk ------------------------------------------------------------------------
// TODO: mock for tests

screenshot! {
    disk,
    json!({
        "type": "disk",
        "interval": "1s",
    }),
    [
        todo => {}
        // TODO: first checks /proc/mounts, and then uses statvfs
        //  maybe add option to point to disk, and create virtual disk? rather than intercepting statvfs?
    ]
}

// dunst -----------------------------------------------------------------------
// TODO: mock for tests

screenshot!(dunst, json!({ "type": "dunst" }), [todo => {}]);

// bar -------------------------------------------------------------------------
// TODO: sample config ?

// screenshot!(bar, json!({}));

// kbd -------------------------------------------------------------------------

screenshot! {
    kbd,
    json!({
        "type": "kbd",
        "show": ["caps_lock", "num_lock", "scroll_lock"]
    }),
    [
        caps_on => {
            r "/sys/class/leds/input0::capslock/brightness" => "1",
            r "/sys/class/leds/input0::numlock/brightness" => "0",
            r "/sys/class/leds/input0::scrolllock/brightness" => "0",
        },
        num_on => {
            r "/sys/class/leds/input0::capslock/brightness" => "0",
            r "/sys/class/leds/input0::numlock/brightness" => "1",
            r "/sys/class/leds/input0::scrolllock/brightness" => "0",
        },
        all_on => {
            r "/sys/class/leds/input0::capslock/brightness" => "1",
            r "/sys/class/leds/input0::numlock/brightness" => "1",
            r "/sys/class/leds/input0::scrolllock/brightness" => "1",
        },
        all_off => {
            r "/sys/class/leds/input0::capslock/brightness" => "0",
            r "/sys/class/leds/input0::numlock/brightness" => "0",
            r "/sys/class/leds/input0::scrolllock/brightness" => "0",
        },
        one_err => {
            r "/sys/class/leds/input0::capslock/brightness" => "1",
            r "/sys/class/leds/input0::numlock/brightness" => "0",
        }
    ]
}

// krb -------------------------------------------------------------------------

screenshot!(
    krb,
    json!({
        "type": "krb",
        "interval": "1s",
    }),
    [
        off => {
            bin "klist" => "#!/usr/bin/env bash\nexit 0",
        },
        on => {
            bin "klist" => "#!/usr/bin/env bash\nexit 1",
        }
    ]
);

// mem -------------------------------------------------------------------------
// TODO: mock for tests

screenshot!(
    mem,
    json!({
        "type": "mem",
        "interval": "1s",
    }),
    // for sysinfo calculations:
    // see: https://github.com/GuillaumeGomez/sysinfo/blob/2fa03b052e92f4d8ce90e57c548b1732f848dbbd/src/linux/system.rs#L257
    [
        at_100 => {
            r "/proc/meminfo" => r#"\
MemTotal:      31250000 kB
MemFree:              0 kB
MemAvailable:  31250000 kB
Buffers:              0 kB
Cached:               0 kB
Shmem:                0 kB
SReclaimable:         0 kB
SwapTotal:     31250000 kB
SwapFree:      31250000 kB
"#,
        },
        at_75 => {
            r "/proc/meminfo" => r#"\
MemTotal:      31250000 kB
MemFree:              0 kB
MemAvailable:  23437500 kB
Buffers:              0 kB
Cached:               0 kB
Shmem:                0 kB
SReclaimable:         0 kB
SwapTotal:     31250000 kB
SwapFree:      31250000 kB
"#,
        },
        at_50 => {
            r "/proc/meminfo" => r#"\
MemTotal:      31250000 kB
MemFree:              0 kB
MemAvailable:  15625000 kB
Buffers:              0 kB
Cached:               0 kB
Shmem:                0 kB
SReclaimable:         0 kB
SwapTotal:     31250000 kB
SwapFree:      31250000 kB
"#,
        },
        at_25 => {
            r "/proc/meminfo" => r#"\
MemTotal:      31250000 kB
MemFree:              0 kB
MemAvailable:   7812500 kB
Buffers:              0 kB
Cached:               0 kB
Shmem:                0 kB
SReclaimable:         0 kB
SwapTotal:     31250000 kB
SwapFree:      31250000 kB
"#,
        },
        at_0 => {
            r "/proc/meminfo" => r#"\
MemTotal:      31250000 kB
MemFree:              0 kB
MemAvailable:         0 kB
Buffers:              0 kB
Cached:               0 kB
Shmem:                0 kB
SReclaimable:         0 kB
SwapTotal:     31250000 kB
SwapFree:     31250000 kB
"#,
        }
    ]
);

// net_usage -------------------------------------------------------------------
// TODO: mock for tests
// TODO: pass thresholds for colours

screenshot!(
    net_usage,
    json!({
        "type": "net_usage",
        "interval": "1s",
    }),
    [todo => {}]
);

// nic -------------------------------------------------------------------------
// TODO: mock for tests

screenshot!(nic, json!({ "type": "nic" }), [todo => {}]);

// pulse -----------------------------------------------------------------------
// TODO: mock for tests

screenshot!(pulse, json!({ "type": "pulse" }), [todo => {}]);

// raw -------------------------------------------------------------------------

screenshot!(
    raw,
    json!({
        "type": "raw",
        "full_text": "Hello, World!",
        "color": "#ff0000",
    }),
    [todo => {}]
);

// script ----------------------------------------------------------------------

screenshot!(
    script,
    json!({
        "type": "script",
        "command": "echo -n hello",
        "output": "simple",
    }),
    [todo => {}]
);

// sensors ---------------------------------------------------------------------
// TODO: mock for tests

screenshot!(
    sensors,
    json!({
        "type": "sensors",
        "interval": "1s",
        // TODO: use istat-sensors and pick one
        "label": "coretemp Package id 0"
    }),
    [todo => {}]
);

// time ------------------------------------------------------------------------

screenshot!(
    time,
    json!({
        "type": "time",
        "interval": "1 s",
        "format_long": "%Y-%m-%d %H:%M:%S",
        "format_short": "%H:%M"
    }),
    [todo => {}]
);
