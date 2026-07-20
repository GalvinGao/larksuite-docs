---
document_id: '7073693024735920133'
directory_id: '7072701762536538117'
title: SJS 变量
full_path: /uYjL24iN/uEjMuEjMuEjM/sjs/sjs-variable
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- SJS
- SJS Variable
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMuEjMuEjM/sjs/sjs-variable
---

# SJS 变量

使用 `var` 关键字声明变量

## 语法规范

- `var` 与 JavaScript 中表现一致，会有变量提升
- 没有声明的变量直接赋值使用，会被定义为全局变量
- 只声明变量而不赋值，默认值为 `undefined`

```javascript
var foo = 1;
var bar = "hello world";
var a; // a=== undefined
```

## 变量名

### 命名规则

- 首字符必须是：字母（a-z,A-Z），下划线（_）
- 首字母以外的字符可以是：字母（a-z,A-Z），下划线（_），数字（0-9）

### 保留标识符

和 JavaScript 一样，以下标识符无法使用

```
arguments
break
case
continue
default
delete
do
else
false
for
function
if
Infinity
NaN
null
require
return
switch
this
true
typeof
undefined
var
void
while
```
