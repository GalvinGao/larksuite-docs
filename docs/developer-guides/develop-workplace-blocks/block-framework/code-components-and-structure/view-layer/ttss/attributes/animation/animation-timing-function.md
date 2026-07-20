---
document_id: '7180269945548587013'
directory_id: '7179507279661957126'
title: animation-timing-function
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-timing-function
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- animation
- animation-timing-function
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-timing-function
---

# animation-timing-function

## 介绍

用于设置播放动画的时间函数。

## 语法

```css
/* keyword */

animation-timing-function: ease;

animation-timing-function: ease-in;

animation-timing-function: ease-out;

/* Function values */

animation-timing-function: cubic-bezier(0.1, 0.7, 1, 0.1);
```

### 取值

-   `linear`


-   `ease`


-   `ease-in`


-   `ease-out`


-   `ease-in-out`


-   `square-bezier()`


-   `cubic-bezier()`

## 标准化语法

```css
animation-timing-function: linear | ease | ease-in | ease-out | ease-in-out | <function>
```

## 注意事项

-   iOS 不支持贝塞尔曲线插值大于`1`（iOS 平台限制）。
