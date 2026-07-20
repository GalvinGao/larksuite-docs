---
document_id: '6967331158355984390'
directory_id: '7021842990278164485'
title: 发送仅特定人可见的消息卡片
full_path: /ukTMukTMukTM/uETOyYjLxkjM24SM5IjN
breadcrumb:
- Server API
- Messaging
- Message card
- Send message cards that are only visible to certain people
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:46Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uETOyYjLxkjM24SM5IjN
---

# 发送仅特定人可见的消息卡片
用于机器人在群会话中发送仅指定用户可见的消息卡片。卡片上将展示"仅对你可见"标识。<br>

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b0ec0ce45942463381457edc7b62e144_RXYCFtfUtb.png?lazyload=true&width=1592&height=486)
## 使用场景
临时消息卡片多用于群聊中用户与机器人交互的中间态。例如在群聊中用户需要使用待办事项类bot创建一条提醒，bot 发送了可设置提醒日期和提醒内容的一张可交互的消息卡片，此卡片在没有设置为临时卡片的情况下为群内全员可见，即群内可看见该用户与 bot 交互的过程。而设置为临时卡片后，交互过程仅该用户可见，群内其他成员只会看到最终设置完成的提醒卡片。
<br><br>通过临时消息卡片，可以减少消息对群聊中不相关用户的打扰，有效降低群消息的噪声。

:::html
<md-alert type="warn">
需要启用[机器人能力](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)；需要机器人在会话群里。
</md-alert>
:::

:::html
<md-alert type="warn">
-  仅触发临时卡片的用户自己可见。
- 不支持转发。
- 只能在群聊使用。
- 仅在用户处于在线状态的Lark客户端上可见。
- 临时消息卡片的[呈现能力](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN)、[交互能力](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN)与消息卡片一致。
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
      <md-td>https://open.larksuite.com/open-apis/ephemeral/v1/send</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
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
| 参数| 类型   | 必须 | 说明  |示例 |                                                   
| - | - | - | - | - |
| chat_id|string | 是   | 发送临时消息的群ID可通过事件推送获取 |oc_5ad573a6411d72b8305fda3a9c15c70e|
open_id <br>user_id <br> email <br>   | string | 是 | 指定发送临时消息卡片的用户，其他人将无法看到临时消息卡片；只需要填 open_id、email、user_id中的一个即可，推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid) （服务端依次读取字段的顺序为 **open_id** > **user_id** > **email**）|ou_5ad573a6411d72b8305fda3a9c15c70e|
| msg_type                                          | string | 是   | 消息的类型，此处固定填 "interactive" |interactive|
| card|object | 是   | 消息卡片的描述内容，具体参考 [基础结构](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN)|
### 请求体示例

```json
{
   "chat_id": "oc_xxxxxxxxxxxx",
   "open_id": "ou_xxxxxxxxxxxx",
   "msg_type": "interactive",
   "card": {
        // card content
    }
}
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
    "data": {
        "message_id": "om_fd2057276f16243756ff8eb6fcd7b705"
    },
    "msg": "ok"
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)

### 请求体demo
```json
{
	 "open_id":"ou_515fbe9d04838174e2035f801453d07f",
	 "chat_id":"oc_286e0a771a76d99f1ec72737ef92ee8d",
     "msg_type": "interactive",
     "card": {
        "elements": [
            {
                "tag": "div",
                "text": {
                    "tag": "plain_text",
                    "content": "overflow & datePicker 功能"
                },
                "fields": [
                    {
                        "is_short": true,
                        "text": {
                            "tag": "lark_md",
                            "content": "**组件模块:**\noverflow & datePicker"
                        }
                    },
                    {
                        "is_short": false,
                        "text": {
                            "tag": "lark_md",
                            "content": "**功能点:**\n"
                        }
                    }
                ]
            },
            {
                "tag": "hr"
            },
            {
                "tag": "action",
                "actions": [
                    {
                        "tag": "overflow",
                        "confirm": {
                            "title": {
                                "tag": "plain_text",
                                "content": "请确认"
                            },
                            "text": {
                                "tag": "plain_text",
                                "content": "跳转方式"
                            }
                        },
                        "options": [
                            {
                                "text": {
                                    "tag": "plain_text",
                                    "content": "多端跳转"
                                },
                                "multi_url": {
                                    "url": "https://www.baidu.com",
                                    "android_url": "https://developer.android.com/",
                                    "ios_url": "https://developer.apple.com/",
                                    "pc_url": "https://www.windows.com"
                                },
                                "value": "james"
                            },
                            {
                                "text": {
                                    "tag": "plain_text",
                                    "content": "指定跳转"
                                },
                                "url": "https://www.baidu.com"
                            },
                            {
                                "text": {
                                    "tag": "plain_text",
                                    "content": "文本内容"
                                },
                                "value": "joy"
                            },
                            {
                                "text": {
                                    "tag": "plain_text",
                                    "i18n": {
                                        "zh_cn": "中文文本-james",
                                        "en_us": "English text-james",
                                        "ja_jp": "日本語文案-james"
                                    }
                                },
                                "value": "james"
                            }
                        ]
                    },
                    {
                        "tag": "date_picker",
                        "placeholder": {
                            "tag": "plain_text",
                            "content": "请选择日期(配有confirm)"
                        },
                        "confirm": {
                            "title": {
                                "tag": "plain_text",
                                "content": "请确认"
                            },
                            "text": {
                                "tag": "plain_text",
                                "content": "跳转方式"
                            }
                        }
                    },
                    {
                        "tag": "date_picker",
                        "placeholder": {
                            "tag": "plain_text",
                            "content": "请选择日期"
                        },
                        "initial_date": "2002-2-22"
                    },
                    {
                        "tag": "picker_time",
                        "placeholder": {
                            "tag": "plain_text",
                            "content": "请选择日期(配有confirm)"
                        },
                        "confirm": {
                            "title": {
                                "tag": "plain_text",
                                "content": "请确认"
                            },
                            "text": {
                                "tag": "plain_text",
                                "content": "跳转方式"
                            }
                        }
                    },
                    {
                        "tag": "picker_time",
                        "placeholder": {
                            "tag": "plain_text",
                            "content": "请选择日期"
                        },
                        "initial_time": "19:09"
                    },
                    {
                        "tag": "picker_datetime",
                        "placeholder": {
                            "tag": "plain_text",
                            "content": "请选择日期(配有confirm)"
                        },
                        "confirm": {
                            "title": {
                                "tag": "plain_text",
                                "content": "请确认"
                            },
                            "text": {
                                "tag": "plain_text",
                                "content": "跳转方式"
                            }
                        }
                    },
                    {
                        "tag": "picker_datetime",
                        "placeholder": {
                            "tag": "plain_text",
                            "content": "请选择日期"
                        },
                        "initial_datetime": "2002-2-22 18:00"
                    }
                ]
            }
        ]
    }
 }
```




