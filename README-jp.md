# input-source-manager

macOSの入力ソースを管理するためのRustライブラリおよびコマンドラインツール (`macism-rust`) です。このプロジェクトは、Swiftを活用してmacOSのText Input Source Services (TIS) APIと連携し、Rustから入力ソースの取得、設定、一覧表示を行うための堅牢なソリューションを提供します。

## 機能

- **現在の入力ソースの取得:** 現在アクティブな入力ソースのIDを取得します。
- **入力ソースの設定:** IDを指定して、特定の入力ソースに直接切り替えます。
- **入力ソースの巡回:** 事前に定義された入力ソースのリストを巡回して切り替えます。
- **利用可能な入力ソースの一覧表示:** システムで利用可能なすべての入力ソースIDのリストを取得します。
- **`macism-rust` CLI:** オリジナルの `macism` ツールと同様の機能に加え、追加機能を備えたコマンドラインインターフェースツールです。

## `macism-rust` CLI の使用方法

`macism-rust` コマンドラインツールは以下の機能を提供します。

- **現在の入力ソースIDを取得:**
  ```bash
  macism-rust
  ```

- **入力ソースを特定のIDに設定:**
  ```bash
  macism-rust jp.sourceforge.inputmethod.aquaskk.Hiragana
  ```

- **キーボード入力ソースIDを一覧表示:**
  ```bash
  macism-rust -l
  # または
  macism-rust --list
  ```

- **パレット入力ソースIDを一覧表示:**
  ```bash
  macism-rust -p
  # または
  macism-rust --palette
  ```

- **すべての入力ソースIDを一覧表示:**
  ```bash
  macism-rust -l -p
  ```

- **バージョン情報を表示:**
  ```bash
  macism-rust --version
  ```

## ライブラリの使用方法

`Cargo.toml` に `input-source-manager` を追加します。

```toml
[dependencies]
input-source-manager = "0.1.0"
```

使用例:

```rust
use input_source_manager::{self, get_current_input_source_id, set_input_source, get_available_ids, InputSourceCategory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    self::initialize(); // 最初に一度呼び出す必要があります

    let current_id = get_current_input_source_id()?;
    println!("現在の入力ソース: {}", current_id);

    let available_ids = get_available_ids(InputSourceCategory::Keyboard)?;
    println!("利用可能なキーボード入力ソース: {:?}", available_ids);

    // 例: 特定の入力ソースに設定
    if let Some(target_id) = available_ids.first() {
        println!("入力ソースを {} に設定します", target_id);
        set_input_source(target_id)?; // 注意: target_id がシステム上の有効な入力ソースである場合にのみ機能します
        println!("新しい現在の入力ソース: {}", get_current_input_source_id()?);
    }

    Ok(())
}
```

## ビルド方法

このプロジェクトは、ビルドプロセス中にSwiftソースファイルをコンパイルするため、`PATH` に `swiftc` が利用可能である必要があります。XcodeまたはXcodeのCommand Line Toolsがインストールされていることを確認してください。

プロジェクトをビルドするには:

```bash
cargo build --release
```

`macism-rust` 実行ファイルは `target/release/` に生成されます。

## 仕組み

このライブラリはハイブリッドなアプローチを採用しています。
1.  **Swiftブリッジ:** コアとなる入力ソース管理ロジックはSwiftで実装されており、macOSネイティブのText Input Source Services (TIS) APIを活用しています。
2.  **Rust FFI:** Rustは、C互換のForeign Function Interface (FFI) を介してSwiftコードと通信します。Swiftコードはスタティックライブラリ (`.a` ファイル) にコンパイルされ、Rustプロジェクトにリンクされます。
3.  **`build.rs`:** カスタムビルドスクリプト (`build.rs`) がSwiftファイルのコンパイルとRustプロジェクトへのリンクを処理し、シームレスなビルド体験を提供します。

## ライセンス

MIT License

## 謝辞

このプロジェクトは、[laishulu](https://github.com/laishulu) 氏の `macism` プロジェクトに強く触発され、その成果を基に構築されています。macOSの入力ソース管理における彼らの基礎的な仕事に心からの感謝を表します。