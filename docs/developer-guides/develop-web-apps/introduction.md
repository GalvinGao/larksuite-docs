---
document_id: '7199617266244419590'
directory_id: '6907567266536554497'
title: 简介
full_path: /uYjL24iN/uMTMuMTMuMTM/introduction
breadcrumb:
- Developer Guides
- Develop Web Apps
- Introduction
document_type: GuideDocumentType
updated_at: 2025-09-24T08:41:24Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/introduction
---

# 简介


Lark客户端网页（Web）应用指的是用 H5 方式开发，可以运行在Lark客户端内的应用。网页应用自身提供了开放能力，并且还可以调用丰富的Lark客户端开放接口（JSAPI），这些开放接口包含了手机系统功能和通讯录、云文档等Lark客户端功能，同时兼具到了客户端侧的性能优化，可以使你的网页应用接近原生体验。

本文介绍网页应用的开发流程，以及网页应用支持的开放能力和开放接口。

## 开发流程

开发Lark开放平台的网页应用主要分为以下步骤，详情可参见[网页应用开发指南概述](/document/uYjL24iN/uITO4IjLykDOy4iM5gjM)。

1. 创建并配置企业自建应用。
2. （可选）鉴权调用 JSAPI。
3. （可选）配置应用的免登流程。
4. 发布并使用应用。

:::note
网页应用开发指南主要介绍企业自建应用的开发流程。如果你需要开发商店应用，可参见[商店应用发布指南](/document/uMzNwEjLzcDMx4yM3ATM/ugzNwEjL4cDMx4CO3ATM)。
:::

## 开放能力

网页应用提供了以下容器开放能力。

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:30%">开放能力</md-th>
<md-th style="width:70%">说明</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>[web-meta](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/web-meta)</md-td>
<md-td>web-meta 是网页的元数据，用于定义网页特性。在Lark中运行的网页支持通过 web-meta 定制网页容器的表现，即使该网页不是一个网页应用。</md-td>
</md-tr>
  
<md-tr>
<md-td>[设置屏幕方向](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/orientation)</md-td>
<md-td>该能力用于设定网页在屏幕展示的方式，支持强制横屏、强制竖屏、跟随系统设置。</md-td>
</md-tr>
  

<md-tr>
<md-td>[配置更多菜单的功能](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hidemoremenuitems)</md-td>
<md-td>在网页更多菜单面板中，支持配置部分功能入口的启停状态，即是否置灰功能按钮。例如，屏蔽更多菜单转发及复制链接功能。</md-td>
</md-tr>
  
<md-tr>
<md-td>[侧滑关闭应用](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/slide-to-close-apps)</md-td>
<md-td>开启该功能后，使用屏幕边缘右滑的交互手势或点击左侧导航返回按钮时，直接关闭网页应用，而不是在会话历史记录中返回上一页。</md-td>
</md-tr>

<md-tr>
<md-td>[隐藏导航栏](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hide-the-navigation-bar)</md-td>
<md-td>该能力用于控制是否在页面顶部隐藏导航栏。</md-td>
</md-tr>
  
<md-tr>
<md-td>[设置导航栏颜色](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/set-navigation-bar-color)</md-td>
<md-td>该能力通过以下参数设置导航栏的颜色。

- 通过 navFgColor 参数设置导航栏前景颜色。

- 通过 navBgColor 参数设置导航栏背景颜色。</md-td>
</md-tr>

<md-tr>
<md-td>[隐藏导航栏按钮](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hide-the-navigation-bar-button)</md-td>
<md-td>该能力支持隐藏页面顶部左右两侧的导航栏按钮。

- showNavLBarBtn 参数用于隐藏左侧导航按钮。

- showNavRBarBtn 参数用于隐藏右侧导航按钮。</md-td>
</md-tr>
  
</md-tbody>
</md-table>
:::

## 开放接口（H5 JSAPI）

开放平台提供了网页应用可以调用的 [H5 JSAPI ](/document/uYjL24iN/uMTMuMTMuMTM/)。调用 JSAPI 依赖开放平台提供的工具包 JSSDK，使用时只需要在调用 JSAPI 的页面引入 JS 文件即可，引入方式如下代码所示。

```js
<script
type= "text/javascript"
src= "https://lf-package-sg.larksuitecdn.com/obj/lark-static-sgsaas/lark/op/h5-js-sdk-1.5.44.js"
></script> 
```

:::warning
**注意事项**：

- 在升级功能时，JS 文件的版本会作相应更新。上方代码中已经是最新的 JSSDK 版本对应的 src。JSSDK 版本号格式为`Major.Minor.Patch`（主版本号.次版本号.修订版本号），例如`https://lf-package-sg.larksuitecdn.com/obj/lark-static-sgsaas/lark/op/h5-js-sdk-1.5.44.js`中的 JSSDK 版本号为`1.5.44`。

- 如有需要（例如，使用新增的 JSAPI），请参考上方代码核对 JSSDK 链接，确保你当前使用的 JSSDK 版本是最新版本。
:::
