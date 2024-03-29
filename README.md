# Latency Slider (Definitive Edition)

This is a fork of - and an improvement upon - the original "Arena Latency Slider". Unfortunately, upon SSBU updating to 13.0.2, the original author decided to remove the source code from public view, and in its place publish a severely neutered version of the original skyline plugin.

However, the beauty of the Internet is that nothing is truly lost, and the beauty of open source is that anyone can make changes and publish different versions of a piece of software.

Hence, I present to you Latency Slider DE (Definitive Edition).

**IMPORTANT:** This build is for use with SSBU v13.0.2 ONLY!

## Features / Usage

- As with the original mod, you can select your desired input delay in any arena (host or otherwise).
- Additionally, you can also select your desired input delay on the character select screen. If you're in the CSS for quickplay / elite, the selected latency will also be displayed at the top. However, you can (blindly) adjust your input latency on *any* CSS, not just the quickplay one (however, only the quickplay one will *display* your input latency).
- The selected input delay will carry over to quickplay / elite / tournament mode.
- You can use D-Pad up / down in the arena screen or CSS to hide / show the input delay text (useful e.g. if you're streaming and afraid to get reported to the Nintendo ninjas).

## How does it work?

Refer to the original author's explanation in [the original README](README_orig.md).

## How to install?

You will need a moddable switch with atmosphere already installed on it. If you don't have this yet (or don't even know what I'm talking about), look into Switch modding in general first, then come back here.

1. Download and install [Skyline](https://github.com/skyline-dev/skyline) by copying the `exefs` directory into the folder `atmosphere/contents/01006A800016E000` (create if it doesn't exist yet)
2. Download the latest `libzlatency_slider_de.nro` from [this page](https://github.com/xNaxdy/latency-slider-de/releases)

    Optional: If you have the knowledge, you can also download `libzlatency_slider_de.sig` and use this file to verify the integrity of `libzlatency_slider_de.nro`. This is mostly useful if you've downloaded the `.nro` from a different place and want to verify that it is one built by me. You can safely skip this optional step however, if you have no idea what I'm talking about here.
3. Copy the `libzlatency_slider_de.nro` to your SD card, into the folder `atmosphere/contents/01006A800016E000/romfs/skyline/plugins` (create if it doesn't exist yet)
4. Boot up the game, go online, and open up an arena. Confirm the mod is working by observing the room id text in the top right (it should show you your input latency now).

Congratulations, Smash Ultimate online is now actually playable!

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

## FAQ

#### Am I allowed to modify / redistribute this software?
Yes, so long as you also publish the source code of your modification. View details in the [license](LICENSE).

#### Will this format my SD Card?
No. But don't take my word for it, the source is available for anyone to see. Feel free to analyze it yourself, or have someone you trust do it for you. Don't use my binary, but compile it yourself (or, again, have someone you trust do it for you). This is one of the benefits of open source, you don't *have* to trust, you can verify.

#### Will I get banned if I use this mod on qp / in arenas?
The short answer is "I don't know, but probably not". The slightly longer answer is that using any mod (online or offline) carries risks with it. However, I think that the biggest risk when using this mod is other people finding out you have it (e.g. because you publicly posted a screenshot with the latency slider text visible) and proceeding to mass report you to Nintendo. Like the original, the mod doesn't send any extra network packets (arenas or otherwise), so short of Nintendo scanning your SD card, there shouldn't be any way to automatically detect it.

#### Does this mod send any additional network packets?

No. See above.

#### Is this mod compatible with the training mode modpack?
Yesn't. `latency-slider-de` and `training-modpack` both hook the draw function of the game in the same place. While this hasn't caused any issues for me or anyone else I've talked to running both mods at once (since v0.2.1), this isn't technically supported. If you notice getting crashes, I suggest disabling one of the two, depending on your current use case.

#### Is this mod compatible with the original arena latency slider, or other latency slider mods?

NO. These essentially do the same thing but differently, running multiple of these mods at the same time is very likely to cause your game to crash. If you want to run this mod, you should remove / disable all other "latency slider" type mods first.

#### Is this mod compatible with the VSync mod?

The VSync mod, aka "1 frame less delay", aka `less-delay` is compatible with this mod. I maintain a repo for that one as well, over here: https://github.com/xNaxdy/less-delay

#### Are you fine with people using this mod to cheat in online tournaments, or on qp?
The short answer is "how people use this mod is up to them". The long answer is this: First, for quickplay, I don't view using this mod as cheating, but rather making the game playable, since there's nothing on the line (if you think GSP holds any meaningful value, I suggest an immediate and thorough psych evaluation), same as in arenas.

When it comes to online tournaments, for those without prizes my views are the same as those for quickplay. Now, for the others, I am of the opinion that those with significant prizes should have died together with COVID lockdowns. The reason is that online, even without any sort of mods, is inherently random and unfair. Network interference, packet loss, and all the fun things aside, since SSBU determines the input delay on the client side, it is entirely possible that two players play the game with different input delays as is, even without the latency slider. So, if anything, latency slider makes online tournaments *fairer*, but only if both players have it and set it to the same delay, of course. Note that I said *fairer* and not *fair*, because you still have random lag spikes which may impact one player more than the other (e.g. if a huge lag spike happens during Ness' PK thunder recovery, or during a frame-tight combo).

Additionally, even without latency slider, there are mods (software and hardware) you can use to reduce your input delay when playing with others (both on quickplay and in arenas). `less-delay` just shaves off 1 frame of input delay no matter the game mode (even offline), and there are 3rd party GC adapters that, at best, improve input delay by another frame and, at worst, make the controller input delay much more consistent than the OG.

The bottom line here is that online tournaments have never been - and will never be - "fair", and if there's a will to cheat, there's a way (even without this mod). Unless you can ensure a 100% stable connection, equal and consistent input delay for both parties, and proctor all players to ensure they aren't using any special hardware to improve their play, this entire discussion is pointless to me.

## Final Note

This fork is published under the original project's AGPL license. Though I pinky promise to never take this repo offline or privatize the source code in any way, I encourage you to fork it and re-share it as much as your heart desires.