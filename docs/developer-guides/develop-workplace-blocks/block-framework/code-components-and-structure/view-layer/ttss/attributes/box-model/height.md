---
document_id: '7180269945544589318'
directory_id: '7179507279661809670'
title: height
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/height
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- Box Model
- height
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/height
---

# height


## 介绍

指定了一个元素的高度。

默认情况下，这个属性决定的是内容区（`content area`）的高度，但是如果将 `box-sizing` 设置为 `border-box`, 这个属性决定的将是边框区域（`border area`）的高度。

## 语法

```css
/* <length> */
height: 120px;
height: 10em;

/* <percentage> */
height: 75%;
```

### 取值

-   `auto`

容器的高度将由其内容的总高度决定。

-   `<length>`

将高度定义为一个绝对值。

-   `<percentage>`

将高度定义为相对包含块高度的百分比。

### 标准化语法

```css
height: auto | <length> | <percentage>
```

