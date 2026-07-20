---
document_id: '7180269945548570629'
directory_id: '7179507279661957126'
title: animation-duration
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-duration
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- animation
- animation-duration
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-duration
---

# animation-duration

## 介绍

用于设置动画持续的时间。

## 语法

```css
/* time */
animation-duration: 100ms;
```

### 取值

-   `<time>`

定义动画的持续时间，默认为`0`。

### 标准化语法

```css
animation-duration: <time>
```

## 注意事项

-   如果持续时间为`0`，则动画不开启。
-   需要注明时间单位（`s`, `ms`），否则动画异常。
