---
document_id: '7180269945547735045'
directory_id: '7179507279661842438'
title: text-shadow
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-shadow
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- text
- text-shadow
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-shadow
---

# text-shadow

## 介绍

为文字添加阴影。可以为文字添加多个阴影，阴影值之间用逗号隔开。每个阴影值由元素在 X 和 Y 方向的偏移量、模糊半径和颜色值组成。

## 语法

```css
/** 无阴影 */

text-shadow: none;

/** offset-x | offset-y | blur-radius | color */

text-shadow: 1px 1px 2px red;
```

## 标准化语法

```css
text-shadow: none | [ <length>{3} <color> ]
```
