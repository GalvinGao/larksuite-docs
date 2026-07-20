---
document_id: '6995834315541053446'
directory_id: '7021842990278148101'
title: 批量发送消息
full_path: /ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM
breadcrumb:
- Server API
- Messaging
- Batch message
- Send messages in batches
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:34Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM
---

# 批量发送消息

给多个用户或者多个部门中的成员发送消息。

:::note
**注意事项：**
- 应用需要启用[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability) 
- 接口权限说明：
	- 必须拥有**获取与发送单聊、群组消息**权限，或者**以应用的身份发消息**权限
	- 至少拥有一个批量发送消息权限：
  		- 给用户发送需要拥有 **给多个用户批量发消息** 权限
  		- 给部门成员发送需要拥有 **给一个或多个部门的成员批量发消息** 权限
- 应用需要拥有对所发送用户或部门的[可用性](/document/home/introduction-to-scope-and-authorization/availability)
- 通过该接口发送的消息 **不支持更新以及回复等操作**
- 只能发送给用户，无法发送给群组
- 异步接口，会有一定延迟，每个应用待发送的消息按顺序处理，请合理安排批量发送范围和顺序。发送消息给单个用户的场景请使用[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)接口
- 单个应用每天通过该接口发送的总消息条数不超过50万
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
      <md-td>https://open.larksuite.com/open-apis/message/v4/batch_send/</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
    </md-tr>
    <md-tr>
      <md-th>接口频率限制</md-th>
      <md-td>[1000 次/分钟、50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)</md-td>
    </md-tr>
   <md-tr>
     <md-th>支持的应用类型</md-th>
      <md-td>
	  <md-app-support types="custom,isv"></md-app-support>
      </md-td>
   </md-tr>


    
    
    <md-tr>
      <md-th>
权限要求
 <md-tooltip type="info">调用该 API 所需的权限。</md-tooltip>
<div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一必选权限即可<br>根据发送目标申请相应可选权限</div>
</md-th>
      <md-td>
<md-perm name="im:message" desc="获取与发送单聊、群组消息（必选）" support_app_types="custom,isv" tags="">获取与发送单聊、群组消息</md-perm>
<md-perm name="im:message:send_as_bot" desc="以应用的身份发消息（必选）" support_app_types="custom,isv" tags="">获取与发送单聊、群组消息</md-perm>  
<md-perm name="im:message:send_multi_users" desc="给多个用户批量发消息（可选）" support_app_types="custom,isv" tags="">给多个用户批量发消息</md-perm>
<md-perm name="im:message:send_multi_depts" desc="给一个或多个部门的成员批量发消息（可选）" support_app_types="custom,isv" tags="">给一个或多个部门的成员批量发消息</md-perm>
</md-td>
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
      <md-td> 是 </md-td> 
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>
 
**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"
          
 [了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)
	</md-td>
</md-tr>
     <md-tr> 
      <md-td>Content-Type</md-td>  
      <md-td>string</md-td>  
      <md-td> 是 </md-td> 
     <md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
   
  </md-tbody> 
</md-table>
:::


