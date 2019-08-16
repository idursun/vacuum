# vacuum

[![Build Status](https://travis-ci.org/idursun/vacuum.svg?branch=master)](https://travis-ci.org/idursun/vacuum)

Vacuum is a system-wide configuration file collector.

This repo contains experimental work towards a final design.

## How it works?

Vacuum find what files to copy and where to find them by processing _.vacuum_ files. 

https://github.com/idursun/vacuum/tree/master/apps

A vacuum file, in essence, is a mini DSL that describes where to look at to find application specific configuration files.


