# Solana Mini Hackathon 2024

### ▶︎Overview
ZK圧縮(zk-Compression)はライトプロトコル（Light Protocol）が、データインフラを提供するヘリウス（Helius）と連携し、ソラナ（Solana）上で圧縮されたデータを簡単に利用可能にするインフラです。今回のハッカソンではLight Protocolを実装いたしました。

### ▶︎Problems
ブロックチェーンのデータ保存コストが高く、データの正確性とプライバシーを両立させる管理が難しい。また、大規模なユーザー導入にはアカウントスペース確保のための高額なコストが発生する。

### ▶︎Solution
ZK圧縮技術によりデータ保存コストを削減し、データの正確性とプライバシーを確保。さらに、ライトプロトコルとヘリウスの連携で技術理解不要な効率的なデータ管理が可能となり、アカウントスペース確保コストも大幅に削減できる。

### ▶︎How to Use Light Protocol
このプロジェクトでは、ブラウザ環境でZK Compression APIと対話するために`@lightprotocol/stateless.js`を使用する方法を示します。

### Building a monorepo

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

### ▶︎Demo Movie

https://youtu.be/bXMbdqoG3sE

### ▶︎Prospective
安価にデータをオンチェーンに保存できるデータ圧縮技術を用いて今まで保存が難しかった音楽や学習済みデータをオンチェーンに保存したい。

**①音楽のオンチェーンの保存技術　申請者が開発**

https://github.com/Jun0908/Chacha-GPT

**②学習済みデータのオンチェーンの保存技術**

現在開発中

### ▶︎Reference

https://www.zkcompression.com/

https://docs.lightprotocol.com/

https://github.com/Lightprotocol/light-protocol/tree/main/cli

https://www.helius.dev/
