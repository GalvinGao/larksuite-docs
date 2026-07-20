---
document_id: '7180269945547145221'
directory_id: '7179507279661842438'
title: text-align
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-align
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- text
- text-align
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-align
---

# text-align

## 介绍

定义行内内容（例如文字）如何相对它的块父元素对齐。

它并不控制块元素自己的对齐，只控制它的行内内容的对齐。

## 语法

```css
text-align: left;

text-align: right;

text-align: center;

text-align: start;

text-align: end;
```

### 取值

-   `left`

行内内容向左侧边对齐。

-   `right`

行内内容向右侧边对齐。

-   `center`

行内内容居中。

-   `start`

文字对齐阅读开始方向侧边。

`LTR`模式下等加`left`；`RTL`模式下等价`right`。

-   `end`

文字对齐阅读开始方向侧边。

`LTR`模式下等加`right`；`RTL`模式下等价`left`。

## 标准化语法

```css
text-align: left | right | center | start | end
```
