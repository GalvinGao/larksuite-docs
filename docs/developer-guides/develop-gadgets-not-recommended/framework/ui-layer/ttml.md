---
document_id: '6965379543684562950'
directory_id: '6907567266536669185'
title: TTML
full_path: /uYjL24iN/ugzNugzNugzN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- TTML
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugzNugzNugzN
---

# TTML

TTML 是用来编写页面结构用的标签语言。

要完整了解 TTML的语法，请参考 [TTML语法介绍](/document/uYjL24iN/uADOuADOuADO)。

主要包括下面一些特性：

## [数据绑定](/document/uYjL24iN/uADOuADOuADO) 

```html
<!--ttml-->
<view> {{message}} </view>
```

```js
// page.js
Page({
  data: {
    message: 'Hello World!'
  }
})
```

## [列表渲染](/document/uYjL24iN/uEDOuEDOuEDO) 

```html
<!--ttml-->
<view tt:for="{{array}}"> {{item}} </view>
```

```js
// page.js
Page({
  data: {
    array: [1, 2, 3, 4, 5]
  }
})
```

## [条件渲染](/document/uYjL24iN/uIDOuIDOuIDO)

```html
<!--ttml-->
<view tt:if="{{view == 'A'}}"> A </view>
<view tt:elif="{{view == 'B'}}"> B </view>
<view tt:else> C </view>
```

```js
// page.js
Page({
  data: {
    view: 'A'
  }
})
```

## [模版](/document/uYjL24iN/uMDOuMDOuMDO)

```html
<!--ttml-->
<template name="staffName">
  <view>
    FirstName: {{firstName}}, LastName: {{lastName}}
  </view>
</template>

<template is="staffName" data="{{...staffA}}"></template>
<template is="staffName" data="{{...staffB}}"></template>
<template is="staffName" data="{{...staffC}}"></template>
```

```js
// page.js
Page({
  data: {
    staffA: {firstName: '大林', lastName: '斯'},
    staffB: {firstName: '吉尔', lastName: '丘'},
    staffC: {firstName: '福', lastName: '罗思'}
  }
})
```

## [事件](/document/uYjL24iN/uQDOuQDOuQDO)

```html
<!--ttml-->
<view bindtap="add"> {{count}} </view>
```

```js
// page.js
Page({
  data: {
    count: 1
  },
  add: function(e) {
    this.setData({
      count: this.data.count + 1
    })
  }
})
```
