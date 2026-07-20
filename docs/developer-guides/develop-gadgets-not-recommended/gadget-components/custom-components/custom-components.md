---
document_id: '6965379541104951301'
directory_id: '6907567266540830722'
title: 自定义组件
full_path: /uYjL24iN/ugTOugTOugTO
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Components
- Custom Components
- Custom Components
document_type: GuideDocumentType
updated_at: 2022-03-11T04:12:08Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugTOugTOugTO
---

# 自定义组件
::: note
从Lark版本 1.18 开始，小程序支持简洁的组件化编程。所有自定义组件相关特性都需要客户端 1.18 版本或更高。
:::

开发者可以将页面内的功能模块抽象成自定义组件，以便在不同的页面中重复使用；也可以将复杂的页面拆分成多个低耦合的模块，有助于代码维护。自定义组件在使用时与基础组件非常相似。

## 创建自定义组件

类似于页面，一个自定义组件由 `json`、 `ttml`、`ttss`、`js` 4 个文件组成。要编写一个自定义组件，首先需要在 `json` 文件中进行自定义组件声明（将 `component` 字段设为 `true` 可将这一组文件设为自定义组件）：

```js
{
  "component": true
}
```

同时，还要在 `ttml` 文件中编写组件模版，在 `ttss` 文件中加入组件样式，它们的写法与页面的写法类似。具体细节和注意事项参见 [组件模版和样式](/document/uYjL24iN/ukTOukTOukTO) 。

**示例代码 (自定义组件的内部 TTML 结构):**

```html
<view class="my-custom-component">
  <text class="header">{{ headerText }}</text>
  <view class="content">
    <slot></slot>
  </view>
</view>
```

下列样式只应用于这个自定义组件

```css
.header {
  color: green;
}
```

在自定义组件的 `js` 文件中，需要使用 `Component()` 来注册组件，并提供组件的属性定义、内部数据和自定义方法。

组件的属性值和内部数据将被用于组件 `ttml` 的渲染，其中，属性值是可由组件外部传入的。更多细节参见 [Component 构造器](/document/uYjL24iN/uADMx4CMwEjLwATM)。

```js
Component({
  properties: {
    // 这里定义了 headerText 属性，属性值可以在组件使用时指定
    headerText: {
      type: String,
      value: '默认标题文案',
    }
  },
  data: {
    // 组件内部数据
    defaultStates: {}
  },
  methods: {
    // 自定义方法
    customMethod: function(){}
  }
})
```

## 使用自定义组件

使用已注册的自定义组件前，首先要在页面的 `json` 文件中进行引用声明。此时需要提供每个自定义组件的标签名和对应的自定义组件文件路径：

```js
{
  "usingComponents": {
    "my-component": "path/to/a/custom/component"
  }
}
```

这样，在页面的 `ttml` 中就可以像使用基础组件一样使用自定义组件。节点名即自定义组件的标签名，节点属性即传递给组件的属性值。

```html
<view class="component-wrapper">
  <my-component header-text="My Title"></my-component>
</view>
```

自定义组件的 `ttml` 节点结构在与数据结合之后，将被插入到引用位置内。

::: note
因为 `ttml` 节点标签名只能是小写字母、中划线和下划线的组合，所以自定义组件的标签名也只能包含这些字符。<br>自定义组件也是可以引用自定义组件的，引用方法类似于页面引用自定义组件的方式（使用 `usingComponents` 字段）。

注意：自定义组件不能与原生组件重名
:::

