# img-search
CLI to help me source property/characters in images.
Makes use of [RustNAO](https://github.com/ClementTsang/RustNAO) to source images.

Basically, this thing downloads the thumbnails for matching images to the format
"(property): (characters)_[0-9]+".

It then opens all these downloaded images in feh, allowing you to check if any of the downloaded images
matches your input image.

After feh closes, the program removes any temp files automatically.

# Prerequisites
 * [Rust](https://www.rust-lang.org/)
 * [feh image viewer](https://feh.finalrewind.org/)

# Installation
You know the drill:
```shell
git clone https://github.com/djairoh/img-search
cd ./img-search
cargo build --release
mv ./target/release/
./img_search <FILE> [DIR]
```

see `img_search --help` for more info.

# Config
Configuration is handled by Confy, 
but you can probably find the config file at '~/.config/img-search/default_config.toml'

The only thing you need to change is the 'api_key' field:
supply your own SauceNAO api key here.

The other available fields are:
 * num_results: how many max results to return for each query 
 * min_similarity: what the minimum similarity is for each returned result
 * rust_log: what level of logging to write to stdout

# Documentation
Standard Rustdoc documentation effort.
Compile with `cargo doc --open`.


