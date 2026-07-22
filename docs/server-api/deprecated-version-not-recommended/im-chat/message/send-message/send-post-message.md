---
document_id: '6967261389551403013'
directory_id: '6907567266537013249'
title: 发送富文本消息
full_path: /ukTMukTMukTM/uMDMxEjLzATMx4yMwETM
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- IM & Chat
- Message
- Send Message
- Send Post Message
document_type: GuideDocumentType
updated_at: 2021-07-13T06:38:10Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMDMxEjLzATMx4yMwETM
---

# 发送富文本消息
给指定用户或者会话发送富文本消息，其中会话包括私聊会话和群会话。

:::html
<md-alert type="warn">
需要启用机器人能力；私聊会话时机器人需要拥有对用户的可见性，群会话需要机器人在群里
</md-alert>
:::

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/message/v4/send/ |
| HTTP Method | POST |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN"> 以应用的身份发消息 </md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体
参数 | 类型 | 必须 | 说明|默认值|实例  
-- | -- | -- | -- |--|--  
open_id <br>user_id <br> email <br> chat_id  | string | 是 | 给用户发私聊消息，只需要填 open_id、email、user_id（可通过[使用手机号或邮箱获取用户 ID接口](/document/ukTMukTMukTM/uUzMyUjL1MjM14SNzITN)获取） 中的一个即可，向群里发消息使用群的 chat_id（可通过[获取群列表接口](/document/ukTMukTMukTM/uITO5QjLykTO04iM5kDN)获取）。服务端依次读取字段的顺序为 chat_id > open_id > user_id > email   ( user_id 对应 V3 接口的 employee_id , chat_id 对应 V3 的 open_chat_id )||ou_5ad573a6411d72b8305fda3a9c15c70e|
root_id | string | 否 | 如果需要回复某条消息，填对应消息的消息 ID||om_40eb06e7b84dc71c03e009ad3c754195|
msg_type | string | 是 | 消息的类型，此处固定填 “post”||post|
content | object | 是 | 消息的内容
&nbsp;&nbsp;∟post | object | 是 | 富文本消息体内容
&nbsp;&nbsp;&nbsp;&nbsp;∟zh_cn | object | 否 | 中文消息体、需要国际化可以增加ja_jp（日语）、en_us（英文）
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;∟title | string | 否 | 消息的标题
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;∟content | list |  是 | 消息的内容

### 请求体示例


```json
{
   "open_id":"ou_5ad573a6411d72b8305fda3a9c15c70e", 
   "root_id":"om_40eb06e7b84dc71c03e009ad3c754195",
   "chat_id":"oc_5ad11d72b830411d72b836c20",
   "user_id": "92e39a99",
   "email":"fanlv@gmail.com",
   "msg_type":"post",
   "content":{
        "post":{ 
         	"zh_cn":{}, // option
            "ja_jp":{}, // option
         	"en_us":{} // option
        }
   }
}
```

### Curl 请求 Demo

```curl 
curl -X POST \
  https://open.larksuite.com/open-apis/message/v4/send/ \
  -H 'Authorization: Bearer t-8b2d47b17ba55267c777840b803c866de101fbee' \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "fanlv@bytedance.com",
    "msg_type": "post",
    "content": {
        "post": {
            "zh_cn": {
                "title": "我是一个标题",
                "content": [
                    [
                        {
                            "tag": "text",
                            "un_escape": true,
                            "text": "第一行&nbsp;:"
                        },
                        {
                            "tag": "a",
                            "text": "超链接",
                            "href": "http://www.larksuite.com"
                        },
                        {
                            "tag": "at",
                            "user_id": "ou_18eac85d35a26f989317ad4f02e8bbbb"
                        }
                    ],
                    [
                        {
                            "tag": "text",
                            "text": "第二行 :"
                        },
                        {
                            "tag": "text",
                            "text": "文本测试"
                        }
                    ],
                    [
                        {
                            "tag": "img",
                            "image_key": "d640eeea-4d2f-4cb3-88d8-c964fab53987",
                            "width": 300,
                            "height": 300
                        }
                    ]
                ]
            }
        }
    }
}'
```
## 响应
### 响应体
|参数|类型|说明|
|-|-|-|
|code|int|返回码，非 0 表示失败|
|msg|string|返回码描述|
data | - | -
&emsp;∟message_id |string| 消息 ID
### 响应体示例
```json
{
    "code": 0,
    "msg": "ok",
    "data":{
       "message_id": "om_92eb70a7120ga8c3ca56a12ee9ba7ca2"
    }
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)


## 请求体实例

```json
{
    "email": "fanlv@bytedance.com",
    "msg_type": "post",
    "content": {
        "post": {
            "zh_cn": {
                "title": "我是一个标题",
                "content": [
                    [
                        {
                            "tag": "text",
                            "un_escape": true,
                            "text": "第一行&nbsp;:"
                        },
                        {
                            "tag": "a",
                            "text": "超链接",
                            "href": "http://www.larksuite.com"
                        },
                        {
                            "tag": "at",
                            "user_id": "ou_18eac85d35a26f989317ad4f02e8bbbb"
                        }
                    ],
                    [
                        {
                            "tag": "text",
                            "text": "第二行 :"
                        },
                        {
                            "tag": "text",
                            "text": "文本测试"
                        }
                    ],
                    [
                        {
                            "tag": "img",
                            "image_key": "d640eeea-4d2f-4cb3-88d8-c964fab53987",
                            "width": 300,
                            "height": 300
                        }
                    ]
                ]
            }
        }
    }
}
```

### 展示效果

![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/a3074630f25fb0e7d3df394c1348ba41_cn.png)



### 支持的标签和参数说明
 
#### text
字段 |类型|必须| 描述|默认值|实例|
-- | -- |-- |-- |-- |-- |
text|string| 是 | 显示的默认的文本内容，如果设置了 i18n 内容，会优先显示 i18n 里面对应的语种内容||test text|
un_escape|bool| 否 |  表示是不是 unescape 解码，默认为 false ，不用可以不填||false|

#### a
字段 |类型 |必须| 描述|默认值|实例|
-- | -- |-- |-- |-- |-- |
text |string|是 | 显示的默认的文本内容，如果设置了 i18n 内容，会优先显示 i18n 里面对应的语种内容||test text|
un_escape |bool| 否 |  表示是不是 unescape 解码，默认为 false ，不用可以不填||false|
href|string|  是 |默认的链接地址||https://bytedance.com|


#### at
字段 |类型 |必须| 描述|默认值|实例|
-- | -- |-- |-- |-- |-- |
user_id |string| 是 | open_id 或者 user_id||ou_18eac85d35a26f989317ad4f02e8bbbb|


#### img
字段 |类型 |必须| 描述|默认值|实例|
-- | -- |-- |-- |-- |-- |
image_key |string|是 | 图片的唯一标识，可以通过[图片上传接口](/document/ukTMukTMukTM/uEDO04SM4QjLxgDN)获得||d640eeea-4d2f-4cb3-88d8-c964fab53987|
height|int| 是 | 图片的高||300|
width |int| 是 | 图片的宽||300|






