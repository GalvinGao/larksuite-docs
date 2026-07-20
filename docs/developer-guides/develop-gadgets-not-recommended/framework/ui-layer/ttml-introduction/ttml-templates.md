---
document_id: '6965379543683891206'
directory_id: '6907567266540453890'
title: 模板
full_path: /uYjL24iN/uMDOuMDOuMDO
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- TTML Introduction
- TTML templates
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDOuMDOuMDO
---

# 模板

TTML提供模板（template），可以在模板中定义代码片段，然后在不同的地方调用。

## 定义模板

使用 name 属性，作为模板的名字。然后在`<template/>`内定义代码片段，如：

```html
<!--
  index: int
  msg: string
  time: string
-->
<template name="msgItem">
  <view>
    <text> {{index}}: {{msg}} </text>
    <text> Time: {{time}} </text>
  </view>
</template>
```

## 使用模板

使用 is 属性，声明需要的使用的模板，然后将模板所需要的 data 传入，如：

```html
<!-- index.ttml -->
<template is="msgItem" data="{{...item}}"/>
```

```js
// index.js
Page({
  data: {
    item: {
      index: 0,
      msg: 'this is a template',
      time: '2016-09-15'
    }
  }
})
```

还可以动态决定具体需要渲染哪个模板：

```html
<template name="odd">
  <view> odd </view>
</template>
<template name="even">
  <view> even </view>
</template>

<block tt:for="{{[1, 2, 3, 4, 5]}}">
    <template is="{{item % 2 == 0 ? 'even' : 'odd'}}"/>
</block>
```

## 模板的作用域

模板拥有自己的作用域，只能使用 data 传入的数据。
