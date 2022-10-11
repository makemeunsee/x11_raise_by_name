# X11 raise by name

## Description

Raises a X11 window given its name.

This was made for the purpose of conveniently raising the `Zoom` toolbar and floating video window as they tend to disappear behind other windows on my setup (XMonad).

## Requirements

A windows manager running on top of X11.

## Usage

`cargo run -- --wname name_of_my_buried_window`

The names of all X11 windows can be found out with this command:

`xwininfo -root -children -w`

On my setup, the buried Zoom windows have the following names:

* `zoom_linux_float_video_window`
* `as_toolbar`

Logger to stdout can be configured with `RUST_LOG=debug`, for additional output.
