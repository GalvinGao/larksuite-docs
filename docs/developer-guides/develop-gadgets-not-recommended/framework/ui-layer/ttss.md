---
document_id: '6965379543683645446'
directory_id: '6907567266536669185'
title: TTSS
full_path: /uYjL24iN/uYDOuYDOuYDO
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- TTSS
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYDOuYDOuYDO
---

# TTSS

TTSS 是一套样式语言，用于描述 TTML 的组件样式。

为了适应广大的前端开发者，TTSS 具有 CSS 大部分特性。同时 TTSS 对 CSS 进行了扩充以及修改。

与 CSS 相比，TTSS 扩展的特性有：

* 尺寸单位
* 样式导入

## 尺寸单位

rpx（responsive pixel）: 可以根据屏幕宽度进行自适应。规定屏幕宽为 750rpx。如在 iPhone6 上，屏幕宽度为 375px，共有 750 个物理像素，则 750rpx = 375px = 750 物理像素，1rpx = 0.5px = 1 物理像素。

|设备|rpx 换算 px (屏幕宽度 / 750)| px 换算rpx (750 / 屏幕宽度)|备注
|-----|---|---|---|
|iPhone5|1rpx = 0.42px|1px = 2.34rpx|
|iPhone6|1rpx = 0.5px|1px = 2rpx|
|iPhone6 Plus|1rpx = 0.552px|1px = 1.81rpx|
|iPad|1rpx = 0.5px|1px = 2rpx|iPad 版本从 3.29 开始支持，屏幕宽度处理为固定的 375px

::: note
设计师可以用 iPhone6 作为视觉稿的标准。
:::

## 样式导入

使用 `@import` 语句可以导入外联样式表，`@import` 后跟需要导入的外联样式表的相对路径，用 `;` 表示语句结束。

### 示例代码：

```css
/** common.ttss **/
.small-p {
  padding:5px;
}
```

```css
/** app.ttss **/
@import "common.ttss";
.middle-p {
  padding:15px;
}
```

## 内联样式

框架组件上支持使用 style、class 属性来控制组件的样式。

* style：静态的样式统一写到 class 中。style 接收动态的样式，在运行时会进行解析，请尽量避免将静态的样式写进 style 中，以免影响渲染速度。

```html
<view style="color:{{color}};" />
```

* class：用于指定样式规则，其属性值是样式规则中类选择器名(样式类名)的集合，样式类名不需要带上.，样式类名之间用空格分隔。

```html
<view class="normal_view" />
```

## 选择器

目前支持的选择器有：

|选择器|样例|样例描述|
|-----|---|---|
|.class|.intro|选择所有拥有 class="intro" 的组件|
|#id|#firstname|选择拥有 id="firstname" 的组件|
|element|view|选择所有 view 组件|
|element, element|view, checkbox|选择所有文档的 view 组件和所有的 checkbox 组件|
|::after|view::after|在 view 组件后边插入内容|
|::before|view::before|在 view 组件前边插入内容|


## 全局样式与局部样式

定义在 app.ttss 中的样式为全局样式，作用于每一个页面。在 page 的 ttss 文件中定义的样式为局部样式，只作用在对应的页面，并会覆盖 app.ttss 中相同的选择器。
