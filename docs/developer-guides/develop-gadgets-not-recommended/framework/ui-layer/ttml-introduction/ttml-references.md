---
document_id: '6965379543684513798'
directory_id: '6907567266540453890'
title: 引用
full_path: /uYjL24iN/uUDOuUDOuUDO
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- TTML Introduction
- TTML references
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDOuUDOuUDO
---

# 引用

TTML 提供两种文件引用方式 import 和 include。


## import

`import` 可以在该文件中使用目标文件定义的 `template`，如：

在 `item.ttml` 中定义了一个叫 item 的 `template`：

```html
<!-- item.ttml -->
<template name="item">
  <text>{{text}}</text>
</template>
```

在 index.ttml 中引用了 item.ttml，就可以使用 item 模板：

```html
<!-- index.ttml -->
<import src="item.ttml"/>
<template is="item" data="{{text: 'forbar'}}"/>
```

## import 的作用域

`import` 有作用域的概念，即只会 `import` 目标文件中定义的 `template`，而不会 `import` 目标文件 `import` 的 `template`。

**如：C import B，B import A，在C中可以使用B定义的template，在B中可以使用A定义的template，但是C不能使用A定义的template。**

```html
<!-- A.ttml -->
<template name="A">
  <text> A template </text>
</template>
```

```html
<!-- B.ttml -->
<import src="a.ttml"/>
<template name="B">
  <text> B template </text>
</template>
```

```html
<!-- C.ttml -->
<import src="b.ttml"/>
<template is="A"/>  <!-- Error! Can not use tempalte when not import A. -->
<template is="B"/>
```

## include

include 可以将目标文件除了 `<template/>` 外的整个代码引入，相当于是拷贝到 include 位置，如：

```html
<!-- index.ttml -->
<include src="header.ttml"/>
<view> body </view>
<include src="footer.ttml"/>
```

```html
<!-- header.ttml -->
<view> header </view>
```

```html
<!-- footer.ttml -->
<view> footer </view>
```
