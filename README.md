
**「1つの projects リポジトリに複数の学習プロジェクト（reference等）をまとめて管理し、別端末でも同期して開発を続ける」**ための黄金ルーチン

​🛠️ フェーズ1：最初のセットアップ（端末A）
​まずは親となる projects を作り、その中に最初の reference を追加します。

​1. 親リポジトリの作成と初期化

mkdir ~/projects && cd ~/projects
git init
### GitHubに親リポジトリを作成（すでにある場合は飛ばしてOK）
gh repo create projects --public --source=. --remote=origin

2. プロジェクト（reference）の作成

cargo new reference  
### referenceフォルダとソースが自動生成される
cd reference
### (ここでRustのコードを書く)

3. 必要なファイルだけを保存してPush

​.gitignore が自動生成されているので、ビルド済みの巨大なバイナリなどは除外され、ソースコードのみが対象になります。

cd ~/projects
git add reference/
git commit -m "Add reference project"
git push -u origin main

🔄 フェーズ2：別端末での再開（端末B）
​新しい端末（別のPCやスマホ）で、これまでの作業を引き継ぎます。

​1. まるごとコピー（Clone）する

git clone https://github.com/katanotaka/projects.git
cd projects/reference
### (編集を再開する)

🔁 フェーズ3：日々の開発ルーチン（共通）
​「編集 → 保存 → 同期」の繰り返し手順です。

​1. 作業を始める前（最新状態を取り込む）
​他の端末で書いた内容を忘れないよう、必ず最初に実行します。

cd ~/projects
git pull origin main --rebase

2. コードを書いて保存（Commit）

### (コードを編集・保存)
git add .
git commit -m "Update reference: added mutable borrowing"

3. GitHubへ送り出す（Push）

git push origin main

💡 失敗しないための「3つの鉄則」

​.git は親（projects）にだけ置く
​各子フォルダ（reference等）の中で git init1 は絶対しないこと。管理が複雑になります。もし間違えて作ったら rm -rf .git で消せばOKです。

​作業前は必ず pull
​これを忘れて編集すると、あとで「競合（Conflict）」が起きて面倒になります。

​不要なファイルは add しない
​target/ フォルダ（ビルド生成物）は重いので、git add . する際も Cargo.toml がある階層の .gitignore がしっかり効いているか確認しましょう。

##​これで「プロジェクト管理の基盤」は完璧です！





