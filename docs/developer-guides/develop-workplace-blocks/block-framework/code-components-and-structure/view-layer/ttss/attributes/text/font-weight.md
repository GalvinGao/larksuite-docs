---
document_id: '7180269945547210757'
directory_id: '7179507279661842438'
title: font-weight
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-weight
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- text
- font-weight
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-weight
---

# font-weight

## 介绍

设置字体粗细。

## 语法

```css
font-weight: bold;

font-weight: normal;

font-weight: 100;

font-weight: 200;

...

font-weight: 900;
```

### 取值

-   `bold`

粗体，与 700 字重相同。

-   `normal`

普通，与 400 字重相同。

-   `<number>`

只能使用 100、200、300、400、500、600、700、800、900，其他值无效。当取值为 500 - 900 时会加粗。

## 例子

```html
<text style="font-weight: normal;">font-weight: normal; </text>



<text style="font-weight: bold;">font-weight: bold; </text>



<text style="font-weight: 100;">font-weight: 100; </text>



<text style="font-weight: 400;">font-weight: 400; </text>



<text style="font-weight: 500;">font-weight: 500; </text>



<text style="font-weight: 900;">font-weight: 900; </text>
```
