# Tips

## Parsing error: 'import' and 'export' may appear only with 'sourceType: module'

上記エラーが出る場合は、`parserOptions` に `"sourceType": "module"` を指定する

```json
{
  "parserOptions": {
    "sourceType": "module"
  }
}
```