### 请求体
参数|类型|必填|描述|  
--|--|--|--|  
msg_type | string | 是 | 消息类型，支持多种消息类型，详见本文“**消息类型及内容示例**”部分|
content | object | 否 | 消息内容，支持除卡片消息外的多种消息内容，详见本文“**消息类型及内容示例**”部分|
card | object | 否 | 卡片消息内容 <br><br>**注意**：card和content字段必须二选一|
department_ids | list | 否 | 支持[自定义部门ID](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/terminology#3c3e6267)和open_department_id，列表长度小于等于 200  <br><br>**注意**：部门下的所有子部门包含的成员也会收到消息<br><br>**示例值：**["3dceba33a33226","d502aaa9514059", "od-5b91c9affb665451a16b90b4be367efa"]|
open_ids | list | 否 | 用户 open_id 列表，长度小于等于 200；ID获取方式可参考文档：[如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid) <br><br>**示例值：**["ou_18eac85d35a26f989317ad4f02e8bbbb","ou_461cf042d9eedaa60d445f26dc747d5e"]|
user_ids | list | 否 | 用户 user_id 列表，长度小于等于 200；ID获取方式可参考文档：[如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)  <br><br>**示例值：**["7cdcc7c2","ca51d83b"]|
union_ids | list | 否 | 用户 union_ids 列表，长度小于等于 200；ID获取方式可参考文档：[如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)  <br><br>**示例值：**["on_cad4860e7af114fb4ff6c5d496d1dd76","on_gdcq860e7af114fb4ff6c5d496dabcet"]|

:::note
- content, card两个字段必须选择其中的一个填写
- department_ids, open_ids, user_ids, union_ids 四个字段至少填一个
:::

#### 消息类型及内容示例
|     msg_type    |          内容示例        |说明 |
| --------- | --------------- | -------   | ----------- | --------- |
|text  |{"content": { "text": "要发送的文本消息" } }| 文本消息 |
|image  |{"content": { "image_key": "xxx-xxx-xxx-xxx-xxx" } }| 其中image key需要通过[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口取得 |
|post  | {"content": { "post":{ POST_CONTENT }}} | post content格式请参见[发送消息Content](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json#45e0953e)|
|share_chat  | {"content":{"share_chat_id": "oc_xxx"}} | 分享群名片|
|interactive  | {"card":{...}} | 卡片消息，构造详见 [消息卡片搭建工具](/document/ukTMukTMukTM/uYzM3QjL2MzN04iNzcDN/message-card-builder) <br><br>**注意**：卡片消息不使用content字段，而是使用card字段|


### 请求体示例
:::note
批量发送消息的内容为JSON结构，不需要转义。
:::

#### 文本消息
```json
{
    "department_ids": [
        "3dceba33a33226",
        "d502aaa9514059"
    ],
    "open_ids": [
        "ou_18eac85d35a26f989317ad4f02e8bbbb",
        "ou_461cf042d9eedaa60d445f26dc747d5e"
    ],
    "user_ids": [
        "7cdcc7c2",
        "ca51d83b"
    ],
    "union_ids": [
         "on_cad4860e7af114fb4ff6c5d496d1dd76",
         "on_gdcq860e7af114fb4ff6c5d496dabcet"
    ],
    "msg_type": "text",
    "content": {
        "text": "test content"
    }
}
```

#### 富文本消息

```json 
{
	"open_ids": [
		"ou_a18fe85d22e7633852d8104226e99eac",
		"ou_9204a37300b3700d61effaa439f34295"
	],
	"department_ids": [
		"od-5b91c9affb665451a16b90b4be367efa"
	],
	"msg_type": "post",
	"content": {
		"post":{
			"zh_cn": {
				"title": "我是一个标题",
				"content": [
					[{
						"tag": "text",
						"text": "第一行"
					},
					{
						"tag": "a",
						"href": "http://www.larksuite.com",
						"text": "Lark"
					}
					]
				]
			},
			"en_us": {
				"title": "I am a title",
				"content": [
					[{
						"tag": "text",
					"text": "first line"
					},
					{
						"tag": "a",
						"href": "http://www.larksuite.com",
						"text": "Lark"
					}
					]
				]
			}
		}
	}
} 
``` 
#### 图片消息

```json 
{
    "open_ids": [
        "ou_a18fe85d22e7633852d8104226e99eac",
        "ou_9204a37300b3700d61effaa439f34295"
    ],
    "department_ids": [
        "od-5b91c9affb665451a16b90b4be367efa"
    ],
    "msg_type": "image",
    "content": {
        "image_key": "img_v2_8e5a80ec-4d94-4bb2-84d5-e25d6c28dacj"
    }
} 
``` 
#### 群分享消息

```json 
{
    "open_ids": [
        "ou_a18fe85d22e7633852d8104226e99eac",
        "ou_9204a37300b3700d61effaa439f34295"
    ],
    "department_ids": [
        "od-5b91c9affb665451a16b90b4be367efa"
    ],
    "msg_type": "share_chat",
    "content": {
        "share_chat_id": "oc_84983ff6516d731e5b5f68d4ea2e1da5"
    }
} 
``` 

#### 卡片消息

:::note
卡片消息的请求体不使用`content`字段，使用`card`字段
:::

- 使用卡片JSON发送
```json 
{
	"open_ids": [
		"ou_a18fe85d22e7633852d8104226e99eac",
		"ou_9204a37300b3700d61effaa439f34295"
	],
	"department_ids": [
		"od-5b91c9affb665451a16b90b4be367efa"
	],
	"msg_type": "interactive",
	"card": {
		"config": {
			"wide_screen_mode": true
		},
		"elements": [
		{
			"tag": "div",
			"fields": [
			{
				"is_short": true,
				"text": {
					"tag": "lark_md",
					"content": "**🗳工单来源：**\n报事报修"
				}
			},
			{
				"is_short": true,
				"text": {
					"tag": "lark_md",
					"content": "**📝工单类型：**\n客服工单"
				}
			}
			]
		}
		],

		"header": {
			"template": "turquoise",
			"title": {
				"content": "📚晒挚爱好书，赢读书礼金",
				"tag": "plain_text"
			}
		}
	}
} 
``` 
- 使用卡片模板发送
```json 
{
	"open_ids": [
		"ou_a18fe85d22e7633852d8104226e99eac",
		"ou_9204a37300b3700d61effaa439f34295"
	],
	"department_ids": [
		"od-5b91c9affb665451a16b90b4be367efa"
	],
	"msg_type": "interactive",
	"card": {
        "type": "template", 
        "data": { 
            "template_id": "ctp_xxxxxx",
            "template_variable": {
              "var_xxx": "xxxxxx"
            }
        }
	}
} 
``` 

## 响应
### 响应体
参数 |类型| 说明 
-- |--| -- 
code |int| 错误码，非 0 表示失败
msg|string|错误描述
data | - | -
&emsp;∟message_id |string| 批量消息任务 ID，用于唯一标识一个批量发送消息的任务
&emsp;∟invalid_department_ids |list| 不合法的部门 ID 列表
&emsp;∟invalid_open_ids |list| 不合法的 open_id 列表
&emsp;∟invalid_user_ids |list| 不合法的 user_id 列表
&emsp;∟invalid_union_ids |list| 不合法的 union_id 列表


### 响应体示例
```json
{
    "code": 0,
    "msg": "ok",
    "data":{
      "invalid_department_ids": [
          "d502aaa9514059"
      ],
      "invalid_open_ids": [
          "ou_456e168d61cec276083b357f7bd3f1491",
          "ou_f8cbdb26fb2e4eda075e003381a102a41"
      ],
      "invalid_user_ids": ["7cdcc7c22"],
      "invalid_union_ids": [
         "on_cad4860e7af114fb4ff6c5d496d1dd76",
         "on_gdcq860e7af114fb4ff6c5d496dabcet"
      ],
      "message_id": "bm-d4be107c616aed9c1da8ed8068570a9f"
      }
}
```

### 错误码

HTTP状态码| 错误码| 描述| 排查建议
-- |--| -- | --
200| 10002| Create or send message fail.| 创建或发送消息失败，请参考请求返回的error信息检查消息格式。
200| 10003| Invalid parameter.| 请求参数缺失或者有误，更多错误信息请参考请求返回的error message。
200| 11101| Open_id not exist.| Open User ID非法。
200| 11203| Send employee ids permission denied.| 通过 user_id 发送消息失败。请确保在[开发者后台](https://open.larksuite.com/app) > **应用详情页** > **权限管理** > **API 权限** 中开通了 **给多个用户批量发消息** 权限。
200| 11204| Send department list permission denied.| 通过 department_ids 发送消息失败。请确保在[开发者后台](https://open.larksuite.com/app) > **应用详情页** > **权限管理** > **API 权限** 中开通了 **给一个或多个部门的成员批量发消息** 权限。
200| 11205| App do not have bot.| [机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)未启用 。可在[开发者后台](https://open.larksuite.com/app) -> 应用能力 -> 添加应用能力页面添加机器人功能，发布新版本后生效。
200| 11207| App is not usage.| 应用在当前租户下不可用，请在[开发者后台](https://open.larksuite.com/app)检查应用的状态。
200| 11209| App not exist.| 应用不存在。
200| 11210| App usage info not exist.| 应用在当前租户下未安装，不可使用，联系租户管理员，确认应用安装状态。
200| 11223| Do not have im write permission.| 当前操作者没有发送消息的权限，请开通权限后发起请求。
200| 11227| Image key not exist.| Image Key非法。
200| 11231| Chat not found.| 请求的会话不存在。
200| 11235| This group is set to only admin can mention @All.| 当前会话禁止@所有人操作。
200| 11246| Create message content fail.| 创建消息内容失败，更多错误信息请参考请求返回的error message。
200| 11247| Internal send message trigger rate limit.| 该应用通过批量发送消息接口发送的总消息数量已超过每日上限 50 万条。
200| 11312| Bot messages do not pass the audit.| 消息内容不合法，请检查消息内容。


