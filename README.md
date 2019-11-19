# vacuum

[![Build Status](https://travis-ci.org/idursun/vacuum.svg?branch=master)](https://travis-ci.org/idursun/vacuum)

Vacuum is a system-wide configuration file collector.

This repo contains highly **experimental** code before settling on a final design and far from being complete. Any feedback is welcome.

## How it works?

Vacuum processes _.vacuum_ files.

A vacuum file, in essence, is a mini DSL that describes where to look at to find application specific configuration files.  

Vacuum files are written per application. An example vacuum file for **WebStorm** is as follows:

```
app "WebStorm" {
    home {
        search ".WebStorm*" {
            cd "config" {
                cd "keymaps" {
                    files "*.xml"
                }
                cd "options" {
                    file "editor.xml"
                }
            }
        }
    }
}
```

By processing this file, vacuum will:
- Create a _WebStorm_ folder to store found configuration files
- Change to the home folder
- Search for directories matching the pattern `.WebStorm` and for each found directory:
    - Change directory to config/keymaps
        - Copy all files matching the pattern `*.xml`
    - Change directory to config/options
        - Copy file with the name `editor.xml`

You can have a look at the currently available _.vacuum_ files [here](https://github.com/idursun/vacuum/tree/master/apps).

An example repository with collected configs is at [here](https://github.com/idursun/configs)

## How to run?

- Clone the repository
- Run `cargo run -- store ./myconfigs` to _vacuum_ configurations into `./myconfigs`
- Run `cargo run -- restore ./myconfig` to restore your configurations from `./myconfig`

[![asciicast](https://asciinema.org/a/263745.svg)](https://asciinema.org/a/263745)
