# Image Resizer Action

This repository provides a simple GitHub Action written in Rust for resizing images to multiple square sizes. The action reads an image from your repository, resizes it to the requested widths and either crops or pads the image to a square.

## Usage

```yaml
- uses: ./
  with:
    image-path: "assets/logo.png"
    sizes: "64,128,256"
    output-dir: "resized"
    crop: true
```

## Example Workflow

```yaml
name: Build Icons
on: [push]

jobs:
  resize:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: ./
        with:
          image-path: "assets/logo.png"
          sizes: "64,128,256"
          output-dir: "icons"
          crop: false
```

## Notes
- The action compiles the Rust binary inside a Docker container and caches build artifacts automatically by GitHub Actions.
- If you run the action frequently, consider using Rust caching strategies to speed up builds.
