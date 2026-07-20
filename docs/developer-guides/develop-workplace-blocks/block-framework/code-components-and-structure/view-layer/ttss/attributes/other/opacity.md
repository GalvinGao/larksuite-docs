---
document_id: '7180269945548374021'
directory_id: '7179507279661826054'
title: opacity
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/other/opacity
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- other
- opacity
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/other/opacity
---

# opacity

## 介绍

`opacity`属性指定了一个元素的透明度。

## 语法

```css
opacity: 0;

opacity: 0.3;

opacity: 1;
```

### 取值

-   `number`

number 是一个`0.0`到`1.0`范围内的数字值，默认值为`1`。这个数值既包含也代表通道的透明度，也就是`alpha`通道的值。任何一个溢出这个取值区间的值，尽管有效，但会被解析为在取值范围内最靠近它的值。

| 值         | 释义      |
| --------- | ------- |
| 0         | 元素完全透明  |
| 0 < n < 1 | 元素半透明   |
| 1         | 元素完全不透明 |

### 标准化语法

```css
opacity: <number>
```
