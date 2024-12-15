![Preview](./cargo/preview.png)

# text-wallpaper-generator-4k 
Rust script to generate 4k resolution wallpapers with text on a solid color background.
* Text into wallpaper
  * The text list is configurable in settings.toml
* Randomized solid color background
  * The color range is configurable in settings.toml
* Randomized font
  * Fonts are placed under the fonts folder
  * Free to add/remove
* Any size
  * Wallpaper size is configurable in settings.toml
* Generate multiple wallpapers with each text
  * The number is configurable in the settings.toml

# How to use
Win users can use pre-built exe
1. download [text-wallpaper-generator-4k_0.1.0_WIN.zip](https://github.com/hiroa-inami/text-wallpaper-generator-4k/releases/download/0.1.0/text-wallpaper-generator-4k_0.1.0_WIN.zip
) and extract it
2. edit settings.toml with notepad (Optional)
3. add your favorite font to fonts folder (Optional)
4. run exe


# How to build
Install [Rust](https://www.rust-lang.org/tools/install)  
clone repo and 
```
cargo run
```
will run script 

```
cargo build --release
```
will generate executable 
# How it works
[code](https://github.com/hiroa-inami/text-wallpaper-generator-4k/blob/main/src/main.rs) is a single file with 140 lines  
Read comments, which start with //

# about LICENSE
each font has its license  
everything else is MIT  

# Thank you
- [Google Fonts](https://fonts.google.com/) for awesome platform
- Font creator on Google Fonts for beautiful fonts
- [Github](https://github.com/) for generous infrastructure
- [Rust Foundation](https://foundation.rust-lang.org/) for creating Rust
