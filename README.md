# dpc's theme (generator)

So many variants of Solarized Dark, all of them slightly sucky...

All I want is Solarized Dark-like background color, and then rest
of the colors (red, yellow, etc.) be what they supposed to be,
with a bright version actually be a bright version of the base color.

Tired of looking, I just fired up Rust, added some oklch color library and wrote
a generator to make exactly what I want (and tweak it
easily). Additional benefit - it's easy generate configs/themes
for any software I might need in the future.

All colors are visible on default background, black is a very dark
gray because of this reason.

Last time I exported it, it looks something like this:

![dpc theme example screenshot](https://i.imgur.com/wLWhegL.png)

Intel One Mono is the font, in case you're wondering.
