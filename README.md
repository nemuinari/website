# Create homepage (Rust)

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
name: Deploy to AWS S3

on:
  push:
    branches:
      - main # mainブランチにpushされた時に発動

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install Trunk
        run: |
          wget -qO- https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
          sudo mv trunk /usr/local/bin/

      - name: Build with Trunk
        run: trunk build --release

      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v4
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: ap-northeast-1 # お使いのリージョン

      - name: Deploy to S3
        run: |
          aws s3 sync dist/ s3://${{ secrets.S3_BUCKET_NAME }} --delete

      - name: CloudFront Invalidation (Optional)
        run: |
          aws cloudfront create-invalidation --distribution-id ${{ secrets.CLOUDFRONT_DISTRIBUTION_ID }} --paths "/*"
```

## setp.5

#### GitHub への Secret 登録

GitHub リポジトリの Settings > Secrets and variables > Actions から、以下の4つを登録する必要があります。

AWS_ACCESS_KEY_ID: AWSのアクセスキー
AWS_SECRET_ACCESS_KEY: AWSのシークレットキー
S3_BUCKET_NAME: 作成したS3バケットの名前
CLOUDFRONT_DISTRIBUTION_ID: CloudFrontのディストリビューションID（まだなら後回しでもOK）

[!IMPORTANT]
AWS IAM の権限設定: > 使用するIAMユーザーには AmazonS3FullAccess と CloudFrontFullAccess（またはそれらに絞った権限）が必要です。

#### AWS 側の事前設定チェック

S3バケットを作成する際、以下の設定を済ませておいてください。

静的ウェブサイトホスティングを有効にする。

パブリックアクセスブロックを解除（CloudFrontを使わない場合）するか、OAC (Origin Access Control) を設定して CloudFront からのみ許可する（推奨）。

インデックスドキュメントを index.html に設定する。
