![pidif](https://user-images.githubusercontent.com/55081/179420332-6b64ab77-3de0-42e3-816a-ed640d3fca5e.png)

# Pidif PDF Viewer

A light PDF Viewer, using Poppler, GTK 4 and Rust. Pidif was built especially for touch-only devices (phones, e-readers and tablet), where screen real estate is scarce, but can be used on desktop too.

## Usage

To maintain a clean UI, Pidif comes only with an "open file" button. Navigating between the pages of a document is done by clicking its left and right halfs. Toggling the bottom and header bar ("fullscreen mode") is done by clicking anywhere in the top 20% of the document.

## Installation

### Compiling manually

- Install the dev packages of libgtk-4, libcairo, and libpoppler.
- `git clone` the repository
- Run `cargo build --release`
