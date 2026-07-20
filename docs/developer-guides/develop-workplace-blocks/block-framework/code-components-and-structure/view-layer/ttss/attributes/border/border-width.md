---
document_id: '7180270043523170310'
directory_id: '7179507279661858822'
title: border-width
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-width
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- border
- border-width
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-width
---

# border-width

## 介绍

用于设置四个边框的宽度，该属性可按顺序设置上边框宽度、右边框宽度、下边框宽度、左边框宽度。

## 语法

```css
/* 四个边框都是细边框 */
border-width: thin;
/* 上边框是细边框 | 右边框是中等边框 | 下边框缺省与上边框相同 | 左边框缺省与右边框相同 */
border-width: thin medium;
/* 上边框是细边框 | 右边框是中等边框 | 下边框是粗边框 | 左边框缺省与右边框相同 */
border-width: thin medium thick;
/* 上边框是细边框 | 右边框是中等边框 | 下边框是粗边框 ｜ 左边框是10px宽的边框 */
border-width: thin medium thick 10px;
```

### 取值

-   `thin`

细边框。

-   `medium`

中等边框。

-   `thick`

粗边框。

-   `<length>`

以具体的尺寸定义边框宽度

## 标准化语法

```css
border-width: [thin | medium | thick | <length>]{1, 4}
```


## 例子

```css
border-width: thin;
border-width: thin medium;
border-width: thin medium thick;
border-width: thin medium thick 10px;
border-width: 5px 10px 15px 20px;
```
