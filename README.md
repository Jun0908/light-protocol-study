# zk-Compression Sample

### ▶︎Overview
ZK圧縮(zk-Compression)の学習サイトです。ZK圧縮(zk-Compression)はライトプロトコル（Light Protocol）が、データインフラを提供するヘリウス（Helius）と連携し、ソラナ（Solana）上で圧縮されたデータを簡単に利用可能にするインフラです。

**約5000倍近くのコストの削減できる**

ZK圧縮(zk-Compression)はState Compressionなしの場合100万ユーザーに仮想通貨をエアドロップする時にかかる
260,000ドルが、ZK圧縮だと5,200分の1と安くなり、50ドルほどまで減らせる。

![画像1](https://github.com/user-attachments/assets/6bd315a0-bb08-4566-a032-b61506ed6a0c)

**マークルツリーを使った圧縮技術**

https://chaldene.net/markle-tree

https://docs.lightprotocol.com/learn/core-concepts/state-trees

![画像2](https://github.com/user-attachments/assets/1553be8b-6df5-4d43-a947-8450d1ecdbb3)

### ▶︎Demo Movie

https://youtu.be/bXMbdqoG3sE

### ▶︎How to Use Light Protocol
このプロジェクトでは、ブラウザ環境でZK Compression APIと対話するために`@lightprotocol/stateless.js`を使用する方法を示します。

以下の手順で環境設定をします。
Node >= v20.9.0に設定をします。

Unchainの環境設定の方法を見ながらSolanaの環境設定を行ってください

https://github.com/unchain-tech/UNCHAIN-projects/blob/main/docs/Solana-dApp/ja/section-2/lesson-1_Solana%20%E3%81%AE%E9%96%8B%E7%99%BA%E7%92%B0%E5%A2%83%E3%82%92%E6%A7%8B%E7%AF%89%E3%81%97%E3%82%88%E3%81%86%EF%BC%81.md

```bash
npm install -g @lightprotocol/zk-compression-cli
git clone git@github.com:Lightprotocol/light-protocol.git
```

. ./scripts/devenv.shは点を二つ付けることが重要です。これをしないとエラーがでます。
build.shまでの実行で30分近くかかります。


```bash
cd examples/browser/nextjs
. ./scripts/devenv.sh &&
./scripts/install.sh &&
./scripts/build.sh
```

Start a light test-validator using the CLI
```bash
cd cli &&
light test-validator
Start the app
cd ../examples/browser/nextjs &&
pnpm dev
```

This will serve and mount the app at http://localhost:1234 and run the code defined in page.tsx.

### ▶︎Reference

https://www.zkcompression.com/

https://docs.lightprotocol.com/

https://github.com/Lightprotocol/light-protocol/tree/main/cli

https://www.helius.dev/
