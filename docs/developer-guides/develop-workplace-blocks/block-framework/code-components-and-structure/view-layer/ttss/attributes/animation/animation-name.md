---
document_id: '7180270043522957318'
directory_id: '7179507279661957126'
title: animation-name
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-name
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- animation
- animation-name
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-name
---

# animation-name

## 介绍

用于设置播放动画的`Keyframe`的名字。

## 语法

```css
/* string */
animation-name: KeyframeName;
```

### 取值

-   `<string>`

定义动画的名称，默认为空字符串。

## 标准化语法

```css
animation-name: <string>
```

## 例子

```css
@keyframes good {
  0% {
    top: 0;
    left: 0;
  }
  30% {
    top: 50px;
  }
  68%,
  72% {
    left: 50px;
  }
  100% {
    top: 100px;
    left: 100%;
  }
}

.test {
    animation-name: good;
}
```
