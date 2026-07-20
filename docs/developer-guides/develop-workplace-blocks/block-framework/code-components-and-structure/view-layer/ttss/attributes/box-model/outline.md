---
document_id: '7180269945544196102'
directory_id: '7179507279661809670'
title: outline
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/outline
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- Box Model
- outline
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/outline
---

# outline

## 介绍

用于设置轮廓的样式。

该属性可按顺序设置 `outline-color`、`outline-style`、`outline-width`。

`border` 和 `outline` 很类似，但有如下区别：

-   轮廓不占据空间，绘制于元素内容周围。

-   根据规范，轮廓通常是矩形，但也可以是非矩形的。

## 语法

```css
/* 样式 */

outline: solid;

/* 颜色 | 样式 */

outline: #f66 dashed;

/* 样式 | 宽度 */

outline: inset thick;

/* 颜色 | 样式 | 宽度 */

outline: green solid 3px;
```

### 标准化语法

```css
outline: <outline-color> || <outline-style> || <outline-width>
```

## 例子

```html
<view> <text>This text has a special focus style. </text></view>
```

```css
view {
  border: 1px solid;
  border-radius: 3px;
  margin: 10px;
  padding: 5px;
  outline: 4px dotted #e73;
  outline-offset: 4px;
  background: #ffa;
}
```
