# Solana Mini Hackathon 2024

### ▶︎Overview
ZK圧縮(zk-Compression)はライトプロトコル（Light Protocol）が、データインフラを提供するヘリウス（Helius）と連携し、ソラナ（Solana）上で圧縮されたデータを簡単に利用可能にするインフラです。今回のハッカソンではLight Protocolを実装いたしました。

ZK compression is an infrastructure provided by Light Protocol, in collaboration with Helius, which offers data infrastructure. It makes compressed data easily accessible on Solana. In this hackathon, we implemented Light Protocol.

**約5000倍近くのコストの削減できる**

ZK圧縮(zk-Compression)はState Compressionなしの場合100万ユーザーに仮想通貨をエアドロップする時にかかる
260,000ドルが、ZK圧縮だと5,200分の1と安くなり、50ドルほどまで減らせる。

**マークルツリーを使った圧縮技術**
https://chaldene.net/markle-tree
https://docs.lightprotocol.com/learn/core-concepts/state-trees


### ▶︎Problems
ブロックチェーンのデータ保存コストが高く、データの正確性とプライバシーを両立させる管理が難しい。
そのため、オフチェーンにデータを保存しているが下記の問題点がある

①IPFSに保存されたデータがピン留めされていない場合、データが削除される可能性がある。

②ブロックチェーン上にはIPFSのハッシュ値などのリファレンス情報が保存されますが、オフチェーンデータが変更されたり、消失した場合、リファレンス情報が無効になる可能性がある。

③アップグレード可能なコントラクトの場合、CIDを含むデータを変更できる可能性がある。

The cost of data storage on the blockchain is high, and it is challenging to manage both data accuracy and privacy. Additionally, large-scale user adoption incurs significant costs for securing account space.

### ▶︎Solution
ZK圧縮技術によりデータ保存コストを削減し、データの正確性とプライバシーを確保。さらに、ライトプロトコルとヘリウスの連携で技術理解不要な効率的なデータ管理が可能となり、アカウントスペース確保コストも大幅に削減できる。

By utilizing zk-Compression technology, data storage costs are reduced while ensuring data accuracy and privacy. Furthermore, the collaboration between Light Protocol and Helius enables efficient data management without the need for technical understanding, significantly reducing the costs associated with securing account space.

### ▶︎Demo Movie

https://youtu.be/bXMbdqoG3sE

### ▶︎Prospective
安価にデータをオンチェーンに保存できるデータ圧縮技術を用いて今まで保存が難しかった音楽や学習済みデータをオンチェーンに保存したい。

We want to use data compression technology that allows for cost-effective on-chain data storage, making it possible to store previously challenging data such as music and trained models on-chain.

**①音楽のオンチェーンの保存技術　申請者が開発/ The applicant developed technology for storing music on-chain**

https://github.com/Jun0908/Chacha-GPT

**②学習済みデータのオンチェーンの保存技術 現在開発中/ On-chain storage technology for trained models is currently under development**

https://github.com/Jun0908/Trained_Model_Converter

### ▶︎How to Use Light Protocol
このプロジェクトでは、ブラウザ環境でZK Compression APIと対話するために`@lightprotocol/stateless.js`を使用する方法を示します。

In this project, we demonstrate how to use @lightprotocol/stateless.js to interact with the ZK Compression API in a browser environment.

以下の手順で環境設定をします。
Node >= v20.9.0に設定をします。

Unchainの環境設定の方法を見ながらSolanaの環境設定を行ってください

Set up your environment with Node >= v20.9.0.

Follow the instructions for setting up the Unchain environment and configure your Solana environment accordingly.

https://github.com/unchain-tech/UNCHAIN-projects/blob/main/docs/Solana-dApp/ja/section-2/lesson-1_Solana%20%E3%81%AE%E9%96%8B%E7%99%BA%E7%92%B0%E5%A2%83%E3%82%92%E6%A7%8B%E7%AF%89%E3%81%97%E3%82%88%E3%81%86%EF%BC%81.md

```bash
npm install -g @lightprotocol/zk-compression-cli
git clone git@github.com:Lightprotocol/light-protocol.git
```

. ./scripts/devenv.shは点を二つ付けることが重要です。これをしないとエラーがでます。
build.shまでの実行で30分近くかかります。

It is important to include two dots when running . ./scripts/devenv.sh. If you don't, it will cause an error. The process can take up to 30 minutes to complete, up to the execution of build.sh.

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
