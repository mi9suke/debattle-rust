# Debattlers - Tauri Desktop Edition

Python版 `debattle-poc` を、モダンな2026年スタイルのデスクトップアプリとして再構築したプロジェクトです。

## 技術スタック
- **Backend**: Rust 1.75+, Tauri 1.5
- **Frontend**: Svelte 5 (Runes), TypeScript, Vite 6
- **Styling**: Tailwind CSS v4 (Glassmorphism, Modern Gradients)
- **AI**: Google Gemini API (gemini-2.0-flash)

## 事前準備
WSL (Ubuntu等) にディレクトリを移行した後、以下のツールをインストールしてください：

1. **Rust & Cargo**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **Node.js**: `nvm install 20`
3. **OS依存関係 (Ubuntu/Debian)**:
   ```bash
   sudo apt-get update
   sudo apt-get install -y libgtk-3-dev libsoup2.4-dev libwebkit2gtk-4.0-dev libappindicator3-dev left-pad librsvg2-dev patchelf
   ```

## セットアップ
1. プロジェクトルートで依存関係をインストール：
   ```bash
   npm install
   ```
2. `src-tauri` ディレクトリに `.env` ファイルを作成し、APIキーを設定：
   ```env
   GEMINI_API_KEY=your_gemini_api_key_here
   ```

## 開発とビルド
- **開発モード**:
  ```bash
  npm run dev
  ```
- **ビルド**:
  ```bash
  npm run build
  ```

## アーキテクチャ
- `src-tauri/src/models.rs`: ゲームのデータ構造（Rust 構造体）
- `src-tauri/src/game_logic.rs`: Python版から移植された討論ゲームの核となるロジック
- `src-tauri/src/ai_client.rs`: Gemini API クライアント（スタブ実装済み）
- `src/App.svelte`: Svelte 5 を使用したリアクティブなUI
- `src/app.css`: Tailwind v4 によるモダンなデザインシステム
