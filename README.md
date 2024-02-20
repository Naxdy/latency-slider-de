# Latency Slider (Definitive Edition)

This is a fork of - and an improvement upon - the original "Arena Latency Slider". Unfortunately, upon SSBU updating to 13.0.2, the original author decided to remove the source code from public view, and in its place publish a severely neutered version of the original skyline plugin.

However, the beauty of the Internet is that nothing is truly lost, and the beauty of open source is that anyone can make changes and publish different versions of a piece of software.

Hence, I present to you Latency Slider DE (Definitive Edition).

**IMPORTANT:** This build is for use with SSBU v13.0.2 ONLY!

## Features

- As with the original mod, you can select your desired input delay in any arena (host or otherwise).
- Additionally, you can also select your desired input delay on the character select screen. If you're in the CSS for quickplay / elite, the selected latency will also be displayed at the top.
- The selected input delay will carry over to quickplay / elite / tournament mode.
- You can use D-Pad up / down in the arena screen or CSS to hide / show the input delay text (useful e.g. if you're streaming and afraid to get reported to the Nintendo ninjas).

## How does it work?

Refer to the original author's explanation in [the original README](README_orig.md).


## Building

If you have the necessary tooling (`rustup`, `cargo` and `cargo-skyline`), you can simply run

```shell
cargo skyline build --release
```

If you do NOT have the necessary tooling, the quickest way for you to get set up is to [download and install Nix](https://nixos.org/download), then proceed to [enable Nix flakes](https://nixos.wiki/wiki/Flakes), and run this command to enter a dev shell:

```shell
nix develop .
```

Nix will download and make available all the necessary dev tools for you to build this plugin. Then you can proceed to run the build command from above.

## Contributing

If there are any Nix enjoyers out there who feel like packaging skyline's rust toolchain so we don't have to rely on `cargo-skyline` anymore and can introduce fully reproducible builds, feel free to submit a pull request! If there are any modding enjoyers out there who want to expand upon this plugin, feel free to do the same!

## Final Note

This fork is published under the original project's AGPL license. Though I pinky promise to never take this repo offline or privatize the source code in any way, I encourage you to fork it and re-share it as much as your heart desires.