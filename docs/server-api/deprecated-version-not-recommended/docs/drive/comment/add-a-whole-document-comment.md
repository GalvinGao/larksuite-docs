---
document_id: '7073817463670538246'
directory_id: '7072190414392262662'
title: 添加全文评论
full_path: /ukTMukTMukTM/ucDN4UjL3QDO14yN0gTN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Drive
- Comment
- Add a Whole Document Comment
document_type: GuideDocumentType
updated_at: 2022-03-11T12:24:30Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ucDN4UjL3QDO14yN0gTN
---

# 添加全文评论

该接口用于根据 filetoken 给文档添加全文评论

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/comment/add_whole |
| HTTP Method | POST |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
|参数|类型|必须|说明|
|--|-----|--|----|
|token|string|是|文件的 token，获取方式见 [对接前说明](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)的第 4 项|
|type|string|是|文档类型  "doc"|
|content|string|是|评论内容|

**转义字符表**: 

|原字符|转义字符|
|--|--|
|<|\&lt;|
|>|\&gt;|
|&|\&amp;|
|'|\&#x27;|
|"|\&quot;|

### 请求体示例
```json
{
	"type": "doc",
	"token": "doccnBKgoMyY5OMbUG6FioTXuBe",
	"content": ""
}
```

## 响应

### 响应体

|参数|说明|
|--|--|
|comment_id|评论id|
|reply_id|回复id|
|create_timestamp|创建时间戳|
|update_timestamp|更新时间戳|

### 响应体示例

```json
{
	"code": 0,
	"data": {
		"comment_id": "1575537199207062072",
		"reply_id": "1575537230887731780",
		"create_timestamp": 1575537230,
		"update_timestamp": 1575537230
	},
	"msg": "Success"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
