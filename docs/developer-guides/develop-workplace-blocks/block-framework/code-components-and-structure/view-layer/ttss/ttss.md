---
document_id: '7180269945547833349'
directory_id: '7179507279661793286'
title: TTSS
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/ttss
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- TTSS
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:38Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/ttss
---

# TTSS

TTSS 是一套样式语言，用于描述 TTML 的组件样式。

为了适应广大的前端开发者，从语法上来讲，TTSS 是 CSS 的严格子集，但是在部分特性中对其进行了扩充。
> 目前移动端 Native 渲染模式下，CSS 属性都不支持继承能力

## 已支持css列表

> 开发者书写的浏览器标准 css 属性，可能在 PC 环境下效果正常，但是在移动端Native 环境下不符合预期，因此为了确保各端效果一致，请提前参考以下已支持范围的列表

### @规则

-   [@import](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/rules/import)


-   [@font-face](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/rules/font-face)


-   [@keyframes](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/rules/keyframes)

### 选择器列表

-   [类型选择器](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/selector-list)
-   [类选择器](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/selector-list)
-   [ID 选择器](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/selector-list)

-   [通配符选择器](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/selector-list)

-   [分组选择器](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/selector-list)

-   [伪元素](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/selector-list)

### 属性

-   #### 定位

    -   [position](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/position/position)
    -   [left](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/position/left)
    -   [top](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/position/top)
    -   [right](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/position/right)
    -   [bottom](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/position/bottom)

-   #### 文本

    -   [color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/color)
    -   [font-size](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-size)
    -   [font-weight](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-weight)
    -   [font-family](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-family)
    -   [font-style](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-style)
    -   [white-space](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/white-space)
    -   [letter-spacing](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/letter-spacing)
    -   [line-height](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/line-height)
    -   [text-align](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-align)
    -   [text-overflow](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-overflow)
    -   [text-decoration](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-decoration)
    -   [text-shadow](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/text-shadow)

-   #### 变形

    -   [transform](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/transform/transform)
    -   [transform-origin](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/transform/transform-origin)

-   #### 动画

    -   [animation](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation)
    -   [animation-name](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-name)
    -   [animation-duration](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-duration)
    -   [animation-timing-function](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-timing-function)
    -   [animation-delay](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-delay)
    -   [animation-iteration-count](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-iteration-count)
    -   [animation-direction](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-direction)
    -   [animation-fill-mode](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-fill-mode)
    -   [animation-play-state](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/animation-play-state)
    -   [transition](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/transition)
    -   [transition-property](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/transition-property)
    -   [transition-duration](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/transition-duration)
    -   [transition-delay](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/transition-delay)
    -   [transition-timing-function](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/animation/transition-timing-function)

-   #### Flexbox

    -   [flex](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/flex)
    -   [flex-grow](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/flex-grow)
    -   [flex-shrink](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/flex-shrink)
    -   [flex-basis](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/flex-basis)
    -   [flex-direction](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/flex-direction)
    -   [flex-wrap](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/flex-wrap)
    -   [align-items](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/align-items)
    -   [align-self](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/align-self)
    -   [align-content](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/align-content)
    -   [justify-content](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/justify-content)
    -   [order](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/flexbox/order)

-   #### 边框

    -   [border](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border)
    -   [border-right](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-right)
    -   [border-left](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-left)
    -   [border-top](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-top)
    -   [border-bottom](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom)
    -   [border-radius](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-radius)
    -   [border-top-left-radius](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-top-left-radius)
    -   [border-bottom-left-radius](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom-left-radius)
    -   [border-top-right-radius](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-top-right-radius)
    -   [border-bottom-right-radius](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom-right-radius)
    -   [border-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-width)
    -   [border-left-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-left-width)
    -   [border-right-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-right-width)
    -   [border-top-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-top-width)
    -   [border-bottom-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom-width)
    -   [border-style](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-style)
    -   [border-left-style](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-left-style)
    -   [border-right-style](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-right-style)
    -   [border-top-style](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-top-style)
    -   [border-bottom-style](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom-style)
    -   [border-color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-color)
    -   [border-left-color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-left-color)
    -   [border-right-color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-right-color)
    -   [border-top-color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-top-color)
    -   [border-bottom-color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/border/border-bottom-color)

-   #### 背景

    -   [background](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background)
    -   [background-color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-color)
    -   [background-image](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-image)
    -   [background-position](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-position)
    -   [background-origin](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-origin)
    -   [background-repeat](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-repeat)
    -   [background-size](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-size)
    -   [background-clip](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/background/background-clip)

-   #### 盒模型

    -   [display](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/display)
    -   [box-sizing](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/box-sizing)
    -   [padding](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/padding)
    -   [padding-left](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/padding-left)
    -   [padding-right](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/padding-right)
    -   [padding-top](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/padding-top)
    -   [padding-bottom](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/padding-bottom)
    -   [margin](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/margin)
    -   [margin-left](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/margin-left)
    -   [margin-right](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/margin-right)
    -   [margin-top](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/margin-top)
    -   [margin-bottom](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/margin-bottom)
    -   [height](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/height)
    -   [width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/width)
    -   [max-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/max-width)
    -   [min-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/min-width)
    -   [max-height](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/max-height)
    -   [min-height](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/min-height)
    -   [overflow](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/overflow)
    -   [overflow-x](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/overflow-x)
    -   [overflow-y](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/overflow-y)
    -   [outline](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/outline)
    -   [outline-color](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/outline-color)
    -   [outline-style](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/outline-style)
    -   [outline-width](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/box-model/outline-width)

-   #### 其它

    -   [visibility](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/other/visibility)
    -   [opacity](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/other/opacity)
    -   [box-shadow](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/other/box-shadow)
    -   [content](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/other/content)

### 基本数据类型

-   `关键字`

-   [<angle>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/angle)

-   [<color>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/color)

-   [<number>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/number)

-   [<length>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/length)

-   [<percentage>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/percentage)

-   [<string>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/string)

-   [<time>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/time)

-   [<gradient>](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/gradient)
