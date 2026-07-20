---
document_id: '6967261389551353861'
directory_id: '6907567266537472001'
title: 获取图片
full_path: /ukTMukTMukTM/uYzN5QjL2cTO04iN3kDN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Images
- Get Image
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:15Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uYzN5QjL2cTO04iN3kDN
---

# 获取图片

根据图片的image_key获取图片内容


:::html
<md-alert type="warn">
需要启用机器人能力；机器人只能获取自己上传的图片。
</md-alert>
:::


## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/image/v4/get?image_key=24383920-9321-4ecd-8b33-bf8ce74e84c8</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 18%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
       <md-th style="width: 15%;">必填</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant-desc">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)
</md-td>
</md-tr>
     <md-tr>
      <md-td>Content-Type</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

### 查询参数
参数|类型|必填/选填|说明|默认值|实例  
--|--|--|--|--|--  
image_key | string| 必填 | 图片的key | |img_fdeb9536-8ec4-485f-988b-6ebd338ad47g|

## 响应
### 响应体
返回图片的二进制数据流
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
