---
document_id: '7073691561008611333'
directory_id: '7072701762536538117'
title: SJS 语句
full_path: /uYjL24iN/uEjMuEjMuEjM/sjs/sjs-statement
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- SJS
- SJS Statement
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMuEjMuEjM/sjs/sjs-statement
---

# SJS 语句

## If 语句

可以使用以下格式的 if 语句

- **if** **(** *expression* **)** *statement*
- **if** **(** *expression* **)** *statement1* **else** *statement2*

If 语句之间可以互相嵌套

```javascript
// if ...
if (表达式) 语句;

if (表达式) 语句;

if (表达式) {
  代码块;
}

// if ... else
if (表达式) 语句;
else 语句;

if (表达式) 语句;
else 语句;

if (表达式) {
  代码块;
} else {
  代码块;
}

// if ... else if ... else ...
if (表达式) {
  代码块;
} else if (表达式) {
  代码块;
} else if (表达式) {
  代码块;
} else {
  代码块;
}
```

## Switch 语句

### 示例语法

```javascript
 switch (表达式) {
  case 变量:
    语句;
  case 数字:
    语句;
    break;
  case 字符串:
    语句;
  default:
    语句;
}
```

- default 分支可以省略不写
- case 关键词后面只能使用：变量，数字，字符串

## For 语句

### 示例语法

```javascript
for (语句; 语句; 语句) 语句;

for (语句; 语句; 语句) {
  代码块;
}
```

- 支持 **break** **continue** 关键字

## While 语句

### 示例语法

```javascript
while (表达式) 语句;

while (表达式) {
  代码块;
}

do {
  代码块;
} while (表达式);
```

- 当表达式为 true 时，循环执行语句或代码块
- 支持 **break** **continue** 关键字
