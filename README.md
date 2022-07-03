# jmyounghoyos Blog

This project aims to create my blog using the @zzhack-stack Yew implementation.

## How to run it

```shell
# First we add the target that we want (in this case WASM).
rustup target add wasm32-unknown-unknown
# Then we install trunk to be able to compile the project.
cargo install trunk
cargo install --locked trunk
cargo install wasm-bindgen-cli
# Now let's build the app.
cd app 
trunk build --release -d ../dist
```

To get a fast look, let's create a simple `Python` server in the `dist` folder:
```shell
python3 -m http.server
```


## Acknowledge

Is used the [zzhack project](https://github.com/zzhack-stack/zzhack) as a base for this blog.
