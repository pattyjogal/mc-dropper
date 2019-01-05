# Dropper
### A Minecraft Server Package Manager

<p align="center">
  <img src="https://d1u5p3l4wpay3k.cloudfront.net/minecraft_gamepedia/8/8d/Dropper.png?version=545724d0fc82921721863cd746a8c341"/>
</p>

This project came from a desire to simplify running a Minecraft server. It aims
to emulate the package management experience similar to what one would experience
when writing programs. Where does the need for this package come from? Beginner users will
have an easier time setting up plugins, and can even copy config files from established
servers online, or from their friends, to get started quickly and tweak from there.
More experienced users can use it to simplify updates/maintenance, replicate servers on
other machines, and provide a non-bulky way of sharing their server config with the world.

What are some of the drawbacks of this project? The main one right now is that no
simplified package database exists for Bukkit and the like. Bukkit's website has no
API to speak of, so this program contains a 95% accurate parser, but it's at the mercy
of package maintainers for the most part. Spigot has quite the library, but it's behind
an annoying Cloudflare wall. The goal of this project is to be as plug-and-play as possible
to support different package repositories, either through HTML scraping or JSON API.

The project is written in the Rust language. This is to minimize runtime errors and
easily compile to Linux, Windows, and Mac targets. If you wish to page through the
source, you will want to get familiar with Rust somewhere: I suggest the [Rust Book](https://doc.rust-lang.org/stable/book/).

## Installation
At time of writing, this crate has not been uploaded to Cargo, nor have I published
binaries. This is simply because it's not ready yet.

For contributors, you are fully free to fork & clone the repository and make pull
requests to it. Make sure you have `cargo` installed, and once you download, you
should be able to run

``` bash
cargo install && cargo check
```

to verify that your clone worked. From there, you can use

``` bash
cargo run
```

to run the program. If you are being super awesome and helping with the CLI, you may
want to pass arguments to the program. You can do so by adding `--` after `cargo run`,
and then your arguments. For instance, if I wanted to run my program with the arguments
`add WorldEdit@6.1.9`, I would run:

``` bash
cargo run -- add WorldEdit@6.1.9
```

## Project Documentation
This project comes with documentation! Hooray! Rust provides an easy to use
[documentation spec](https://doc.rust-lang.org/rust-by-example/meta/doc.html)
that even supports Markdown. I not only encourage you to use it, but it is a requirement
to complete a pull request to this project. If you wish to view the documentation, you
can run

``` bash
cargo doc
```

which will dump the files to a `target/doc/dropper` folder. Open up the `index.html` file
there to view the docs in the browser.

## Project Wishlist
The big thing that this project needs is a command line interface for users to work with.
Ideally, this would be super simple and the documentation behind it would be ultra
understandable, since there are a sizeable amount of younger MC server enthusiasts who are
not as technically experienced.
