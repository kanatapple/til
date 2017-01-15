# babel-node

babel-cli をインストールすると一緒にインストールされる  
本番向けのものではない  
babel の REPL(対話型評価環境)

## 使い方

REPL 起動

```bash
babel-node
```

コードを実行

```bash
babel-node -e "class Test {}"
```

ソースコード(test.js)をコンパイルして実行

```bash
babel-node test
```
