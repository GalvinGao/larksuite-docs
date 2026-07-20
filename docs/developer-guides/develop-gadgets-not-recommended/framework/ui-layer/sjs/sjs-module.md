---
document_id: '7073692582769885190'
directory_id: '7072701762536538117'
title: SJS 模块
full_path: /uYjL24iN/uEjMuEjMuEjM/sjs/sjs-module
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- SJS
- SJS Module
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMuEjMuEjM/sjs/sjs-module
---

# SJS 模块

sjs 可以在独立的`sjs`文件中编写，也可以在 ttml 文件中使用 <sjs> 标签编写。每个 sjs 文件或者 sjs 标签内联的代码都是一个独立的模块。

## 使用方法

### .sjs 文件

```javascript
// utils/index.sjs

function formatTime(dateStr) {
  var date = getDate(dateStr);
  var year = date.getFullYear();
  var month = date.getMonth() + 1;
  var day = date.getDate();
  var hour = date.getHours();
  var minute = date.getMinutes();
  var second = date.getSeconds();

  return (
    [year, month, day].map(formatNumber).join("/") +
    " " +
    [hour, minute, second].map(formatNumber).join(":")
  );
}

function formatNumber(n) {
  n = n.toString();
  return n[1] ? n : "0" + n;
}

module.exports = {
  formatTime: formatTime,
};
```

### ttml 文件

在 ttml 文件中引用上面的`sjs`文件模块

```xml
<!-- pages/index.ttml -->
<sjs module="utils" src="../utils/index.sjs" />
<view>hello {{ utils.formatTime(myTime) }}</view>
```

### sjs 标签

上面的 sjs 文件，也可以编写在 sjs 标签内

```xml
<!-- 内联的sjs代码 -->

<sjs module="utils">
  function formatTime(dateStr) {
    // 代码省略
  }
  function formatNumber(n) {
    // 代码省略
  }
  module.exports = { formatTime: formatTime }
</sjs>
```

## require 和 module

sjs 文件模块或者内联模块都是独立的：

- 每个模块作用域是隔离的，模块内定义的变量和函数对于其它模块是不可见的
- 一个模块如果需要对外暴露变量或函数，需要通过 module.exports 导出对象
- 一个模块如果要引用另一模块导出的对象，需要通过 require 函数引入

### module 对象

每个模块都有一个内置的 module 对象，可以通过定义 module.exports 属性来导出变量或函数

```javascript
function tool() {
  // a tool function
}

module.exports = {
  foo: "bar",
};
module.exports.extra = {
  key: "value",
  tool: tool,
};
```

### require 函数

在 sjs 模块中引用其他 sjs 文件模块，可以使用 require 函数。注意：

- require 的路径必须是相对路径，且只能是 sjs 文件，不能引用 javascript
- 当一次引用一个模块后，会使用该模块的 module 生成一个单例对象，然后在多个页面、多个地方、多次引用时都指向该单例对象
- 不支持动态 require，比如 `var foo = "../some_path/" + bar; require(foo)`

示例代码：

```javascript
// page/a.sjs
module.exports = function (name) {
  return "hello " + name;
};

// page/b.sjs
var my = "world";
var say = require("./a.sjs");

module.exports = function () {
  return say(my);
};
```

## sjs 标签

`<sjs>` 标签可以用来引用 .sjs 文件模块，或者编写内联模块。它有两个属性 `module` 和 `src`。

### module 属性

module 属性用来定义模块名，其命名必须符合下面两个规则：

- 首字符必须是：字母（a-zA-Z），下划线（_）
- 剩余字符可以是：字母（a-zA-Z），下划线（_）， 数字（0-9）

如果在同一 ttml 文件内出现重复的 module 属性名，则后者会覆盖前者，对于不同的 ttml 文件，可以使用重复的 module 属性名

### src 属性

当需要引入一个 sjs 文件模块时，可以设置 src 属性，注意 src 必须是一个相对路径。

## tips

- `<sjs>` 模块只能在定义模块的 ttml 文件中被访问到。使用 `<include>` 或 `<import>` 时，`<sjs>` 模块不会被引入到对应的 ttml 文件中
- `<template>` 标签中，只能使用定义该 `<template>` 的 ttml 文件中定义的 `<sjs>` 模块
