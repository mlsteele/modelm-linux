# modelm-linux

A Linux **Mechanical keyboard audio simulator** for your keyboard written in [Rust](https://www.rust-lang.org/).

This program is a lazy port of [millerjs](https://github.com/millerjs/modelm)'s awesome original OSX [modelm](https://github.com/millerjs/modelm). This brings the fun to linux, or at least Ubuntu, I haven't tried it on anything else. This clone is far less featureful. The world would be a better place if this were a PR to that project, but this was easier.

> *Get yourself clickity-clacking.*

Inspired by the [IBM Model M Keyboard](https://en.wikipedia.org/wiki/Model_M_keyboard) and a disproportionate love of clicky keyboards over non clicky keyboards, this is a simple program to simulate the Model M by providing audible keystroke feedback.

## Requirements

[Rust](https://github.com/rust-lang/rustup) and [Cargo](https://crates.io/).

Install OpenAL audio dependency.
```bash
sudo apt-get install -y libsndfile1-dev libopenal-dev
```

## Running

```bash
git clone git@github.com:mlsteele/modelm-linux.git
cd modelm-linux
cargo run
```

### Credits

Thanks to [millerjs](https://github.com/millerjs) for the original [modelm](https://github.com/millerjs/modelm) which got me hooked on my virtual clickity-clacking keyboard.

Currently ships with IBM sounds from https://webwit.nl/input/kbsim/
