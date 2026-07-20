---
document_id: '7180270043522564102'
directory_id: '7179507279661809670'
title: min-height
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/min-height
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- Box Model
- min-height
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/min-height
---

# min-height

## 介绍

用来给给定元素设置最小高度。它可以阻止 `height` 属性的应用值小于 `min-height` 的值。 `min-height` 的值会同时覆盖 `max-height` 和 `height`。

## 语法

```css
/* <length> */

min-height: 120px;

min-height: 10em;

/* <percentage> */

min-height: 75%;
```

### 取值

-   `<length>`

固定的最小高度。负值会让该声明失效。

-   `<percentage>`

以父级块级容器高度的百分比作为最小高度。

### 标准化语法

```css
min-height: <length> | <percentage>
```
