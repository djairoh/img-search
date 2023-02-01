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

# Documentation
Standard Rustdoc documentation effort.
Compile with `cargo doc --open`.


