# パッケージ

## cross-env

クロスプラットフォームで環境変数を設定するやつ

```bash
cross-env NODE_ENV=production node app.js
```

複数設定することもできる

```bash
cross-env NODE_ENV=production TEST=test node app.js
```