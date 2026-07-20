---
document_id: '7180269945548161029'
directory_id: '7179507279661809670'
title: padding
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/padding
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- Box Model
- padding
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/padding
---

# padding

## 介绍

TTSS 简写属性，控制元素所有四条边的内边距区域。一个元素的内边距区域指的是其内容与其边框之间的空间。

## 语法

```css
/* 应用于所有边 */

padding: 1em;

/* 上边下边 | 左边右边 */

padding: 5% 10%;

/* 上边 | 左边右边 | 下边 */

padding: 1em 2em 2em;

/* 上边 | 右边 | 下边 | 左边 */

padding: 5px 1em 0 2em;
```

### 取值

-   `<length>`

以固定值为内边距。

-   `<percentage>`

相对于包含块的宽度，以百分比值为内边距。

### 标准化语法

```css
padding: [<length> | <percentage>]{1, 4}
```

## 例子

```html
<view class="a"><text>此元素有合适的内边距。</text></view>
<view class="b"><text>此元素的内边距很大！</text></view>
```

```css
.a {
  background-color: red;
  padding: 20px 50px;
}

.b {
  background-color: blue;
  padding: 110px 50px 50px 110px;
}
```
