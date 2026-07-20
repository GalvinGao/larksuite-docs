---
document_id: '7026663890609553414'
directory_id: '7002892512470679558'
title: 接收消息内容
full_path: /uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content
breadcrumb:
- Server API
- Messaging
- Message management
- Message content introduction
- Received message content
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:29Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content
---

# 接收消息内容

:::note
本文将说明从[获取会话历史消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list)、[获取指定消息的内容](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/get)等接口中获得的各类型消息的 content 字段。
:::

## 示例

Content 是 String 类型， JSON 结构的消息内容。

以文本类型消息为例，示例如下：

```json 
{
    "content": "{\"text\":\"文本消息\"}"
}
```


## 各类型消息 JSON 结构

### 文本 text
:::note
- 文本消息中，超链接的格式为 `[超链接文本](超链接地址)`，如：`[Lark 开放平台](https://open.larksuite.com)`；特别地，对于邮箱类型的超链接，格式为 `[邮箱文本](mailto:邮箱地址)`。
- 消息中的@会被替换为 “@_user_x" 形式的内容，表示被@的用户或机器人的序号。例如，第3个被@到的成员，值为“@_user_3”。 可以根据该序号在消息 ==mentions== 字段中获取被@的用户或机器人的详细信息，详见[资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#39ee4e65)。
- 粗体、下划线等文本样式将被忽略，仅显示文本内容。
:::

```json 
{
    "text": "@_user_1 文本消息"
}

```




### 富文本 post
:::note
获取消息时的内容（包括发送消息响应体中的消息内容）与发送时的内容**不完全一致**。`md` 标签仅支持发送，获取消息内容时将不包含此标签，会根据 `md` 中的内容转换为其他标签；此外，引用、有序、无序列表会退化成文本标签（text tag）进行输出。
:::

