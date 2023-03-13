# Poke IP Go

A simple app to demonstrate Rust's multi-platform capabilities. Currently one can "catch" Pokemon by getting one based on current public facing IP address, or rolling completely random one, there's also storage support, so all your Pocket Monsters are safe. Project's main goal is to show off wide range of functionality (GUI, RNG, network requests, persistent storage) and how it can be achieved on different platforms.

## Running

We're using helper tools to run our app on different platforms, since it makes our life easier. 

### Desktop

For Windows/Linux (and probably OSX), once can just use `cargo` or [`rust-mobile/xbuild`](https://github.com/rust-mobile/xbuild) tool.

```sh 
 $ cargo run
```
```sh 
 $ x run
```
 
### Mobile
Main reason for using `xbuild` is so that we don't have to deal with all the project setup and let tool handle that for us. To run, select your device from `x devices`.

```sh 
 $ x run --device $your_android_device
````

### Web

```sh 
 $ trunk serve
```

