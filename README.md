Well, I suppose that the cat is out of the bag, yeah? Someone has ported the input latency adjustment part of HDR to vanilla.

Since I co-authored this feature, I think it is fair that I explain how it works so that misinformation about it being "fair" or anything like that doesn't spread.

# Client Side Only
Honestly, we aren't quite sure why the SSBU development team did not synchronize this between players, it should have been one of the key things they did synchronize to ensure fair play.

The easiest way to explain this is:
- Player A: Has the mod and has set their latency to 2 frames.
- Player B: Does not have the mod, Smash Ultimate assigns them 5 frames by default.

Assuming there is no stuttering in this game, Player A is about to play the *same exact game* as player B, but with 3 frames less (48 milliseconds) delay than player B.

This is very problematic. It means that in any online situation, one player can have a very real and very influential advantage.

48ms is far less than the average human reaction time, for sure. However in vanilla Ultimate online, you are guaranteed a minimum of 10 frames of input delay: 2 frames from the game being triple buffered, 4 frames from the Nintendo Switch's input drivers being slow, and 4 frames as the bare minimum amount you can get online. When are dealing with a reference value of 10 frames, any single frame makes a big impact on both game feel and ability to react. This advantage is real, and will hurt competition.

# "How does it not lag?"
This gets a little bit complex, but bare with me.

It is not feasible, in a production environment, to synchronize enough information between all parties in order to properly get one-way ping. This means that when your ping is calculated, it is calculated as a round trip time between you and the other party. What does this mean?

It means that the ping that you see is calculated via something like this:
```
roundtrip_ping = your_ping_to_them + their_ping_to_you
```
`your_ping_to_them` and `their_ping_to_you` we don't know the value of, only their sum.

So we do the next best thing and average it, the ping you get reported is
```
your_ping = roundtrip_ping / 2 = (your_ping_to_them + their_ping_to_you) / 2
```
So let's think about what that means if your calculated ping to another player is supposedly 120ms
```
your_ping = 120ms
120ms = roundtrip_ping / 2
240ms = roundtrip_ping
240ms = your_ping_to_them + their_ping_to_you
```

With those last two values, anything that properly sums to 240ms are valid, meaning one player could have `40ms` one-way ping and the other could have `200ms` one-way ping. This allows players to have different input latency on client side. Ideally they would be synchronized online to whichever one is higher to ensure fair play, however it's a safe assumption on the Ultimate devs' side of things that the roundtrip ping would be the same between both players and therefore both values of input latency would be identical

# So why does this matter?
When we released this for HewDraw Remix (please just call it HDR guys I beg), there were a few things that we had going in our environment that we could, not necessarily safely, but fairly ignore the glaring issues with introducing something like this:
- It was included in the HDR plugin, so it could not be disabled or enabled directly
- HDR disables the quickplay button, meaning you cannot play quickplay at all.
- HDR immediately desyncs if the other player in a battle arena does not have HDR installed.

All of these combined ensure that in any situation where you are able to use the online latency adjuster, the other player(s) are able to use it as well.

That's no longer the case.

The released plugin (which, as a side note, violated HDR license by not including the modified source code. they also attributed the feature to the wrong plugin, good job random GameBanana user) was advertised to be able to work on quickplay as long as you set the latency in an arena first. You can play with literally anybody, whether that is MkLeo or little Timmy who just got a Switch for Christmas. Playing with *anybody* and giving yourself an advantage is the problem here.

# "So you care about competitive integrity?"
Hell no. I don't, basically none of the HDR team does (at least for WiFi tournaments). But you know who does? Nintendo.

You know who hosts tournaments, even if they are basically for scraps, on WiFi? Nintendo.

This kind of modding could also very easily be considered illegal in Japan, (where it would have a much bigger impact as their network infrastructure and geographical location is much better for this kind of mod), so it is entirely likely that if this gets spread around and used for illegitimate reasons, Nintendo will take action against the users, and if they update Smash again likely against the modding scene.

It's frustrating that something I've worked on even has the potential to bring about this result but it is what it is, there's no going back at this point.

This Twitter thread does a great job explaining what I am too emotionally exhausted to present: https://twitter.com/willy_crescent/status/1602701257365389312

# "How many frames does it actually save?"
In offline Ultimate, there are 6 frames of input delay on a good monitor. 2 of those are from the game's graphics pipeline being triple-buffered, and the other 4 are from the Nintendo Switch's input drivers.

When playing online, Ultimate assigns you a bare minimum of 4 frames of additional input delay, with more being added on depending on the ping (see above) you have to the other player(s) you are playing with.

The mod allows you to pick any number for those additional frames, between 0 and 25. If both players have a good connection, i.e. fiber + LAN or something equivalent, and are somewhat close geographically, you can set that number down to 2f consistently, sometimes 1f. I (AZ) can play with someone else (CA) consistently on 2f and then when my internet is having a good day I can play on 1f.