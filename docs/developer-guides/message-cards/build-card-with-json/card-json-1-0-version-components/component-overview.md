---
document_id: '7395083758943649797'
directory_id: '7393222083608576006'
title: 组件概述
full_path: /uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/component-overview
breadcrumb:
- Developer Guides
- Message cards
- Build card with JSON
- Card JSON 1.0 version components
- Component overview
document_type: GuideDocumentType
updated_at: 2024-08-30T10:00:35Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/component-overview
---

# 组件概述

本章节描述支持通过卡片 JSON 构建的组件。卡片 JSON 通过定义 `tag` 字段声明组件：

```json
{
  "tag": "" // 在此声明组件的标签。不同的组件标签不同。
}
```

## 容器类组件

容器类组件可用于布局内容或配置交互逻辑。在容器组件中支持添加展示类组件和交互类组件。

| 组件 | 客户端版本要求 | 描述 |
| --- | --- | --- |
| [分栏（column_set）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/column-set) | / | 分栏支持横向排布多列，在列内自由组合图文内容，搭建出如数据表、商品或文章列表、差旅信息等图文并茂、交互友好的卡片。 |
| [表单容器（form）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/form-container) | Lark V6.6 及以上 | 表单容器支持用户在前端本地录入一批表单项后，通过点击一次 提交 按钮，将这一批本地缓存的表单内容一次回调至开发者的服务端，实现异步提交多个表单项数据的效果。 |
| [交互容器（interactive_container）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/interactive-container) | Lark V7.4 及以上 | 交互容器允许你基于业务需求在交互容器中内嵌组件，并灵活组合多个交互容器，并统一定义多个交互容器的样式、交互能力等，实现多种组合效果和丰富的卡片交互。 |
| [折叠面板（collapsible_panel）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/collapsible-panel) | Lark V7.9 及以上 | 折叠面板允许你在卡片中折叠次要信息，如备注、较长文本等，以突出主要信息。 |


## 展示类组件

展示类组件用于构成卡片的主要内容，不具备交互能力。

| 组件 | 客户端版本要求 | 描述 |
| --- | --- | --- |
| [标题（header）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/title) | / | 标题组件用于构建Lark卡片的标题样式与内容，支持添加卡片主标题、副标题、后缀标签和标题图标。 |
| [普通文本（div）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/plain-text) | / | 普通文本组件支持添加普通文本和前缀图标，并设置文本大小、颜色、对齐方式等展示样式。 |
| [富文本（markdown）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/rich-text) | / | 富文本（Markdown）组件支持渲染文本、图片、分割线等元素。 |
| [图片（img）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/image) | / | 图片组件支持通过调用[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口或在新版Lark卡片搭建工具中上传图片，在卡片内添加图片。 |
| [多图混排（img_combination）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/multi-image-laylout) | / | 多图混排组件支持通过调用[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口或在新版Lark卡片搭建工具中上传图片，在卡片内添加多张图片。 |
| [人员（person）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/user-profile) | / | 人员组件支持展示人员的用户名和头像。你可通过传入人员的 open_id、user_id 或 union_id 使用该组件。 |
| [人员列表（person_list）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/user-list) | / | 人员列表组件支持展示多个人员的用户名和头像。你可通过传入人员的 open_id、user_id 或 union_id 使用该组件。 |
| [图表（chart）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/chart) | Lark V7.1 及以上 | 图表组件基于 [VChart](https://www.visactor.io/) 的图表定义，支持折线图、面积图、柱状图、饼图、词云等多种数据呈现方式，帮助你可视化各类信息，提高信息沟通效率。 |
| [表格（table）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/table) | Lark V7.4 及以上 | 表格组件支持在表格中添加普通文本、选项标签、人员列表以及数字格式的内容。 |
| [分割线（hr）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/divider) | / | 分割线组件是一条长横线，用于分割卡片的内容，使呈现内容更清晰。 |


## 交互类组件

交互类组件为卡片提供了交互能力。用户在接收到包含交互组件的卡片时，可直接在卡片内访问链接或处理业务。

| 组件 | 客户端版本要求 | 描述 |
| --- | --- | --- |
| [输入框（input）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/input) | Lark V6.8 及以上 | 输入框组件支持收集不固定的文本内容，如原因、评价、备注等。 |
| [按钮（button）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/button) | / | 按钮组件提供配置按钮的回传交互能力或者链接跳转能力，并支持多种样式和尺寸。 |
| [折叠按钮组（overflow）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/overflow) | / | 折叠按钮组组件支持将多个按钮添加在折叠按钮组中，默认情况下按钮组为折叠状态，点击按钮组将会展示组内所有按钮。 |
| [下拉选择-单选（select_static）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/single-select-dropdown-menu) | / | 下拉选择-单选组件支持自定义单选菜单的选项文本、图标和回传参数。 |
| [下拉选择-多选（multi_select_static）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/multi-select-dropdown-menu) | Lark V7.4 及以上 | 下拉选择-多选组件支持自定义多选菜单的选项文本、图标和回传参数。 |
| [人员选择-单选（select_person）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/single-select-user-picker) | / | 人员选择-多选组件支持添加指定人员作为单选选项。 |
| [人员选择-多选（multi_select_person）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/multi-select-user-picker) | Lark V7.4 及以上 | 人员选择-多选组件支持添加指定人员作为多选选项。 |
| [日期选择器（date_picker）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/date-picker) | / | 日期选择器组件支持提供日期选项。 |
| [时间选择器（picker_time）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/time-selector) | / | 时间选择器组件支持提供时间选项。 |
| [日期时间选择器（picker_datetime）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/date-time-picker) | / | 日期时间选择器组件支持提供时间和日期选项。 |
| [多图选择（select_img）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/image-picker) | Lark V7.6 及以上 | 多图选择组件支持提供图片选项，支持单选、多选图片。 |
| [勾选器（checker）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/checker) | Lark V7.9 及以上 | 勾选器支持配置回调响应，主要用于任务勾选的场景。 |

