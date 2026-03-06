# Create homepage (Rust)

実装環境:

- Rust/WebAssembly/Yew/Stylist.
- GitHub Pages.

```t
https://nemuinari.github.io/website/
```

## step.1

#### Wasmターゲットを追加

```bash
$ rustup target add wasm32-unknown-unknown
```

#### ビルド兼サーバーツールをインストール

```bash
$ cargo install trunk
```

## step.2

#### フォルダ作成

```bash
$ cargo new my-homepage
$ cd my-homepage
```

#### 必要なライブラリ(Yew)を追加

```bash
$ cargo add yew --features csr
```

## step.3

#### index.html を作成

```html
<!DOCTYPE html>
<html lang="ja">
  <head>
    <meta charset="utf-8" />
    <title>My Rust Homepage</title>
  </head>
  <body></body>
</html>
```

#### src/main

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello, Rust Wasm!" }</h1>
            <p>{ "This is my page." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

#### ローカルで起動

```bash
$ trunk serve --open
```

## step.4

#### GitHub repository の準備

```bash
$ mkdir -p .github/workflows
$ touch .github/workflows/deploy.yml
```

#### .github/workflows/deploy.yml

```yaml
name: Deploy Yew App to GitHub Pages

on:
  push:
    branches:
      - main

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      # Rust ツールチェーンのセットアップ
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      # Trunk のインストール
      - name: Download and install Trunk
        run: |
          wget -qO- https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
          sudo mv trunk /usr/local/bin/

      # ビルド実行（リポジトリ名 "website" に合わせたパス設定）
      - name: Build with Trunk
        run: trunk build --release --public-url "/website/"

      # Yew-router を使っている場合の 404 対策（必要なければ削除可）
      - name: Copy index.html to 404.html
        run: cp dist/index.html dist/404.html

      - name: Setup Pages
        uses: actions/configure-pages@v4

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: "./dist"

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```
