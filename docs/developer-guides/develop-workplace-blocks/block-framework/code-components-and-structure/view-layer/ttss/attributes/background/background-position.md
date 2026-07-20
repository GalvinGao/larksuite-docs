---
document_id: '7180269945544474630'
directory_id: '7179507279661776902'
title: background-position
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-position
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- background
- background-position
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-position
---

# background-position

## 介绍

`background-position`属性指定背景图像的开始绘制位置。可以用逗号分隔的列表来描述一个或多个背景图片的起始绘制位置，在默认情况下，背景图片被放置在绘制范围内的左上角。

## 语法

```css
background-position: left top;
```

### 标准化语法

```css
background-position: [left | center | right | top | bottom] [left | center | right | top | bottom]] | [[<length> | <percentage>] [<length> | <percentage>]
```

## 例子

```css
background-position: left top;
background-position: 50% 50%;
background-position: 30px 40px;
background-position: 50% 40px, right bottom;
```
