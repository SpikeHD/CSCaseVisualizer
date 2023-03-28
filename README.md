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

If it freezes up, feel free to just restart the program. If you have a vast CS inventory history, the program will probably take a little while.

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

<details>
  <summary>Expand for screenshots</summary>
  <div align="center">
    <img height="800" src="https://user-images.githubusercontent.com/25207995/228137744-fe37cf38-3b4b-4965-b87c-8feac2c4fc34.png" />
    <img height="800" src="https://user-images.githubusercontent.com/25207995/228137788-6cebf89f-711a-4361-862c-d0ff19d0575f.png" />
    <img height="800" src="https://user-images.githubusercontent.com/25207995/228137689-6c94f7ba-b475-407b-b1be-11e7797ed9e6.png" />
  </div>
</details>

# Building

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [Yarn](https://classic.yarnpkg.com/en/docs/install)

## Build steps

1. Clone the repository
2. Navigate to the respository's root directory (`cd /path/to/Caseity`)
3. Run `yarn install` to install the JavaScript dependencies
4. Run `cargo install` to install Rust dependencies
5. Run `yarn tauri build` to build the program

Your built program will be in the `src-tauri/target/release` folder.

# Contributing

Want to make a change? Great! Pull requests, issues, etc. are all welcome!

# Thank you
* https://github.com/cantryDev/CSGOCaseStatsViewer (helped in the initial kick-off of this project)
