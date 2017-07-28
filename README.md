Status: `arping` works!

# Overview

A basic implementation of a user-space network stack.

This will only work under linux due to the way the `tuntap` device is setup.

This will need to run as root in order to create the `tuntap` device.

Usage:

```
$ sudo cargo run
```

From another terminal:

```
$ arping -I tap1 10.0.0.1
```
