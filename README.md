This repository contains the code for the lessons of the excellent
[Beginning Game Programming v2.0](http://lazyfoo.net/tutorials/SDL/)
from lazyfoo.net ported to [rust](http://www.rust-lang.org/).

# Requirements

* libsdl2
* libsdl2-image
* rust
* cargo

# Running the lessons

Each lesson compiles to its own binary. Run `cargo build` to build all the
binaries, which you'll find in the `target` folder.

# Missing Lessons

Not all lessons have been ported. Unless stated otherwise, I've just not got
around to it yet.

* Lesson 16: [rust-sdl2_ttf](https://github.com/andelf/rust-sdl2_ttf) fails to
  compile against the current rust master;
* Lessons 19 and 20: I don't have a joystick (yet);
