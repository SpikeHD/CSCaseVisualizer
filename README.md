<div align="center">
  <h1>Caseity</h1>

  <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/Caseity" />
  <img src="https://img.shields.io/github/repo-size/SpikeHD/Caseity" />
</div>

<div align="center">
  <strong>
    A simple CS case visualizer written in Rust and JavaScript
  </strong>
</div>

# Table of Contents

- [Table of Contents](#table-of-contents)
- [Setup](#setup)
- [Getting your Steam Cookie](#getting-your-steam-cookie)
- [Screenshots](#screenshots)
- [Building](#building)
  - [Requirements](#requirements)
  - [Build steps](#build-steps)
- [Contributing](#contributing)

# Setup

Download a [release](https://github.com/SpikeHD/Caseity/releases) and extract it to a folder of your choice. Or, if you don't trust it (that is fair), you can [build it yourself](#building).

\**You should make sure to put it in a folder that does not require admin permissions, in case you want to save your dumps!*

# Getting your Steam Cookie

1. Open your browser's developer tools (F12)
2. Go to the Network tab
3. Go to [Steam](https://steamcommunity.com/)
4. Click on the first request
5. Go to the Headers tab
6. Copy the whole value of the `cookie` field

It should go without saying that giving your cookie to a random program is a bad idea. If you don't trust me, you can [build the program](#building) yourself and check the source code.

# Screenshots

TODO

# Building

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [Yarn](https://classic.yarnpkg.com/en/docs/install)

## Build steps

1. Clone the repository
2. Navigate to the respository's root directory (`cd /path/to/Caseity`)
3. Run `yarn install` to install the JavaScript dependencies
4. Run `yarn tauri build` to build the program

Your built program will be in the `src-tauri/target/release` folder.

# Contributing

Want to make a change? Great! Pull requests, issues, etc. are all welcome!