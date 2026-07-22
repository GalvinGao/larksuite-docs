---
document_id: '6967331173081743365'
directory_id: '7312653929568059398'
title: 获取元数据
full_path: /ukTMukTMukTM/uMjN3UjLzYzN14yM2cTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Drive
- File
- Obtain Metadata
document_type: GuideDocumentType
updated_at: 2023-12-25T07:13:14Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMjN3UjLzYzN14yM2cTN
---

# 获取元数据


该接口用于根据 token 获取各类文件的元数据。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=suite&version=v1&resource=docs_api&method=meta)

:::note
请求用户需要拥有该文件的访问（读）权限
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/suite/docs-api/meta |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm><br><md-perm name="drive:drive.metadata:readonly" desc="查看云空间中文件元数据" support_app_types="custom,isv" tags="">查看云空间中文件元数据</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag> 或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|request_docs||是|请求文档，一次不超过200个|
|&ensp;∟docs_token|string|是|文件的 token，获取方式见[概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)|
|&ensp;∟docs_type|string|是|文件类型  <br>1) "doc": Lark文档<br>2) "sheet": Lark电子表格 <br>3) "bitable": Lark多维表格<br>4) "mindnote": Lark思维笔记 <br>5) "file": Lark文件|
### 请求体示例
```json
{
    "request_docs": [
        {
            "docs_token": "12345",
            "docs_type": "doc"
        },  
        {
            "docs_token": "12345",
            "docs_type": "sheet"
        }
    ]
}
```

## 响应
### 响应体
|参数|说明|
|--|--|
|docs_metas|文件元数据|
|&ensp;∟docs_token|文件token|
|&ensp;∟docs_type|文件类型|
|&ensp;∟title|标题|
|&ensp;∟owner_id|文件拥有者|
|&ensp;∟create_time|创建时间（Unix时间戳）|
|&ensp;∟latest_modify_user|最后编辑者|
|&ensp;∟latest_modify_time|最后编辑时间（Unix时间戳）|

### 响应体示例
```json
{
    "code": 0, 
    "msg": "Success",
    "data": { 
        "docs_metas": [ { 
                "docs_token": "doc22222",
                "docs_type": "doc",
                "title": "abc", 
                "owner_id": "12345", 
                "create_time": 123456, 
                "latest_modify_user": "12345", 
                "latest_modify_time": 123456
            }
        ]
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