```json 
{
    "title":"我是一个标题",
    "content":[
        [
            {
                "tag":"text",
                "text":"第一行 :",
                "style": ["bold", "underline"]
            },
            {
                "tag":"a",
                "href":"http://www.larksuite.com",
                "text":"超链接",
                "style": ["bold", "italic"]
            },
            {
                "tag":"at",
                "user_id":"@_user_1",
                "user_name":"",
                "style": []
            }
        ],
        [
            {
                "tag":"img",
                "image_key":"img_47354fbc-a159-40ed-86ab-2ad0f1acb42g"
            }
        ],
        [
            {
                "tag":"text",
                "text":"第二行:",
                "style": ["bold", "underline"]
            },
            {
                "tag":"text",
                "text":"文本测试",
                "style": []
            }
        ],
        [
            {
                "tag":"img",
                "image_key":"img_47354fbc-a159-40ed-86ab-2ad0f1acb42g"
            }
        ],
        [
            {
                "tag":"media",
                "file_key": "file_v2_0dcdd7d9-fib0-4432-a519-41d25aca542j",
                "image_key": "img_7ea74629-9191-4176-998c-2e603c9c5e8g"
            }
        ],
        [
            {
                "tag": "emotion",
                "emoji_type": "SMILE"
            }
        ],
        [
            {
                "tag": "hr"
            }
        ],
        [
            {
                "tag": "code_block",
                "language": "GO",
                "text": "func main() int64 {\n    return 0\n}"
            }
        ]
    ]
}

```

 **富文本支持的标签和参数说明** :

 **text** 

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| text | string | 文本内容 |
| un\_escape | bool | 表示是不是 unescape 解码 |
| style | []string | 文本内容的加粗、下划线、删除线和斜体样式，可选值分别为`bold`、`underline`、`lineThrough`与`italic`，没有样式则为空列表 |

 **a** 

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| text | string | 链接显示的文本内容 |
| href | string | 链接地址 |
| style | []string | 文本内容的加粗、下划线、删除线和斜体样式，可选值分别为`bold`、`underline`、`lineThrough`与`italic`，没有样式则为空列表 |

 **at** 

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| user\_id | string | 被@的用户或机器人的序号。例如，第3个被@到的成员值为“@_user_3”；@成员的详细信息可在消息 ==mentions== 中根据序号获取，详见[资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#39ee4e65) |
| user\_name | string | 用户姓名 |
| style | []string | 文本内容的加粗、下划线、删除线和斜体样式，可选值分别为`bold`、`underline`、`lineThrough`与`italic`，没有样式则为空列表 |

 **img** 

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| image\_key | string | 图片的唯一标识 |

 **media** 
 
| 字段 | 类型 | 描述 |
| --- | --- | --- |
| file\_key | string | 视频文件的唯一标识 |
| image\_key | string | 视频封面图片的唯一标识 |

 **emotion** 

| 字段 | 类型 | 描述 |
| --- | ---| --- |
| emoji_type | string | 表情类型， 部分可选值请参见[表情文案](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/emojis-introduce) |


 **code_block** 

| 字段 | 类型 | 描述 |
| --- | ---| --- |
| language | string | 代码块语言，支持 PYTHON、C、CPP、GO、JAVA、KOTLIN、SWIFT、PHP、RUBY、RUST、JAVASCRIPT、TYPESCRIPT、BASH、SHELL、SQL、JSON、XML、YAML、HTML、THRIFT等 |
| text | string | 代码块内容 |

 **hr** 

富文本支持 `tag` 为 `hr`，表示一条分割线。无其它参数。


### 图片 image

```json 
{
    "image_key": "img_4adb3cc3-902b-4187-b0f1-842f67fd017g"
}

```

### 文件 file

```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg",
    "file_name": "test.txt" // 文件名
}

```

### 文件夹 folder

```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg",
    "file_name": "folder" //文件夹名称 
}

```

### 音频 audio

```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg", // 文件key
    "duration": 2000                                    // 时长 毫秒级,
}

```

### 视频 media

```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg",  // 文件key
    "image_key": "img_xxxxxx",                           // 视频封面图片key
    "file_name":"测试视频.mp4",                            // 文件名
    "duration": 2000                                     // 视频时长 毫秒级,
}

```

### 表情包 sticker

```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg"
}

```


### 消息卡片 interactive
:::note
注意：该卡片结构与在卡片搭建工具中获取的卡片 JSON 不一致，暂不支持返回原始卡片 JSON。
:::

卡片结构各字段说明请参考[了解卡片结构](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN)。  
```json 
{
    "title": "卡片标题",
    "elements": [
        [
            {
                "tag": "button",
                "text": "主按钮",
                "type": "primary"
            },
            {
                "tag": "button",
                "text": "次按钮",
                "type": "default"
            },
            {
                "tag": "button",
                "text": "危险按钮",
                "type": "danger"
            }
        ],
        [
            {
                "tag": "a",
                "href": "https://www.larksuite.com",
                "text": "Lark"
            },
            {
                "tag": "text",
                "text": "整合即时沟通、日历、音视频会议、云文档、云盘、工作台等功能于一体，成就组织和个人，"
            },
            {
                "tag": "at",
                "user_id": "@_user_1",
                "user_name": ""
            },
            {
                "tag": "text",
                "text": "更高效、更愉悦。"
            }
        ],
        [
            {
                "tag": "hr"
            }
        ],
        [
            {
                "tag": "text",
                "text": "图片标题"
            },
            {
                "tag": "img",
                "image_key": "img_acd8a194-3e63-49ca-bcf6-224624457a3g"
            }
        ],
        [
            {
                "tag": "note",
                "elements": [
                    {
                        "tag": "img",
                        "image_key": "img_acd8a194-3e63-49ca-bcf6-224624457a3g"
                    },
                    {
                        "tag": "text",
                        "text": "备注信息"
                    }
                ]
            }
        ],
        [
            {
                "tag": "text",
                "text": "深度整合使用率极高的办公工具，企业成员在一处即可实现高效沟通与协作。"
            },
            {
                "tag": "img",
                "image_key": "img_acd8a194-3e63-49ca-bcf6-224624457a3g"
            }
        ],
        [
            {
                "tag": "text",
                "text": "在移动端同样进行便捷的沟通、互动与协作，手机电脑随时随地保持同步。"
            },
            {
                "tag": "select_static",
                "options": [
                    "选项1",
                    "选项2",
                    "选项3",
                    "选项4"
                ],
                "placeholder": "默认提示文本"
            }
        ],
        [
            {
                "tag": "text",
                "text": "ISV产品接入及企业自主开发，更好地对接现有系统，满足不同组织的需求。"
            },
            {
                "tag": "overflow",
                "options": [
                    "打开 Lark 应用目录",
                    "打开 Lark 开发文档",
                    "打开 Lark 官网"
                ]
            }
        ],
        [
            {
                "tag": "text",
                "text": "国际权威安全认证与信息安全管理体系，为企业提供全生命周期安全保障。"
            },
            {
                "tag": "date_picker",
                "placeholder": "请选择日期",
                "initial_date": "2021-1-1"
            }
        ]
    ]
}

```

### 红包 hongbao

```json 
{
    "text": "[红包]"
}

```

### 日程
#### 日程分享卡片 share_calendar_event

```json 
{
    "summary": "日程分享测试",
    "start_time": "1608265395000", // 毫秒级时间戳
    "end_time": "1608267015000"    // 毫秒级时间戳
}

```

#### 日程邀请卡片 calendar

```json 
{
    "summary": "日程邀请测试",
    "start_time": "1608265395000", // 毫秒级时间戳
    "end_time": "1608267015000"    // 毫秒级时间戳
}

```

#### 日程转让卡片/日程附言/切换日程所在日历 general_calendar

```json 
{
    "summary": "日程转让测试",
    "start_time": "1608265395000", // 毫秒级时间戳
    "end_time": "1608267015000"    // 毫秒级时间戳
}

```

### 群名片 share_chat

```json 
{
    "chat_id": "oc_0dd200d32fdaxxxxxxxx32f76"
}

```

### 个人名片 share_user

```json 
{
    "user_id": "ou_0dd200d32xxxxx6d2c2ef1ddb32f76" // 用户open_id
}

```

### 系统消息 system
根据系统消息模板 `template` 中的变量{xxx}，取相应的变量名参数值。

```json 
{
        "template": "{from_user} invited {to_chatters} to this chat.",
        "from_user": ["botName"],
        "to_chatters": ["小明", "小王", "小红"]
}
```
```json

{
        "template": "{divider_text}",
        "from_user": [],
        "to_chatters": [],
        "divider_text": {
            "text": "新会话",
            "i18n_text": {
               "zh_cn": "新话题",
               "en_us": "New Session"
            }
        }
}

```


### 位置 location

```json 
{
    "name": "xx省xx市",
    "longitude": "xxx.xxx",
    "latitude": "xxx.xxx"
}

```

### 视频通话 video_chat

```json 

{
    "topic": "视频通话消息",
    "start_time": "1623124523829" // 毫秒级时间戳
}
```

### 任务 todo

```json 
{
    "task_id": "acd096a5-a157-4b9d-80e2-5b317456f005",
    "summary": {"title":"","content":[[{"tag":"text","text":"多吃水果，多运动，健康生活，快乐工作。"}]]}, 
    "due_time": "1623124318000"
}
```

**任务的参数说明** :

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| task_id | string | 任务ID，使用此ID可以对任务进行操作，详情参见[任务功能概述](/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/overview) |
| summary | post | [富文本](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content#45e0953e)格式的任务标题 |
| due_time | string | 任务截止时间的毫秒级时间戳 |

### 投票 vote

```json 
{
    "topic": "投票测试",
    "options": ["选项1","选项2","选项3"]
}
```
**投票的参数说明** :

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| topic | string | 投票主题 |
| options | list | 选项内容列表 |


### 合并转发 merge_forward
```json 
{
    "content": "Merged and Forwarded Message"
}
```
消息内容无实际意义，可调用[获取指定消息的内容](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/get)获取合并转发消息中的子消息。




