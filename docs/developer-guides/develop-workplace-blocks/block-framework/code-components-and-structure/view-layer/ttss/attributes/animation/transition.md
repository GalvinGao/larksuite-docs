---
document_id: '7180269945547849733'
directory_id: '7179507279661957126'
title: transition
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/transition
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- animation
- transition
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/transition
---

# transition

## 介绍

用于设置元素过渡动画效果，是`transition-property`、`transition-duration`、`transition-timing-function`、`transition-delay`的简写属性。

## 语法

```css
/* property name | duration | delay */
transition: width 4s 1s;

/* property name | duration | timing function */

transition: background-color 4s ease-in-out;

/* property name | duration | timing function | delay */

transition: opacity 4s ease-in-out 1s;
```

## 标准化语法

```css
transition: <transition-property> || <transition-duration> || <transition-timing-function> || <transition-delay>
```
