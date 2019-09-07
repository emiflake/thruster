# Thruster [![dependency status](https://deps.rs/repo/github/emiflake/thruster/status.svg)](https://deps.rs/repo/github/emiflake/thruster) [![build status](https://travis-ci.com/emiflake/thruster.svg?branch=master)](https://travis-ci.org/emiflake/thruster) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![Language](https://img.shields.io/github/languages/top/emiflake/thruster)
A Rust Raytracer

:warning: This is a work in progress project :warning:

Built on `image` crate with imgui UI.
# Setup
You must download and install OIDN, which you can get from [here](https://github.com/OpenImageDenoise/oidn/releases). Currently, the only way I have found to get it to work is to add the directory to both `OIDN_DIR` and `LD_LIBRARY_PATH`. Hopefully an alternative method will be found in the future.

To run the raytracer, simply run these two commands:
```
git clone https://github.com/emiflake/thruster
cd ./thruster
cargo run --release
```
For now, there is not much customizability.

# Screenshots
![img](https://github.com/emiflake/thruster/blob/master/screenshot.png)
