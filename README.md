# Image Resizer Action

> A fast, Rust-powered GitHub Action to batch-resize images by width while preserving aspect ratio.

![CI Status](https://github.com/elarsaks/resize-rs/actions/workflows/ci.yml/badge.svg)

## 📦 Usage

In your workflow, reference this action and supply the folder of source images, desired widths, and an output folder:

## [📖 Example Workflow ](https://github.com/elarsaks/resize-rs/blob/master/.github/workflows/resize-images.yml)

```yaml
name: Resize App Images

on:
  push:
    branches: [master] # Trigger on pushes to master branch
  workflow_dispatch: # Allow manual runs from the Actions tab

# Grant write permission so the workflow can commit back to the repo
permissions:
  contents: write

jobs:
  resize:
    runs-on: ubuntu-latest

    steps:
      # 1. Checkout the repo so we can read/write files
      - name: Check out code
        uses: actions/checkout@v3
        with:
          token: ${{ secrets.GITHUB_TOKEN }} # Provides auth for checkout & push
          persist-credentials: true # Keep token in Git config for commits

      # 2. Configure a Git user so commits have a valid author
      - name: Configure Git user
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"

      # 3. Run the custom resize action
      - name: Resize images
        uses: elarsaks/resize-rs@v0.1.1
        with:
          source-dir: assets/original # Source directory of original images
          sizes: "64,128,256" # Target resize dimensions
          output-dir: assets/resized # Where resized images are saved

      # 4. Commit & push only if new/resized files exist
      - name: Commit & push resized files
        run: |
          git add assets/resized
          git diff --cached --quiet || (      # Check for staged changes
            git commit -m "ci: add resized images"
            git push                         # Push commits to the triggering branch
          )

```  

## 🔧 Inputs

| Input        | Description                                 | Required | Default |
|--------------|---------------------------------------------|:--------:|:-------:|
| `source-dir` | Directory containing original images        |   true   |    —    |
| `sizes`      | Comma-separated list of target widths (px)  |   true   |    —    |
| `output-dir` | Directory where resized images will be saved|   true   |    —    |

## 🛠️ Outputs

It writes resized files into `output-dir`.


## 📄 License

[MIT](LICENSE) © Elar Saks
