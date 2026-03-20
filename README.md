* GitHub Push Workflow *
1. git init (初回のみ)
2. git add .
3. git commit -m "message"
4. gh repo create [name] --source=. --public --push (初回のみ)
   (2回目以降は git push)

* If SSH Error *
eval $(ssh-agent) && ssh-add ~/.ssh/id_ed25519


​🛠️ おすすめのワークフロー（Rust学習用）
​新しい練習（例えば「借用」）を始める時は、以下の流れが最短です。
​ディレクトリ作成: mkdir borrowing && cd borrowing
​プロジェクト初期化: cargo init
​最初の保存: git add . && git commit -m "init"
​GitHubへ公開: gh repo create rust-borrowing --source=. --public --push
