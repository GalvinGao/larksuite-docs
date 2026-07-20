---
document_id: '7180270043523006470'
directory_id: '7179507279661776902'
title: background
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- background
- background
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background
---

# background

## 介绍

`background`属性是如下属性的简写： `background-color` `background-image`

`background`并不要求如上每一种属性都有，没有的即采取默认属性。

## 语法

```css
/* bg-color bg-image */
background: green url('https://www.w3school.com.cn/i/bg_flower.gif');
```

### 标准化语法

```css
background: <bg-color> || <bg-image>
```

## 例子

```html
<text class="warning">Here is a paragraph<text></text></text>
```

```css
.warning {
  background: pink;
}
```

## 注意事项

-   background 组合属性目前仅支持`  background-color ` & `background-image`，其他背景相关属性都必须单独书写
