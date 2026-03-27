# N.N.Lab — Homepage (Rust/Wasm)

個人ホームページ。Rust + WebAssembly + Yew で構築し、GitHub Pages でホスティング。

🔗 https://nemuinari.github.io/website/

---

## 技術スタック

| 項目              | 内容                                                    |
| ----------------- | ------------------------------------------------------- |
| 言語              | Rust (edition 2024)                                     |
| UI フレームワーク | [Yew](https://yew.rs/) 0.21 (CSR)                       |
| スタイリング      | [Stylist](https://github.com/futursolo/stylist-rs) 0.13 |
| ビルドツール      | [Trunk](https://trunkrs.dev/)                           |
| ターゲット        | `wasm32-unknown-unknown`                                |
| ホスティング      | GitHub Pages                                            |
| CI/CD             | GitHub Actions                                          |

---

## プロジェクト構成

```
.
├── assets/                    # 静的ファイル（画像・JSON）
│   ├── header_1.png           # ヒーローエリア背景
│   ├── header_2.png           # ヒーローエリアタイトルロゴ
│   ├── header_3.png           # ヒーローエリアサブタイトル
│   ├── profile_1.png          # プロフィール背景画像
│   ├── profile_2.png          # プロフィール前面画像
│   ├── rtm-icon.png           # Works サムネイル
│   ├── articles.json          # CI-EN 記事一覧（CI で自動生成）
│   └── arts/                  # イラスト画像
├── scripts/
│   └── fetch_cien_articles.py # CI-EN 記事取得スクリプト
├── src/
│   ├── main.rs                # エントリーポイント・ルートコンポーネント
│   ├── components/            # UI コンポーネント
│   │   ├── mod.rs
│   │   ├── header.rs          # ナビゲーション + ヒーロービジュアル
│   │   ├── arts.rs            # イラスト一覧（モーダル表示付き）
│   │   ├── blog.rs            # ブログ記事一覧（CI-EN 連携）
│   │   ├── works.rs           # 制作物一覧
│   │   ├── works_data.rs      # Works データ定義
│   │   ├── profile.rs         # プロフィールセクション
│   │   ├── footer.rs          # フッター
│   │   └── test.rs            # 開発用プレースホルダー
│   └── css/
│       ├── mod.rs             # CSS 結合・StyleSource 生成
│       ├── variable.css       # CSS カスタムプロパティ（変数）
│       ├── base.css           # リセット・基本スタイル
│       ├── header.css         # ヘッダー・ヒーロービジュアル
│       ├── blog.css           # ブログセクション
│       ├── arts.css           # Arts セクション
│       ├── works.css          # Works セクション
│       ├── profile.css        # プロフィールセクション
│       ├── test.css           # 工事中表示
│       └── footer.css         # フッター
├── index.html
├── Cargo.toml
└── .github/workflows/deploy.yml
```

---

## コンポーネント一覧

### `Header`

ナビゲーションバーとヒーロービジュアルエリアを管理。

- 固定ナビゲーションバー（ロゴ + リンク6件）
- モバイル向けハンバーガーメニュー（`☰`）
- `IntersectionObserver` でビューポート検出 → CSS アニメーション発火
- ヒーロー画像は3レイヤー構成（背景 / タイトル / サブタイトル）で遅延 fade-in

### `Blog`

CI-EN の記事一覧をビルド時に生成した `assets/articles.json` から取得して表示。

- マウント時に `gloo-net` で JSON を fetch
- 初期表示 3件、"View More" で全件展開（アニメーション付き再描画）
- ローディング中はドットアニメーション、取得失敗時はエラーメッセージ表示
- モバイルでは 2カラム表示・3件目を非表示に（Show All 時は全表示）

### `Arts`

イラスト画像をグリッド表示。画像データは `arts.rs` 内の定数で静的定義。

- サムネイル用と拡大用で別画像を使用（モアレ対策）
- 初期表示6件、"View More" で全件展開
- 画像をクリック（タップ）するとモーダルで拡大表示
- モーダルはオーバーレイクリックまたは Esc キーで閉じる
- 画像は `max-width: 90vw / max-height: 90vh` に収まるようスケーリング
- `IntersectionObserver` で初回スクロール到達時にアニメーション発火

### `Works`

制作物をグリッド表示。データは `works_data.rs` で静的定義。

- 初期表示 3件、"View More" で全件展開
- カードホバーで画像拡大 + カード浮き上がりエフェクト
- 画像未設定時はプレースホルダー（丸）を表示
- `IntersectionObserver` で初回スクロール到達時にアニメーション発火

### `Profile`

プロフィール情報を左右レイアウトで表示。

- 左：2レイヤー画像（背景が右から、前面が左から slide-in）
- 右：ラベル + 値のエントリーリスト（順次 fade-in）
- モバイルでは横並びレイアウトを維持しつつサイズ調整

### `Footer`

著作権表示のみのシンプルなフッター。

---

## CSS 設計

すべての CSS は `src/css/mod.rs` の `get_app_style()` で `concat!` + `include_str!` により1つの `StyleSource` に結合し、Stylist 経由でスコープ付きクラスとして注入する。

```
variable.css → base.css → header.css → blog.css → arts.css
             → profile.css → works.css → modal.css → test.css → footer.css
```

### CSS カスタムプロパティ（`variable.css`）

```css
--header-height: 64px --primary-text: #333 --border-color: #ddd
  --max-width-content: 1100px --max-width-visual: 1500px
  --ease-slide: cubic-bezier(0.25, 0.45, 0.45, 0.95)
  --ease-standard: cubic-bezier(0.25, 0.46, 0.45, 0.94);
```

### アニメーション方針

各セクションは `visibility: hidden` で初期化し、`IntersectionObserver` が交差を検出した時点で `.animate` クラスを付与 → CSS アニメーション（`fadeInLefty` / `fadeInRighty`）を発火させる。カード類は `--i` CSS 変数で `animation-delay` を段階的にずらしてストリーク演出を実現。

---

## ローカル開発

### 1. 前提条件

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### 2. 開発サーバー起動

```bash
trunk serve --open
```

`http://localhost:8080` でホットリロード付きで起動する。

> **Note:** Blog セクションは `assets/articles.json` を参照する。ローカルで表示するには後述のスクリプトを先に実行するか、ダミーの JSON を手動で配置すること。

### 3. CI-EN 記事 JSON をローカル生成（任意）

```bash
python3 scripts/fetch_cien_articles.py
```

`assets/articles.json` が生成される。

### 4. リリースビルド

```bash
trunk build --release --public-url "/website/"
```

成果物は `dist/` に出力される。

---

## CI/CD・デプロイ

`.github/workflows/deploy.yml` により以下が自動実行される。

**トリガー:** `main` ブランチへの push、および毎日 00:00 UTC（スケジュール実行）

```
1. Checkout
2. Rust ツールチェーン + wasm32 ターゲットセットアップ
3. Trunk インストール
4. scripts/fetch_cien_articles.py 実行 → assets/articles.json 生成
5. trunk build --release --public-url "/website/"
6. dist/index.html を dist/404.html にコピー（SPA ルーティング対策）
7. GitHub Pages へデプロイ
```

スケジュール実行により、コードを変更しなくても CI-EN の新着記事が毎日自動反映される。

---

## Works へのデータ追加

`src/components/works_data.rs` の `get_works_data()` にエントリーを追加する。

```rust
WorkItem {
    id: 2,
    title: "Project Name".to_string(),
    url: "https://github.com/...".to_string(),
    img_path: "assets/project-icon.png".to_string(), // 空文字でプレースホルダー表示
},
```

---

## ライセンス

MIT © 2026 N.N.``
