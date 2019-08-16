# vacuum

[![Build Status](https://travis-ci.org/idursun/vacuum.svg?branch=master)](https://travis-ci.org/idursun/vacuum)

Vacuum is a system-wide configuration file collector.

This repo contains highly **experimental** code before settling on a final design and far from being complete. Any feedback is welcome.

## How it works?

Vacuum find what files to copy and where to find them by processing _.vacuum_ files. A vacuum file, in essence, is a mini DSL that describes where to look at to find application specific configuration files. You can have a look at the currently available .vacuum files [here](
https://github.com/idursun/vacuum/tree/master/apps).

## How to try?

- Clone the repository
- Run `cargo run -- ./myconfigs` to _vacuum_ configurations into `./myconfigs`
