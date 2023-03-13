# Poke IP Go

A simple app to demonstrate Rust's multi-platform capabilities. Currently one can "catch" Pokemon by getting one based on current public facing IP address, or rolling completely random one, there's also storage support, so all your Pocket Monsters are safe. Project's main goal is to show off wide range of functionality (GUI, RNG, network requests, persistent storage) and how it can be achieved on different platforms.

## Running

We're using helper tools to run our app on different platforms, since it makes our life easier. 

### Desktop

For Windows/Linux (and probably OSX), once can just use `cargo` or `rust-mobile/xbuild` tool.

```sh 
 $ cargo run
```
```sh 
 $ x run
```
 
### Mobile

### Web

```sh 
 $ trunk serve
```

