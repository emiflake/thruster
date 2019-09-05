# Thruster
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
