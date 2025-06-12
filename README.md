# Image Resizer Action

This repository provides a simple GitHub Action written in Rust for resizing images to multiple widths while preserving aspect ratio.

## Usage

```yaml
- uses: ./
  with:
    source-dir: "assets/original"
    sizes:      "64,128,256"
    output-dir: "resized"
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
          source-dir: "assets/original"
          sizes:      "64,128,256"
          output-dir: "icons"
```

## Notes
- The action compiles the Rust binary inside a Docker container.
- Build artifacts are automatically cached by GitHub Actions.
- Consider using Rust caching strategies to speed up frequent runs.

## Developing Locally

You can test and iterate on this action without pushing to GitHub by using [act](https://github.com/nektos/act) or by running the Rust binary directly.

### Using act

1. Install act (via Homebrew on macOS):
   ```bash
   brew install act
   ```
2. Pre-pull the Rust image to avoid re-downloads:
   ```bash
   docker pull rust:latest
   ```
3. Run the workflow locally:
   ```bash
   act -j build --container-architecture linux/amd64 --pull if-not-present
   ```

### Direct Cargo Invocation

Build, test, and run the binary:

```bash
# Run tests
cargo test

# Build release and resize images
cargo build --release
./target/release/image-resizer \
  --source-dir assets/original \
  --sizes 64,128,256 \
  --output-dir out
```


# High-Level Architecture of the Image Resizer Action

This project is a self-contained GitHub Action, written in Rust, that scans a folder of images, resizes each one to a set of target widths (preserving aspect ratio), and writes the results back into your repo. Here’s how the pieces fit together:

1. `src/main.rs`  
   • Parses three inputs (`source-dir`, `sizes`, `output-dir`) via `clap`.  
   • Walks the source directory, checks for already-resized files, and for each missing size uses the `image` crate to read, resize, and save a new PNG.

2. `Dockerfile`  
   • **Builder stage**: pulls `rust:latest`, compiles your binary in `/workspace`.  
   • **Runtime stage**: uses a slim Debian base, copies in the compiled `image-resizer` binary, and sets it as the `ENTRYPOINT`.  
   • This multi-stage build produces a tiny container that contains only your executable and its glibc dependencies.

3. `action.yml`  
   • Declares the Action’s metadata (name, author, description).  
   • Defines three required inputs, each mapping to a CLI flag on your binary.  
   • Tells GitHub to build your Dockerfile and invoke your binary with `--source-dir=…`, `--sizes=…`, and `--output-dir=…`.

4. `.github/workflows/ci.yml`  
   • On every push or pull request, spins up a Rust container, builds the release binary, then invokes your Action against `assets/original`.  
   • Shows the resized outputs, configures Git to commit & push any new files back to your repo (using the Actions token).

5. `README.md`  
   • Documents how to consume the Action in another workflow.  
   • Explains local development:  
     – **With `act`** to run your workflow in Docker.  
     – **With `cargo run`** to invoke the binary directly on your machine.

6. Local Development & Testing  
   • **Direct**: `cargo build --release && ./target/release/image-resizer --source-dir assets/original --sizes 64,128,256 --output-dir assets/resized`  
   • **Using act**: pre-pull `rust:latest` and run `act -j build --container-architecture linux/amd64 --pull if-not-present` to exercise your CI job locally.

By combining a Rust CLI, a multi-stage Docker build, an `action.yml` manifest and a CI workflow that can commit results back into your repo, you get a reusable, high-performance image-resizing Action that can be plugged into any front-end project’s GitHub pipeline.```# High-Level Architecture of the Image Resizer Action

This project is a self-contained GitHub Action, written in Rust, that scans a folder of images, resizes each one to a set of target widths (preserving aspect ratio), and writes the results back into your repo. Here’s how the pieces fit together:

1. `src/main.rs`  
   • Parses three inputs (`source-dir`, `sizes`, `output-dir`) via `clap`.  
   • Walks the source directory, checks for already-resized files, and for each missing size uses the `image` crate to read, resize, and save a new PNG.

2. `Dockerfile`  
   • **Builder stage**: pulls `rust:latest`, compiles your binary in `/workspace`.  
   • **Runtime stage**: uses a slim Debian base, copies in the compiled `image-resizer` binary, and sets it as the `ENTRYPOINT`.  
   • This multi-stage build produces a tiny container that contains only your executable and its glibc dependencies.

3. `action.yml`  
   • Declares the Action’s metadata (name, author, description).  
   • Defines three required inputs, each mapping to a CLI flag on your binary.  
   • Tells GitHub to build your Dockerfile and invoke your binary with `--source-dir=…`, `--sizes=…`, and `--output-dir=…`.

4. `.github/workflows/ci.yml`  
   • On every push or pull request, spins up a Rust container, builds the release binary, then invokes your Action against `assets/original`.  
   • Shows the resized outputs, configures Git to commit & push any new files back to your repo (using the Actions token).

5. `README.md`  
   • Documents how to consume the Action in another workflow.  
   • Explains local development:  
     – **With `act`** to run your workflow in Docker.  
     – **With `cargo run`** to invoke the binary directly on your machine.

6. Local Development & Testing  
   • **Direct**: `cargo build --release && ./target/release/image-resizer --source-dir assets/original --sizes 64,128,256 --output-dir assets/resized`  
   • **Using act**: pre-pull `rust:latest` and run `act -j build --container-architecture linux/amd64 --pull if-not-present` to exercise your CI job locally.

By combining a Rust CLI, a multi-stage Docker build, an `action.yml` manifest and a CI workflow that can commit results back into your repo, you get a reusable, high-performance image-resizing Action that can be plugged into any front-end project’s GitHub pipeline.```
