# addontest

An example plugin for [omegga](https://github.com/brickadia-community/omegga) leveraging [napi] for building [node addons](https://nodejs.org/api/addons.html) in [rust].

## Build

1. Install [rust]
1. Setup [napi] via `npm i -g @napi-rs/cli`
1. Install dependencies `npm i`
1. Build via `npm run build`

## Install

`omegga install gh:meshiest/addontest`

## Usage

`/sum <number> <number>`

[napi]: https://github.com/napi-rs/napi-rs
[rust]: https://www.rust-lang.org/