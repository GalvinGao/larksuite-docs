---
document_id: '7180270043522416646'
directory_id: '7179507279661907974'
title: 框架介绍
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/block-framework-introduction
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Block Framework Introduction
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/block-framework-introduction
---

# 框架介绍

小组件（Block）开发框架的目标是以简单、高效的方式让开发者构建Lark套件小组件，因此在设计上，沿用了Lark小程序的 DSL 语言，降低开发者的学习门槛。

整个框架分为两部分：业务渲染层（Render）和业务逻辑层（JS Runtime）。

-   业务渲染层：由 TTML 和 TTSS 组成，负责将业务逻辑层提供的数据转换为视图，呈现最终的渲染结果。
-   业务逻辑层：由 JS 和 JSON 描述组成，为业务渲染层提供数据，并响应来自于业务渲染层的用户操作事件。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a632b313d1bcac5e2de10a6abf585391_j95VIpHojl.png?lazyload=true&width=3280&height=1826)

## [响应式的数据绑定](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttml/data-binding)

框架核心是一个响应式的数据绑定系统，可以让数据与视图非常简单地保持同步。当做数据修改的时候，只需要在逻辑层修改数据，视图层就会做相应的更新。

```html
<!-- test.ttml -->
<view>
    <text bindtap="onTap">{{test}}</text>
</view>
```

```javaScript
// test.js
Block({
    data: {
        test: 'hello'
    },

    methods: {
        onTap() {
            this.setData({
                test: 'world'
            });
        }
    }
})
```

-   小组件（Block）框架会自动将逻辑层数据中的`test`和视图层的`test`进行绑定，当小组件首次渲染时，会显示文本“hello”。
-   点击文本，视图层会发送一个`tap`事件给逻辑层，逻辑层触发对应的事件处理函数 `onTap`。
-   回调函数执行，通过`setData`修改`test`的值为“world”，视图层的文本内容会自动更新为“world”。

## [基础组件](/document/uAjLw4CM/uYjL24iN/block/component/basic-components)

小组件（Block）框架内置了很多基础组件，这些组件会有内置的样式和特殊逻辑，开发者可以通过组合这些基础组件，开发出丰富的小组件应用。

## [API](/document/uAjLw4CM/uYjL24iN/block/api/api-introduction)

小组件（Block）框架提供了一系列的 API 接口，用于获取更底层的能力，比如获取用户信息、本地存储、网络请求等。
