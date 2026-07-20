---
document_id: '7426640831766560774'
directory_id: '7182696786035851270'
title: 设置屏幕方向
full_path: /uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/orientation
breadcrumb:
- Developer Guides
- Develop Web Apps
- Open Ability
- Set screen orientation
document_type: GuideDocumentType
updated_at: 2024-10-21T03:47:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/orientation
---

# 设置屏幕方向

该能力用于设定网页在屏幕展示的方式，支持强制横屏、强制竖屏、跟随系统设置。

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
      <md-td><md-version>V5.15</md-version></md-td>
      <md-td><md-version>V5.15</md-version></md-td>
      <md-td>**X**</md-td>
     <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>  
</md-tbody>
</md-table>
:::

## 注意事项

- 设置屏幕方向的能力不支持 Pad 设备。

- 该能力与网页内横竖屏切换按钮（即锁定屏幕后旋转设备方向出现的按钮）功能无关。

## 使用方式

设置屏幕方向的配置基于 web-meta 能力实现，参数配置如下表所示。

:::note
- 使用前，请先了解 [web-meta](/document/uYjL24iN/uMTMuMTMuMTM/web-app-open-ability/web-meta) 的能力与使用方式。

- 如果不通过 web-meta 属性设定屏幕方向，则应用在默认情况下跟随系统设置。例如：

	- 如果手机设备设置为自动旋转，则页面可自动旋转。

	- 如果手机设备设置为锁定屏幕，则页面为竖屏。
:::

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">
                参数名称
            </md-th>
            <md-th style="width: 15%;">
                数据类型
            </md-th>
            <md-th style="width: 15%;">
                是否必填
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
                固定取值：orientation
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
                期望设定的屏幕方向。

可选值：
              
- `default`：和系统展示方式保持一致。
              
- `portrait`：强制为竖屏。
              
- `landscape`：强制为横屏。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例配置

基于 web-mete 能力的不同定义方式，设置屏幕方向的示例配置如下。


### 示例一：通过 \<meta\> 标签定义 page-meta

该配置方式仅针对当前页面生效，该配置方式仅针对当前页面生效，其中引入的 JSSDK 最新版本信息可参见[开发网页应用简介](/document/uYjL24iN/uMTMuMTMuMTM/introduction)。

```html
<html>
  <head>
    <!-- 在 head 中引入 -->
    <meta name="orientation" content="landscape" lk-config>
    
    <!-- 在 head 中 meta 标签之后引入 jssdk -->
    <script type="text/javascript" src="https://lf1-cdn-tos.bytegoofy.com/goofy/lark/op/h5-js-sdk-1.5.23.js"></script>
  </head>
</html>
```

### 示例二：通过 URL 参数定义 page-meta

该配置方式仅针对当前页面生效。

```
https://larksuite.com/?lk_meta=%7B%22page-meta%22%3A%7B%22orientation%22%3A%22landscape%22%7D%7D
```

### 示例三：通过 URL 参数定义 view-meta

该配置方式针对整个应用生效。

```
https://larksuite.com/?lk_meta=%7B%22view-meta%22%3A%7B%22orientation%22%3A%22landscape%22%7D%7D
```
