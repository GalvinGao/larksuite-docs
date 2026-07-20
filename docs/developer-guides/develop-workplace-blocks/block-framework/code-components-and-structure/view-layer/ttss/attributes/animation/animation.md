---
document_id: '7180270043523284998'
directory_id: '7179507279661957126'
title: animation
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- animation
- animation
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation
---

# animation

## 介绍

用于设置动画。

它是`animation-name`，`animation-duration`, `animation-timing-function`，`animation-delay`，`animation-iteration-count`，`animation-direction`，`animation-fill-mode`和`animation-play-state`属性的一个简写属性形式。

## 语法

```css
/* @keyframes name | duration | timing-function | delay |

iteration-count | direction | fill-mode | play-state */

animation: slidein 3s ease-in 1s 2 reverse both paused;

/* @keyframes name | duration | timing-function | delay */

animation: slidein 3s linear 1s;

/* @keyframes name | duration */

animation: slidein 3s;
```

## 标准化语法

```css
animation: <animation-name> || <animation-duration> || <animation-timing-function> || <animation-delay> || <animation-iteration-count> || <animation-direction> || <animation-fill-mode> || <animation-play-state>
```

## 注意事项

-   按顺序填写。`name`和`duration`必须要有，别的都可以省略。


-   animation 的参数之间只能有**一个空格**，否则会出现参数解析错误。


-   timing-function 内部不能有空格，如`cubic-bezier(1, 0.4, 0.7, 0.9)`为不当写法，`cubic-bezier(1,0.4,0.7,0.9)`为正确写法。


-   参数按顺序填写。


-   可以根据需求省略掉不需要的属性。
