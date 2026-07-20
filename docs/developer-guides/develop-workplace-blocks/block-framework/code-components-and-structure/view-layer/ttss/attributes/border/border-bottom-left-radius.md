---
document_id: '7180270043522793478'
directory_id: '7179507279661858822'
title: border-bottom-left-radius
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom-left-radius
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- border
- border-bottom-left-radius
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom-left-radius
---

# border-bottom-left-radius

## 介绍

用于添加左下角圆角边框。第一个值是水平半径，第二个值是垂直半径。如果省略第二个值，则复制第一个值。如果长度为零，则边角为方形，而不是圆形。水平半径的百分比值参考边框盒的宽度，而垂直半径的百分比值参考边框盒的高度。

## 语法

```css
/* the corner is a circle */
/* border-bottom-left-radius: radius */
border-bottom-left-radius: 3px;
/* the corner is an ellipsis */
/* border-bottom-left-radius: horizontal vertical */
border-bottom-left-radius: 0.5em 1em;
```

### 取值

-   `<length>`

定义圆角的形状。

-   `<percentage>`

以百分比定义圆角的形状

## 标准化语法

```css
border-bottom-left-radius: [<length> | <percentage>] [ / [<length> | <percentage>]]?
```
