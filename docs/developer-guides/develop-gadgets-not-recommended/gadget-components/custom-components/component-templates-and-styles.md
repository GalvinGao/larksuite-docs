---
document_id: '6965379543683268614'
directory_id: '6907567266540830722'
title: 组件模版和样式
full_path: /uYjL24iN/ukTOukTOukTO
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Components
- Custom Components
- Component Templates and Styles
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukTOukTOukTO
---

# 组件模版和样式

类似于页面，自定义组件拥有自己的 `ttml` 模版和 `ttss` 样式。

## 组件模版

组件模版的写法与页面模板相同。组件模版与组件数据结合后生成的节点树，将被插入到组件的引用位置上。

在组件模板中可以提供一个 `<slot>` 节点，用于承载组件引用时提供的子节点。


```html
<!-- 自定义组件模板 -->
<view class="component-wrapper">
  <view>组件内部节点</view>
  <slot></slot>
</view>
```

```html
<!-- 引用自定义组件的页面模版 -->
<view>
  <my-component>
    <view>组件 slot 内容</view>
  </my-component>
</view>
```

::: note
在模版中引用到的自定义组件及其对应的节点名需要在 `json` 文件中显式定义，否则会被当作一个无意义的节点。
:::

## 模版数据绑定

与普通的 `ttml` 模版类似，可以使用数据绑定，这样就可以向子组件的属性传递动态数据。

```html
<!-- 引用自定义组件的页面模版 -->
<view>
  <my-component
    data-a="{{ data1 }}"
    data-b="{{ data2 }}">
    <view>组件 slot 内容</view>
  </my-component>
</view>
```

在以上例子中，组件的属性 `dataA` 和 `dataB` 将收到页面传递的数据。页面可以通过 `setData` 来改变绑定的数据字段。

## 组件的 slot

在组件的 `ttml` 中可以包含 `slot` 节点，用于承载组件使用者提供的 `ttml` 结构。

默认情况下，一个组件的 `ttml` 中只能有一个 `slot`。需要使用多 `slot` 时，应使用不同的 `name` 来区分。

```html
<!-- 自定义组件模板 -->
<view class="wrapper">
  <slot name="before"></slot>
  <view>Content gose here</view>
  <slot name="after"></slot>
</view>
```

使用时，用 `slot` 属性来将节点插入到不同的 `slot` 上。

```html
<!-- 引用自定义组件的页面模版 -->
<view>
  <my-component>
    <!-- 这部分内容将被放置在组件 <slot name="before"> 的位置上 -->
    <view slot="before">这里是插入到组件slot name="before"中的内容</view>

    <!-- 这部分内容将被放置在组件 <slot name="after"> 的位置上 -->
    <view slot="after">这里是插入到组件slot name="after"中的内容</view>
  </my-component>
</view>
```

## 组件样式

组件对应 `ttss` 文件的样式，只对组件 `ttml` 内的节点生效。编写组件样式时，需要注意以下几点：

- 继承样式，如 `font` 、`color` ，会从组件外继承到组件内。
- 除继承样式外，`app.ttss` 中的样式、组件所在页面的的样式对自定义组件无效。
