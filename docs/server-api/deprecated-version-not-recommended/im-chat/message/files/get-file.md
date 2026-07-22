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

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/open-file/v1/get?file_key=file_36r377cb-c6h2-4b6d-ag67-0ac3e796008g |
| HTTP Method | GET |


### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant-desc">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


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

