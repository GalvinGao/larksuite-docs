---
document_id: '6967331173081956357'
directory_id: '7148629329634639878'
title: 文档搜索
full_path: /ukTMukTMukTM/ugDM4UjL4ADO14COwgTN
breadcrumb:
- Server API
- Docs
- Space
- Search
- Document Search
document_type: GuideDocumentType
updated_at: 2022-10-08T09:38:10Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN
---

# 搜索文档

该接口用于根据搜索条件进行文档搜索。


## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/suite/docs-api/search/object |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="drive:drive" desc="查看、评论、编辑和管理云文档所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云文档所有文件</md-perm><br><md-perm name="drive:drive:readonly" desc="查看、评论和下载云文档所有文件" support_app_types="custom,isv" tags="">查看、评论和下载云文档所有文件</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |

### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|search_key|string|是|搜索关键字 |
|count|int|否|搜索返回数量，0 <= count <= 50|
|offset|int|否|搜索偏移位，offset >= 0，offset + count < 200|
|owner_ids|list<string>|否|文档所有者的userid|
|chat_ids|list<string>|否|文档所在群的chatid|
|docs_types|list<string>|否|文档类型，支持："doc", "sheet", "slide", "bitable", "mindnote", "file" |
  
### 请求体示例
```json
{
    "search_key": "search key",
    "count": 10, 
    "offset": 0,
    "owner_ids": ["xxx", "xxx"],
    "chat_ids": ["xxx", "xxx"],
    "docs_types": ["doc", "sheet"]
}
```
## 响应
### 响应体
  |参数|说明|
|--|--|
|docs_entities|搜索匹配文档列表|
|&ensp;∟docs_token|文档token|
|&ensp;∟docs_type|文档类型|
|&ensp;∟title|标题|
|&ensp;∟owner_id|文件所有者|
|has_more|搜索偏移位结果列表后是否还有数据|
|total|搜索匹配文档总数量|
### 响应体示例

```json
{
    "code": 0,
    "msg": "Success",
    "data": {
        "docs_entities": [
            {
                "docs_token": "xxx",
                "docs_type": "doc",
                "title": "xxx",
                "owner_id": "xxx"
            },
            {
                "docs_token": "xxx",
                "docs_type": "sheet",
                "title": "xxx",
                "owner_id": "xxx"
            }
        ],
        "has_more": false,
        "total": 10
    }
}
```
### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

