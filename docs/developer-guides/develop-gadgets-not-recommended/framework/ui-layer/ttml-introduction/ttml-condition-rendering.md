---
document_id: '6965379541104148485'
directory_id: '6907567266540453890'
title: 条件渲染
full_path: /uYjL24iN/uIDOuIDOuIDO
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- TTML Introduction
- TTML condition rendering
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDOuIDOuIDO
---

# 条件渲染

## tt:if

在框架中，使用 tt:if="{{condition}}" 来判断是否需要渲染该代码块：

```html
<!-- index.ttml -->
<view tt:if="{{condition}}"> True </view>
```

```js
// index.js
Page({
  data: {
    condition: true
  }
})
```

也可以用 tt:elif 和 tt:else 来添加一个 else 块：

```html
<view tt:if="{{length > 5}}"> 1 </view>
<view tt:elif="{{length > 2}}"> 2 </view>
<view tt:else> 3 </view>
```

## block tt:if

如果要同时控制多个标签的渲染，可以使用 block 和 tt:if，block 不会在实际展示页面中产生任何标签。

```html
<block tt:if="{{true}}">
  <view> view1 </view>
  <view> view2 </view>
</block>
```

## hidden

我们也可以通过一个更加简单的属性来控制标签的展示。

```html
<!-- index.ttml -->
<view hidden="{{condition}}"> True </view>
```

::: note
因为 tt:if 之中的模板也可能包含数据绑定，所有当 tt:if 的条件值切换时，框架有一个局部渲染的过程，因为它会确保条件块在切换时销毁或重新渲染。
<br>同时 tt:if 也是惰性的，如果在初始渲染条件为 false，框架什么也不做，在条件第一次变成真的时候才开始局部渲染。
<br>相比之下，hidden 就简单的多，组件始终会被渲染，只是简单的控制显示与隐藏。
:::

::: note
一般来说，tt:if 有更高的切换消耗而 hidden 有更高的初始渲染消耗。因此，如果需要频繁切换的情景下，用 hidden 更好，如果在运行时条件不大可能改变则 tt:if 较好。
:::

