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

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 25%;">组件</md-th>

      <md-th style="width: 20%;">客户端版本要求</md-th>
      <md-th style="width: 40%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>
        [分栏（column_set）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/column-set)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        分栏支持横向排布多列，在列内自由组合图文内容，搭建出如数据表、商品或文章列表、差旅信息等图文并茂、交互友好的卡片。
      </md-td>
    </md-tr>
    
    <md-tr>
      <md-td>
        [表单容器（form）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/form-container)
      </md-td>
     
      <md-td>Lark V6.6 及以上</md-td>
      <md-td>
        表单容器支持用户在前端本地录入一批表单项后，通过点击一次 提交 按钮，将这一批本地缓存的表单内容一次回调至开发者的服务端，实现异步提交多个表单项数据的效果。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [交互容器（interactive_container）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/interactive-container)
      </md-td>
     
      <md-td>Lark V7.4 及以上</md-td>
      <md-td>
        交互容器允许你基于业务需求在交互容器中内嵌组件，并灵活组合多个交互容器，并统一定义多个交互容器的样式、交互能力等，实现多种组合效果和丰富的卡片交互。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [折叠面板（collapsible_panel）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/containers/collapsible-panel)
      </md-td>
     
      <md-td>Lark V7.9 及以上</md-td>
      <md-td>
        折叠面板允许你在卡片中折叠次要信息，如备注、较长文本等，以突出主要信息。
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>

:::

## 展示类组件

展示类组件用于构成卡片的主要内容，不具备交互能力。
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">组件</md-th>

      <md-th style="width: 20%;">客户端版本要求</md-th>
      <md-th style="width: 40%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>
        [标题（header）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/title)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        标题组件用于构建Lark卡片的标题样式与内容，支持添加卡片主标题、副标题、后缀标签和标题图标。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [普通文本（div）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/plain-text)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        普通文本组件支持添加普通文本和前缀图标，并设置文本大小、颜色、对齐方式等展示样式。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [富文本（markdown）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/rich-text)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        富文本（Markdown）组件支持渲染文本、图片、分割线等元素。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [图片（img）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/image)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        图片组件支持通过调用[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口或在新版Lark卡片搭建工具中上传图片，在卡片内添加图片。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [多图混排（img_combination）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/multi-image-laylout)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        多图混排组件支持通过调用[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口或在新版Lark卡片搭建工具中上传图片，在卡片内添加多张图片。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [人员（person）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/user-profile)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        人员组件支持展示人员的用户名和头像。你可通过传入人员的 open_id、user_id 或 union_id 使用该组件。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [人员列表（person_list）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/user-list)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        人员列表组件支持展示多个人员的用户名和头像。你可通过传入人员的 open_id、user_id 或 union_id 使用该组件。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [图表（chart）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/chart)
      </md-td>
     
      <md-td>Lark V7.1 及以上</md-td>
      <md-td>
        图表组件基于 [VChart](https://www.visactor.io/) 的图表定义，支持折线图、面积图、柱状图、饼图、词云等多种数据呈现方式，帮助你可视化各类信息，提高信息沟通效率。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
      [表格（table）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/table)
      </md-td>
     
      <md-td>Lark V7.4 及以上</md-td>
      <md-td>
        表格组件支持在表格中添加普通文本、选项标签、人员列表以及数字格式的内容。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [分割线（hr）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/divider)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        分割线组件是一条长横线，用于分割卡片的内容，使呈现内容更清晰。
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

## 交互类组件

交互类组件为卡片提供了交互能力。用户在接收到包含交互组件的卡片时，可直接在卡片内访问链接或处理业务。

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">组件</md-th>
 
      <md-th style="width: 20%;">客户端版本要求</md-th>
      <md-th style="width: 40%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>
        [输入框（input）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/input)
      </md-td>
     
      <md-td>Lark V6.8 及以上</md-td>
      <md-td>
        输入框组件支持收集不固定的文本内容，如原因、评价、备注等。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [按钮（button）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/button)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        按钮组件提供配置按钮的回传交互能力或者链接跳转能力，并支持多种样式和尺寸。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [折叠按钮组（overflow）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/overflow)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        折叠按钮组组件支持将多个按钮添加在折叠按钮组中，默认情况下按钮组为折叠状态，点击按钮组将会展示组内所有按钮。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [下拉选择-单选（select_static）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/single-select-dropdown-menu)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        下拉选择-单选组件支持自定义单选菜单的选项文本、图标和回传参数。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [下拉选择-多选（multi_select_static）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/multi-select-dropdown-menu)
      </md-td>
     
      <md-td>Lark V7.4 及以上</md-td>
      <md-td>
        下拉选择-多选组件支持自定义多选菜单的选项文本、图标和回传参数。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [人员选择-单选（select_person）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/single-select-user-picker)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        人员选择-多选组件支持添加指定人员作为单选选项。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [人员选择-多选（multi_select_person）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/multi-select-user-picker)
      </md-td>
     
      <md-td>Lark V7.4 及以上</md-td>
      <md-td>
        人员选择-多选组件支持添加指定人员作为多选选项。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [日期选择器（date_picker）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/date-picker)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        日期选择器组件支持提供日期选项。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [时间选择器（picker_time）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/time-selector)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        时间选择器组件支持提供时间选项。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [日期时间选择器（picker_datetime）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/date-time-picker)
      </md-td>
     
      <md-td>/</md-td>
      <md-td>
        日期时间选择器组件支持提供时间和日期选项。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
        [多图选择（select_img）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/image-picker)
      </md-td>
     
      <md-td>Lark V7.6 及以上</md-td>
      <md-td>
        多图选择组件支持提供图片选项，支持单选、多选图片。
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>
       [勾选器（checker）](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/interactive-components/checker)
      </md-td>
     
      <md-td>Lark V7.9 及以上</md-td>
      <md-td>
        勾选器支持配置回调响应，主要用于任务勾选的场景。
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
