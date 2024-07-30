# Light Protocolの使用方法

このプロジェクトでは、ブラウザ環境でZK Compression APIと対話するために`@lightprotocol/stateless.js`を使用する方法を示します。

## モノレポのビルド

以下の手順で環境設定をします。
Node >= v20.9.0に設定をします。

Unchainの環境設定の方法を見ながらSolonaの環境設定を行ってください
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

##　参照

https://www.zkcompression.com/

https://docs.lightprotocol.com/

https://github.com/Lightprotocol/light-protocol/tree/main/cli
