---
document_id: '7426640831766659078'
directory_id: '7182696786035851270'
title: 侧滑关闭应用
full_path: /uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/slide-to-close-apps
breadcrumb:
- Developer Guides
- Develop Web Apps
- Open Ability
- Swipe to close the app
document_type: GuideDocumentType
updated_at: 2024-10-21T03:47:10Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/slide-to-close-apps
---

# 侧滑关闭应用

开启该功能后，使用屏幕边缘右滑的交互手势或点击左侧导航返回按钮时，直接关闭网页应用，而不是在会话历史记录中返回上一页。

## 支持说明

当前能力在不同Lark客户端的版本支持情况。

| 是否需要鉴权 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 不需要 | <md-version>V5.31</md-version> | <md-version>V5.31</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 使用方式

侧滑关闭应用的能力基于 web-meta 实现，参数配置如下表所示。

:::note
- 使用前，请先了解 [web-meta](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/web-meta) 能力和使用方式。

- 如果不通过 web-meta 属性设置侧滑关闭应用，则默认情况下，侧滑会返回会话历史记录中的上一级页面，若没有上一页，则关闭网页应用。
:::

| 名称 | 数据类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| name | String | 是 | 固定值：slideToClose |
| content | String | 是 | 期望设定的返回效果。<br>**可选值**：<br>- `true`: 关闭应用。<br>- `false`: 返回上一级。 |



## 示例配置

基于 web-meta 能力的不同定义方式，示例配置如下。

### 示例一：通过 \<meta\> 标签定义 page-meta

该配置方式仅针对当前页面生效，其中引入的 JSSDK 最新版本信息可参见[开发网页应用简介](/document/uYjL24iN/uMTMuMTMuMTM/introduction)。

```html
<html>
  <head>
    <!-- 在 head 中引入 -->
    <meta name="slideToClose" content="true" lk-config>
    
    <!-- 在 head 中 meta 标签之后引入 jssdk -->
    <script type="text/javascript" src="https://lf1-cdn-tos.bytegoofy.com/goofy/lark/op/h5-js-sdk-1.5.23.js"></script>
  </head>
</html>
```

### 示例二：通过 URL 参数定义 page-meta

该配置方式仅针对当前页面生效。

```
https://larksuite.com/?lk_meta=%7B%22page-meta%22%3A%7B%22slideToClose%22%3A%22true%22%7D%7D
```

### 示例三：通过 URL 参数定义 view-meta

该配置方式针对整个应用生效。

```
https://larksuite.com/?lk_meta=%7B%22view-meta%22%3A%7B%22slideToClose%22%3A%22true%22%7D%7D
```

:::note
针对当前页面生效的方式，需要在次级页面设置侧滑关闭应用，而不是在主页设置。如果会话历史记录不存在上一页，则侧滑或点击返回按钮时，均退出应用。
:::
