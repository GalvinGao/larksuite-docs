---
document_id: '7426640831766642694'
directory_id: '7182696786035851270'
title: 配置更多菜单中的功能
full_path: /uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hidemoremenuitems
breadcrumb:
- Developer Guides
- Develop Web Apps
- Open Ability
- Config more functions in the menu
document_type: GuideDocumentType
updated_at: 2024-10-21T03:47:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hidemoremenuitems
---

# 配置更多菜单中的功能

在网页更多菜单面板中，支持配置部分功能入口的启停状态，即是否置灰功能按钮。例如，屏蔽更多菜单转发及复制链接功能。



## 支持说明

当前能力在不同Lark客户端的版本支持情况。

| 是否需要鉴权 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 不需要 | <md-version>V5.27</md-version> | <md-version>V5.27</md-version> | <md-version>V5.27</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |


## 注意事项

目前仅支持页面级别的配置，每次重新加载页面都会读取最新的配置并应用。

## 使用方式

配置更多菜单中的功能基于 web-meta 能力实现，参数配置如下表所示。

:::note
- 使用前，请先了解 [web-meta](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/web-meta) 能力与使用方式。

- 如果不通过 web-meta配置该功能，则应用在默认情况下展示所有菜单项。

- 如果你希望在网页未加载完成之前就配置某些菜单不可用，请使用web-meta 的 page-meta URL 方式进行配置。 
:::

| 名称 | 数据类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| name | String | 是 | 固定值：hideMenuItems |
| content | String | 是 | 期望屏蔽菜单界面功能入口的唯一标识。详情可参见下文 **支持置灰的菜单功能项** 表。<br>格式示例："sendToChat,shareViaOtherApp"。 |


**支持置灰的菜单功能项**

| 菜单项标识 | 菜单项名称 | 菜单项功能描述 |
| --- | --- | --- |
| sendToChat | 发送至会话 | 分享当前网页到会话。 |
| shareViaOtherApp | 微信 | 分享网页链接至微信等三方应用。 |
| copyLink | 复制链接 | 复制当前页面链接到系统剪贴板。 |
| openInBrowser | 浏览器打开 | 在外部浏览器中打开当前页面。 |


## 示例配置

基于 web-meta 能力的不同定义方式，示例配置如下。

### 示例一：通过 \<meta\> 标签定义 page-meta

该配置方式仅针对当前页面生效，其中引入的 JSSDK 最新版本信息可参见[开发网页应用简介](/document/uYjL24iN/uMTMuMTMuMTM/introduction)。

```html
<html>
  <head>
    <!-- 在 head 中引入 -->
    <meta name="hideMenuItems" content="sendToChat,shareViaOtherApp" lk-config>
    
    <!-- 在 head 中 meta 标签之后引入 jssdk -->
    <script type="text/javascript" src="https://lf1-cdn-tos.bytegoofy.com/goofy/lark/op/h5-js-sdk-1.5.23.js"></script>
  </head>
</html>
```

### 示例二：通过 URL 参数定义 page-meta

该配置方式仅针对当前页面生效。

```
https://larksuite.com/?lk_meta=%7B%22page-meta%22%3A%7B%22hideMenuItems%22%3A%22sendToChat%2CcopyLink%22%7D%7D
```

:::note
该功能暂不支持通过 web-meta 的 view-meta URL 方式实现。
:::
