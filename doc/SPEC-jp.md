# input-source-manager の仕様

## 1. 概要

このドキュメントは、`input-source-manager` Rustライブラリおよびそれに付随するコマンドラインインターフェース (CLI) ツール `macism-rust` の仕様を定義します。このプロジェクトは、macOSの入力ソースをRustアプリケーションから管理するための堅牢で信頼性の高い方法を提供することを目的としており、コア機能にはネイティブのSwift APIを活用しています。

## 2. CLI 仕様 (`macism-rust`)

`macism-rust` コマンドラインツールは以下の機能を提供します。

- **コマンド名:** `macism-rust`

- **デフォルトの動作 (引数なし): 現在の入力ソースIDの取得**
  ```bash
  macism-rust
  ```
  現在アクティブな入力ソースのIDを標準出力に出力します。

- **バージョン情報:**
  ```bash
  macism-rust --version
  # または
  macism-rust -v
  ```
  `macism-rust` ツールのバージョンを出力します。

- **利用可能な入力ソースIDのリスト表示:**
  ```bash
  macism-rust --list
  # または
  macism-rust -l
  ```
  システムで利用可能なすべての入力ソースIDを、1行に1つずつ標準出力に出力します。

- **明示的な現在の入力ソースIDの取得:**
  ```bash
  macism-rust get
  ```
  現在アクティブな入力ソースのIDを標準出力に出力します。これは機能的にはデフォルトの動作と同じですが、明示的なサブコマンドを提供します。

- **入力ソースの設定:**
  ```bash
  macism-rust set <INPUT_SOURCE_ID>
  ```
  `<INPUT_SOURCE_ID>` で指定された入力ソースにアクティブな入力ソースを切り替えようとします。成功した場合は、新しくアクティブになった入力ソースのIDを出力します。指定されたIDが見つからない場合や切り替えに失敗した場合は、エラーメッセージが標準エラーに出力されます。

## 3. ライブラリAPI 仕様 (`input-source-manager`)

`input-source-manager` Rustライブラリは以下の公開APIを提供します。

### 3.1. Enum

#### `pub enum InputSourceError`
入力ソースマネージャーとのやり取り中に発生する可能性のあるエラーを表します。

- `SwiftError(i32)`: 基盤となるSwiftコード内でエラーが発生しました。エラーコードが含まれます。
- `SourceNotFound`: 要求された入力ソースIDが利用可能なソースの中に見つかりませんでした。
- `SwitchFailed`: 入力ソースの切り替えに失敗しました。
- `InternalError`: 文字列変換の失敗など、内部エラーが発生しました。

#### `pub enum SwitchResult`
入力ソース切り替え操作の結果を表します。

- `Switched`: 入力ソースが正常に切り替わりました。
- `NotSwitched`: 入力ソースが切り替わりませんでした（例: すでにターゲットのソースだった場合）。

### 3.2. 関数

#### `pub fn initialize()`
基盤となるSwiftのInputSourceManagerを初期化します。この関数は、このライブラリの他の関数を使用する前に一度だけ呼び出す必要があります。利用可能な入力ソースの内部リストを設定します。

#### `pub fn get_current_input_source_id() -> Result<String, InputSourceError>`
現在アクティブな入力ソースのIDを取得します。

成功した場合は現在の入力ソースのIDを含む `Ok(String)` を返します。操作が失敗した場合は `InputSourceError` を返します。

#### `pub fn switch_input_source(sources: &[String]) -> Result<(SwitchResult, String), InputSourceError>`
提供された入力ソースIDのリストに基づいて入力ソースを切り替えます。

この関数は、現在の入力ソースに基づいてリスト内の次の入力ソースを決定します。現在のソースがリストにない場合は、リストの最初のソースに切り替えます。現在のソースがリストの最後にある場合は、最初のソースに戻って巡回します。

切り替えが発生したかどうか、および新しいソースIDを含む `Ok((SwitchResult, String))` を返します。操作が失敗した場合は `InputSourceError` を返します。

#### `pub fn get_available_ids() -> Result<Vec<String>, InputSourceError>`
利用可能なすべての入力ソースIDのリストを返します。

成功した場合は入力ソースIDのリストを含む `Ok(Vec<String>)` を返します。操作が失敗した場合は `InputSourceError` を返します。

#### `pub fn set_input_source(id: &str) -> Result<String, InputSourceError>`
入力ソースを指定されたIDに直接設定します。

この関数は、提供された正確なIDに入力ソースを切り替えようとします。ソースのリストを巡回することはありません。

成功した場合は新しいアクティブなソースIDを含む `Ok(String)` を返します。操作が失敗した場合は `InputSourceError` を返します（例: ソースが見つからない場合）。

## 4. アーキテクチャ

このライブラリは、macOSの入力ソースサービスと連携するためにハイブリッドなアーキテクチャを採用しています。

1.  **Swiftブリッジ:** macOSネイティブのText Input Source Services (TIS) APIと連携するためのコアロジックはSwiftで実装されています。これには、現在の入力ソースの取得、新しいソースの選択、利用可能なソースのリスト表示などの機能が含まれます。
2.  **Rust FFI:** Rustは、C互換のForeign Function Interface (FFI) を介してSwiftコードと通信します。Swiftソースファイル (`InputSourceManager.swift`、`rust_bridge.swift`) はスタティックライブラリ (`.a` ファイル) にコンパイルされます。
3.  **`build.rs`:** Rustで書かれたカスタムビルドスクリプト (`build.rs`) は、SwiftファイルのコンパイルとメインのRustプロジェクトへのリンクを調整します。このスクリプトは、`swiftc` が正しく呼び出され、オブジェクトファイルがアーカイブされ、結果として生成されるスタティックライブラリが、必要なSwiftランタイムライブラリやmacOSフレームワーク（`ApplicationServices`、`CoreFoundation`、`AppKit`）とともにRust実行ファイルに適切にリンクされることを保証します。

## 5. ライセンス

MIT License