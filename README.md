<p align="center"> <img alt="pidif screenshot" src="https://user-images.githubusercontent.com/55081/179860375-486574a8-cc29-4126-bd69-d304ea27acae.png" /></p>



# Pidif PDF Viewer

A light PDF Viewer, using Poppler, GTK 4 and Rust. Pidif was built especially for touch-only devices (phones, e-readers and tablet), where screen real estate is scarce, but can be used on desktop too.

## Usage

To maintain a clean UI, Pidif comes only with an "open file" button. Navigating between the pages of a document is done by clicking its left and right halfs. Toggling the bottom and header bar ("fullscreen mode") is done by clicking anywhere in the top 20% of the document.

## Installation

### Distribution packages

- Arch Linux: [pidif](https://aur.archlinux.org/packages/pidif) <sup>AUR</sup>
- Alpine Linux ( & PostmarketOS): [pidif](https://pkgs.alpinelinux.org/packages?name=pidif&branch=edge&repo=&arch=x86_64&maintainer=) <sup>testing</sup>


### Compiling manually

- Install the dev packages of `gtk-4`, `cairo`, and `poppler`.
- `git clone` the repository
- Run `cargo build --release`
