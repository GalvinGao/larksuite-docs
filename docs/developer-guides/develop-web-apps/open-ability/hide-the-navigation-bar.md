---
document_id: '7428070294323085318'
directory_id: '7182696786035851270'
title: 隐藏导航栏
full_path: /uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hide-the-navigation-bar
breadcrumb:
- Developer Guides
- Develop Web Apps
- Open Ability
- Hide the navigation bar
document_type: GuideDocumentType
updated_at: 2024-10-21T03:47:14Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hide-the-navigation-bar
---

# 隐藏导航栏

该能力用于控制是否在页面顶部隐藏导航栏，顶部状态栏不在控制范围内，遵循系统默认表现，iOS状态栏与内容视图重叠，Android状态栏在内容视图上方。



## 支持说明

当前能力在不同Lark客户端的版本支持情况。

| 是否需要鉴权 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 不需要 | <md-version>V6.3</md-version> | <md-version>V6.3</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 使用方式

是否隐藏导航栏的功能基于 web-meta 实现，参数配置如下表所示。

:::note
- 使用前，请先了解 [web-meta](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/web-meta) 能力与使用方式。

- 如果不通过 web-meta 配置该功能，则默认显示导航栏。

:::

| 名称 | 数据类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| name | String | 是 | 固定值：showNavBar |
| content | String | 是 | 期望设定的导航栏效果。<br>**可选值**：<br>- `true`: 显示导航栏。<br>- `false`: 隐藏导航栏。 |



## 示例配置

### 示例一：通过 \<meta\> 标签定义 page-meta

该配置方式仅针对当前页面生效，其中引入的 JSSDK 最新版本信息可参见[开发网页应用简介](/document/uYjL24iN/uMTMuMTMuMTM/introduction)。

```html
<html>
  <head>
    <!-- 在 head 中引入 -->
    <meta name="showNavBar" content="false" lk-config>
    
    <!-- 在 head 中 meta 标签之后引入 jssdk -->
    <script type="text/javascript" src="https://lf1-cdn-tos.bytegoofy.com/goofy/lark/op/h5-js-sdk-1.5.23.js"></script>
  </head>
</html>
```

### 示例二：通过 URL 参数定义 page-meta

该配置方式仅针对当前页面生效。

```
https://larksuite.com/?lk_meta=%7B%22page-meta%22%3A%7B%22showNavBar%22%3A%22false%22%7D%7D
```

### 示例三：通过 URL 参数定义 view-meta

该配置方式针对整个应用生效。

```
https://larksuite.com/?lk_meta=%7B%22view-meta%22%3A%7B%22showNavBar%22%3A%22false%22%7D%7D
```
