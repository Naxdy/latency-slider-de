# Latency Slider (Definitive Edition)

This is a fork of - and an improvement upon - the original "Arena Latency Slider". Unfortunately, upon SSBU updating to 13.0.2, the original author decided to remove the source code from public view, and in its place publish a severely neutered version of the original skyline plugin.

However, the beauty of the Internet is that nothing is truly lost, and the beauty of open source is that anyone can make changes and publish different versions of a piece of software.

Hence, I present to you Latency Slider DE (Definitive Edition).

## Features / Usage

- As with the original mod, you can select your desired input delay in any arena (host or otherwise).
- Additionally, you can also select your desired input delay on the character select screen. If you're in the CSS for quickplay / elite, the selected latency will also be displayed at the top. However, you can (blindly) adjust your input latency on _any_ CSS, not just the quickplay one (however, only the quickplay one will _display_ your input latency).
- The selected input delay will carry over to quickplay / elite / tournament mode.
- You can use D-Pad up / down in the arena screen or CSS to hide / show the input delay text (useful e.g. if you're streaming and afraid to get reported to the Nintendo ninjas).

## How does it work?

Refer to the original author's explanation in [the original README](README_orig.md).

## How to install?

You will need a moddable switch with atmosphere already installed on it. If you don't have this yet (or don't even know what I'm talking about), look into Switch modding in general first, then come back here.

1. Download and install [Skyline](https://github.com/skyline-dev/skyline) by copying the `exefs` directory into the folder `atmosphere/contents/01006A800016E000` (create if it doesn't exist yet)
2. Download the latest `liblatency_slider_de.nro` from [this page](https://github.com/xNaxdy/latency-slider-de/releases)

> [!NOTE]
> This project supports reproducible builds. If you want to ensure that the `.nro` uploaded to the Releases page is actually built from the source available, you can build the project yourself very easily. See the instructions in the "Building" section below.

3. Copy the `liblatency_slider_de.nro` to your SD card, into the folder `atmosphere/contents/01006A800016E000/romfs/skyline/plugins` (create if it doesn't exist yet)
4. Boot up the game, go online, and open up an arena. Confirm the mod is working by observing the room id text in the top right (it should show you your input latency now).

Congratulations, Smash Ultimate online is now actually playable!

### What is `liblatency_slider_de_classic.nro`?

The "classic" version of the plugin is closer to the original latency slider in functionality:

- You can only see and adjust your desired latency in arenas, D-pad inputs on any CSS are ignored.
- The latency you select in arenas carries over to all gamemodes, including quickplay / elite.

The reason you'd pick classic over mainline is mod compatibility. Because it's much more barebones, it has a higher chance of being compatible with big, complex mods like the CSK collection. If you don't have any issues with mod compatibility however, there is no reason to use "classic" over the regular version.

## Building

The project can be reproducibly built using only the Nix package manager.

The quickest way for you to get set up is to [download and install Nix](https://nixos.org/download) (the package manager, not necessarily the entire NixOS distribution), then proceed to [enable Nix flakes](https://nixos.wiki/wiki/Flakes), and run this command to build the package:

```shell
nix build .#

# or use this command, which doesn't even require you to clone this repository:

nix build github:Naxdy/latency-slider-de
```

Nix will take care of the rest, that is downloading the necessary toolchain, build tools, and dependencies, as well as compile everything for you. The resulting library will be put in `./result/lib/liblatency_slider_de.nro`, which you can then upload to your Switch, or copy to your SD card as usual.

## Contributing

If you have Nix installed (see above), you can enter a fully-featured dev shell using:

```shell
nix develop .
```

This will download and make available all tooling and dependencies that you need to begin work on the project.

If there are any modding enjoyers out there who want to expand upon this plugin, feel free to open a pull request!

## FAQ

#### Am I allowed to modify / redistribute this software?

Yes, so long as you also publish the source code of your modification. View details in the [license](LICENSE).

#### Will this format my SD Card?

No. But don't take my word for it, the source is available for anyone to see. Feel free to analyze it yourself, or have someone you trust do it for you. Don't use my binary, but compile it yourself (or, again, have someone you trust do it for you). This is one of the benefits of open source, you don't _have_ to trust, you can verify.

Additionally, the project is set up to support reproducible builds, which means that if you build the project locally using Nix, you will get the exact same binary, bit-by-bit, as someone else building from the same source.

#### Will I get banned if I use this mod on qp / in arenas?

The short answer is "I don't know, but probably not". The slightly longer answer is that using any mod (online or offline) carries risks with it. However, I think that the biggest risk when using this mod is other people finding out you have it (e.g. because you publicly posted a screenshot with the latency slider text visible) and proceeding to mass report you to Nintendo. This is the main reason why I've opted to implement the "stealth mode" functionality into this mod.

Like the original, the mod doesn't send any extra network packets (arenas or otherwise), so short of Nintendo scanning your SD card, there shouldn't be any way to automatically detect it.

#### Does this mod send any additional network packets?

No. See above.

#### Is this mod compatible with the original arena latency slider, or other latency slider mods?

NO. These essentially do the same thing but differently, running multiple of these mods at the same time is very likely to cause your game to crash. If you want to run this mod, you should remove / disable all other "latency slider" type mods first.

#### Is this mod compatible with the VSync mod?

The VSync mod, aka "1 frame less delay", aka `less-delay` is compatible with this mod. I maintain a repo for that one as well, over here: https://github.com/Naxdy/less-delay

#### Is this mod compatible with the Ultimate Training Modpack?

Yesn't. Both Latency Slider DE and the Ultimate Training Modpack attempt to hook the game's `draw` function, by trying to find its signature. The thing is, once one of the mods hooks that function, its signature changes, so whichever mod is loaded _second_ will fail to find that function. Therefore, if Latency Slider DE detects that you have Ultimate Training Modpack installed, it will display a warning message letting you know that it will run with reduced functionality in order to avoid crashing the game, making it so that it will not be able to display the input latency on the character select screen in Elite Smash.

It will, however, remain fully functional otherwise (including applying your selected latency in all online modes), and you will still be able to select your desired latency in arenas, as well as adjust it "blindly" on any CSS.

#### Is this mod compatible with [insert other mod here]?

Don't know, don't care. SSBU modding is kind of like chemistry, except you don't know the chemicals you're using. Any concoction has the chance to blow up in your face. The answer is: Try it. If it works, great. If not, you'll have to pick which mod you want enabled.

I _highly_ suggest using a mod manager of some sort to enable / disable mods as you need, to avoid potential complications.

There's also the `liblatency_slider_classic.nro` version of the plugin, which is a more barebones build, closer in functionality of the original latency slider (with the exception that it too works in all online modes). Because it is more barebones, there's a higher chance that it may work together with mods that present issues with the mainline latency slider.

Unfortunately I can't really invest any time into ensuring that latency-slider-de is compatible with each and every mod out there. However, if you submit a PR that improves compatibility with other mods, I will happily accept it!

#### Are you fine with people using this mod to cheat in online tournaments, or on qp?

The short answer is "how people use this mod is up to them". The long answer is this: First, for quickplay, I don't view using this mod as cheating, but rather making the game playable, since there's nothing on the line (if you think GSP holds any meaningful value, I suggest an immediate and thorough psych evaluation), same as in arenas.

When it comes to online tournaments, for those without prizes my views are the same as those for quickplay. Now, for the others, I am of the opinion that those with significant prizes should have died together with COVID lockdowns. The reason is that online, even without any sort of mods, is inherently random and unfair. Network interference, packet loss, and all the fun things aside, since SSBU determines the input delay on the client side, it is entirely possible that two players play the game with different input delays as is, even without the latency slider. So, if anything, latency slider makes online tournaments _fairer_, but only if both players have it and set it to the same delay, of course. Note that I said _fairer_ and not _fair_, because you still have random lag spikes which may impact one player more than the other (e.g. if a huge lag spike happens during Ness' PK thunder recovery, or during a frame-tight combo).

Additionally, even without latency slider, there are mods (software and hardware) you can use to reduce your input delay when playing with others (both on quickplay and in arenas). `less-delay` just shaves off 1 frame of input delay no matter the game mode (even offline), and there are 3rd party GC adapters that, at best, improve input delay by another frame and, at worst, make the controller input delay much more consistent than the OG.

The bottom line here is that online tournaments have never been - and will never be - "fair", and if there's a will to cheat, there's a way (even without this mod). Unless you can ensure a 100% stable connection, equal and consistent input delay for both parties, and proctor all players to ensure they aren't using any special hardware to improve their play, this entire discussion is pointless to me.

#### Did you steal the source code of this plugin?

No. This project's source code has been licensed under the AGPL by the original authors, which is a license that grants everyone the right to view, modify, and redistribute the project's source code, so long as the original license is preserved. See [this project's license](./LICENSE), as well as [the license for HDR](https://github.com/HDR-Development/HewDraw-Remix/blob/pre-release/LICENSE) - the project this functionality was originally developed for.

I would also like to stress at this point that the AGPL regards "network interaction" as "distribution", meaning that if you download the source code for this plugin, modify it locally, and then use it to play online with others, you _have_ to publish your modifications to the source code, otherwise you are in violation of the license. According to section 13 of the [license](./LICENSE):

> Notwithstanding any other provision of this License, if you modify the Program, your modified version must prominently offer all users interacting with it remotely through a computer network (if your version supports such interaction) an opportunity to receive the Corresponding Source of your version by providing access to the Corresponding Source from a network server at no charge, through some standard or customary means of facilitating copying of software. This Corresponding Source shall include the Corresponding Source for any work covered by version 3 of the GNU General Public License that is incorporated pursuant to the following paragraph.

## Final Note

This fork is published under the original project's AGPL license. Though I pinky promise to never take this repo offline or privatize the source code in any way, I encourage you to fork it and re-share it as much as your heart desires.
