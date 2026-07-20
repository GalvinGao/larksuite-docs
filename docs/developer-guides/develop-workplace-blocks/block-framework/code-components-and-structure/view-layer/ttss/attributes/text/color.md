---
document_id: '7180270043523153926'
directory_id: '7179507279661842438'
title: color
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/color
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- text
- color
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/color
---

# color

## 介绍

`color`属性指定文本的颜色。

## 语法

```css
/* <named-color> values */
color: red;
color: orange;
color: tan;
/* <hex-color> values */
color: #090;
color: #009900;
color: #090a;
color: #009900aa;
/* <rgb()> values */
color: rgb(34, 12, 64, 0.6);
color: rgba(34, 12, 64, 0.6);
color: rgb(34 12 64 / 0.6);
color: rgba(34 12 64 / 0.3);
color: rgb(34 12 64 / 60%);
color: rgba(34.6 12 64 / 30%);
/* <hsl()> values */
color: hsl(30, 100%, 50%, 0.6);
color: hsla(30, 100%, 50%, 0.6);
color: hsl(30 100% 50% / 0.6);
color: hsla(30 100% 50% / 0.6);
color: hsl(30 100% 50% / 60%);
color: hsla(30.2 100% 50% / 60%);
```

### 取值

-   `<color>`

-   `<gradient>`

## 标准化语法

```css
color: <color> | <gradient>
```

## 例子

```html
<text class="container text1">文本颜色test</text>
<text class="container text2">文本颜色test</text>
<text class="container text3">文本颜色test</text>
<text class="container text4">文本颜色test</text>
<text class="container text5">文本颜色test</text>
<text class="container text6">文本颜色test</text>
<text class="container text7">文本颜色test</text>
<text class="container text8">文本颜色test</text>
```

```css
.text1 {
  color: rgb(201, 76, 76);
}

.text2 {
  color: rgba(201, 76, 76, 0.6);
}

.text3 {
  color: hsl(89, 43%, 51%);
}

.text4 {
  color: hsla(89, 43%, 51%, 0.6);
}

.text5 {
  color: #ff0;
}

.text6 {
  color: #ff0000;
}

.text7 {
  color: #00ff00ff;
}

.text8 {
  color: orange;
}
```

## 注意事项

为`gradient`类型时，只有`text`文本组件有效。
