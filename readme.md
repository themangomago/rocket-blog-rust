# Rocket Blog
## A blog using Rust and Rocket.rs

This is a simple blog written in Rust and using the [Rocket.rs](https://rocket.rs) framework for educational purpose only. It is not meant to be used in production.
It's using a file-based database for storing the blog posts and the users. This might be a potential security risk.

## Running the blog
Rocket.rs makes use of unstable features, therefore you will need to enable the nightly toolchain for this project.
```bash
rustup override set nightly
```
```bash
cargo run
```


[](https://rocket.rs/v0.4/guide/requests/)


## TODO

- [x] Add user
- [ ] Delete user
- [x] Change password
- [x] Profile
- [x] Edit post
- [x] Pagination
- [ ] Clean up and optimize :)

Links:

https://bulma.io/documentation/overview/colors/

https://github.com/SergioBenitez/Rocket/tree/v0.4/examples

