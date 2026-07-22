---
document_id: '6967331158354903046'
directory_id: '7072190414392197126'
title: 查询导入结果
full_path: /ukTMukTMukTM/uETO2YjLxkjN24SM5YjN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- sheets
- Sheet Operation
- Query Import Results
document_type: GuideDocumentType
updated_at: 2022-03-11T12:24:27Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uETO2YjLxkjN24SM5YjN
---

# 查询导入结果
该接口用于查询文件导入结果。查询30分钟无结果为导入失败。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/sheets/v2/import/result |
| HTTP Method | GET |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag> 或 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 查询参数    

| 参数          | 类型   | 必须 | 说明                                                         | 
| ------------- | ------ | ---- | ------------------------------------------------------------ | 
| ticket        | string | 是   | 导入时获取的凭证                                             | 
### cURL 请求示例
``` 
curl --location --request GET 'https://open.larksuite.com/open-apis/sheets/v2/import/result?ticket=6948310613016144800' \
--header 'Authorization: Bearer t-e346617a4acfc3a11d4ed24dca0d0c0fc8e0067e' \
```

## 响应  

### 响应体
| 参数        |类型| 说明               |
| ----------- |-----| ------------------ |
| ticket      |string| 导入时获取的凭证   |
| url         |string| 导入文档的链接     |
| warningCode |int| 导入的文档的状态码 |

warningCode的信息是以二进制数位存储的，每一位对应的含义如下：

| 5            | 4            | 3              | 2            | 1          | 0      |
| ------------ | ------------ | -------------- | ------------ | ---------- | ------ |
| 文件夹不存在 | 图片导入失败 | 云空间图片超限 | 图片超过限制 | 格子被截断 | 列截断 |

以warningCode=18为例，18（十进制）=10010（二进制） ，对应的信息是「格子被截断」、「图片导入失败」

| 0            | 1            | 0              | 0            | 1          | 0      |
| ------------ | ------------ | -------------- | ------------ | ---------- | ------ |
| 文件夹不存在 | 图片导入失败 | 云空间图片超限 | 图片超过限制 | 格子被截断 | 列截断 |

### 响应体示例    

```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "ticket": "6891610404246520328",
        "url": "https://bytedance.feishu.cn/sheets/shtcnaryaxxxx",
        "warningCode": 32
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
