---
document_id: '7180269945547456517'
directory_id: '7179507279661776902'
title: background-size
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-size
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- background
- background-size
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-size
---

# background-size

## 介绍

`background-size`属性设定背景图片的大小。可以用逗号分隔的列表来描述多个背景图片的大小。

## 语法

```css
background-size: 50%;
background-size: 40px, 30px;
background-size: cover;
background-size: 50% 50%, contain;
```

### 取值

-   `auto`

**默认值。** 背景图片保留其原始的尺寸。

-   `<length>`

设置背景图片的宽和高，如果只给出一个值，那么另一个值自动设为`auto`。

-   `<percentage>`

以绘制范围百分比的形式设置背景图片的宽高。同理，如果只给出一个值，另一个值为`auto`。

-   `cover`

重设图片大小以保证图片可以完全覆盖整个边框，超出的部分会被裁掉。

-   `contain`

重设背景图片的大小且保证图片完全可见（某一边完全贴合）。

### 标准化语法

```css
background-size: [<length> | <percentage> | auto ]{1,2} | cover | contain
```
