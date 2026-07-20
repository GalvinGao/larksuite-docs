---
document_id: '7426640831766577158'
directory_id: '7182696786035851270'
title: 隐藏底部导航栏
full_path: /uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hide-the-bottom-navigation-bar
breadcrumb:
- Developer Guides
- Develop Web Apps
- Open Ability
- Hide the bottom navigation bar
document_type: GuideDocumentType
updated_at: 2024-10-21T03:47:27Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/hide-the-bottom-navigation-bar
---

# 隐藏底部导航栏

该能力用于控制是否隐藏页面底部的导航栏。


## 支持说明

当前能力在不同Lark客户端的版本支持情况。

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">是否需要鉴权</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>


    <md-tr>
      <md-td>不需要</md-td>
      <md-td><md-version>V7.7</md-version></md-td>
      <md-td><md-version>V7.7</md-version></md-td>
      <md-td>**X**</md-td>
     <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>  
</md-tbody>
</md-table>
:::


## 使用方式

是否隐藏底部导航栏的功能基于 web-meta 实现，参数配置如下表所示。

:::note
- 使用前，请先了解 [web-meta](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/web-meta) 能力与使用方式。

- 如果不通过 web-meta 配置该功能，则默认显示底部导航栏。

:::
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">
                名称
            </md-th>
            <md-th style="width: 15%;">
                数据类型
            </md-th>
            <md-th style="width: 15%;">
                必填
            </md-th>
            <md-th style="width: 45%;">
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                name
            </md-td>
            <md-td>
                String
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>
                固定值：showBottomNavBar
            </md-td>
        </md-tr>
    </md-tbody>
  	<md-tbody>
        <md-tr>
            <md-td>
                content
            </md-td>
            <md-td>
                String
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>
                期望设定的底部导航栏效果。

**可选值**：
- `true`: 显示底部导航栏。
              
- `false`: 隐藏底部导航栏。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例配置

### 示例一：通过 \<meta\> 标签定义 page-meta

该配置方式仅针对当前页面生效，其中引入的 JSSDK 最新版本信息可参见[开发网页应用简介](/document/uYjL24iN/uMTMuMTMuMTM/introduction)。

```html
<html>
  <head>
    <!-- 在 head 中引入 -->
    <meta name="showBottomNavBar" content="false" lk-config>
    
    <!-- 在 head 中 meta 标签之后引入 jssdk -->
    <script type="text/javascript" src="https://lf1-cdn-tos.bytegoofy.com/goofy/lark/op/h5-js-sdk-1.5.23.js"></script>
  </head>
</html>
```

### 示例二：通过 URL 参数定义 page-meta

该配置方式仅针对当前页面生效。

```
https://larksuite.com/?lk_meta=%7B%22page-meta%22%3A%7B%22showBottomNavBar%22%3A%22false%22%7D%7D
```

### 示例三：通过 URL 参数定义 view-meta

该配置方式针对整个应用生效。

```
https://larksuite.com/?lk_meta=%7B%22view-meta%22%3A%7B%22showBottomNavBar%22%3A%22false%22%7D%7D
```
