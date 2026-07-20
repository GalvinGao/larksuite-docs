---
document_id: '6967261316286709765'
directory_id: '6907567266537668609'
title: 获取文件
full_path: /ukTMukTMukTM/uMDN4UjLzQDO14yM0gTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Files
- Get File
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:16Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMDN4UjLzQDO14yM0gTN
---

# 获取文件

根据文件的 file_key 拉取文件内容，当前仅可用来获取用户与机器人单聊发送的文件


:::html
<md-alert type="warn">
需要启用机器人能力；机器人只能获取自己上传的文件
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
      <md-td>https://open.larksuite.com/open-apis/open-file/v1/get?file_key=file_36r377cb-c6h2-4b6d-ag67-0ac3e796008g</md-td>
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
file_key | string| 必填 | 文件的key ||file_36r377cb-c6h2-4b6d-ag67-0ac3e796008g|
<br>
<br>
<br>



## 响应
### 响应体
返回文件的二进制数据流
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

