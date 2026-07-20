---
document_id: '7180270043522301958'
directory_id: '7179507279662006278'
title: '@keyframes'
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/rules/keyframes
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- '@ rules'
- '@keyframes'
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:38Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/rules/keyframes
---

# @keyframes

关键帧`@keyframes`规则，通过在动画序列中定义关键帧的样式来控制 TTSS 动画序列中的中间步骤。和`transition`相比，`keyframes`可以控制动画序列的中间步骤。

## 特性

### 属性个数不定

如果一个关键帧中没有出现其他关键帧中的属性，那么这个属性将使用插值（不能使用插值的属性除外，这些属性会被忽略掉）。例如：

```css
@keyframes identifier {
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
```

### 关键帧取值

-   `from`

等价于`0%`。

-   `to`

等价于`100%`。

-   百分比

动画序列中触发关键帧的时间点，使用百分值来表示。

### 支持的属性

`keyframes`动画仅支持`transform`、`opacity`、`background-color`属性。
