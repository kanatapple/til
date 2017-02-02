# Template Syntax

## Class binding

```html
<!-- hoge は変数 -->
<div [class]="hoge"></div>
```

```html
<!-- isTrue の値によって付け加える class を変えたい時 -->
<div [class]="'hoge' + (isTrue ? ' true' : ' false')"></div>
```