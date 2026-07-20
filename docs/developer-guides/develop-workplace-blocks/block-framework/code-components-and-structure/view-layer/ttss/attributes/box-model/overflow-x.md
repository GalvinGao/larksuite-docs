---
document_id: '7180269945548193797'
directory_id: '7179507279661809670'
title: overflow-x
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/overflow-x
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- Box Model
- overflow-x
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/overflow-x
---

# overflow-x

## 介绍

当块级元素的内容在水平方向发生溢出时如何进行裁剪。

## 语法

```css
overflow-x: visible;

overflow-x: hidden;

overflow-x: auto;
```

### 取值

-   `hidden`

**默认值。** 超出元素框的内容默认会被裁剪。

-   `visible`

内容不会被修剪，可以呈现在元素框之外。

-   `auto`

取决于用户代理。如果内容适合填充框内部，则它看起来与可见内容相同，但仍会建立新的块格式化上下文。如果内容溢出，渲染器会提供滚动条。

### 标准化语法

```css
overflow-x: hidden | visible | auto
```
