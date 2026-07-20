---
document_id: '7139727755097718789'
directory_id: '7137610938320961542'
title: 发送与更新审批bot消息
full_path: /home/quickly-develop-three-party-approvals/send-and-update-approval-bot-messages
breadcrumb:
- Home
- Quickly develop three-party approvals
- Send and update approval bot messages
document_type: GuideDocumentType
updated_at: 2023-05-17T03:03:42Z
source_url: https://open.larksuite.com/document/home/quickly-develop-three-party-approvals/send-and-update-approval-bot-messages
---

# 发送与更新审批bot消息

当有新的审批待办，或者审批待办的状态有更新时，你可以通过Lark审批的Bot告知用户。

:::note
当前调试台不支持调试该接口，你可以使用本地命令工具进行测试。
:::

## 发送审批bot消息

你可以通过调用**发送审批Bot消息**接口，将审批代办消息实时推送给用户。

请求示例如下：
:::note
参数说明可参考[发送审批Bot消息](/document/ukTMukTMukTM/ugDNyYjL4QjM24CO0IjN)。
:::

```
curl --location --request POST 'https://www.larksuite.com/approval/openapi/v1/message/send' \
--header 'Authorization: Bearer t-790b33160a8af2387c93dad2f61e07d8d9f4f2e1' \
--header 'Content-Type: application/json; charset=utf-8' \
--data-raw '{
    "template_id":"1008",
    "user_id":"16fb9ff3",
    "uuid":"uuid12",
    "approval_name":"@i18n@1",
    "title_user_id":"16fb9ff3",
    "title_user_id_type": "user_id",
    "comment":"@i18n@2",
    "content":{
        "user_id":"",
         "user_id_type": "",
        "department_id":"",
        "summaries":[
            {
                "summary":"@i18n@3"
            }
        ]
    },
    "note":"@i18n@4",
    "actions":[
        {
            "action_name":"DETAIL",
            "url":" https://www.baidu.com",
            "android_url":"https://bytedance.com",
            "ios_url":"https://bytedance.com",
            "pc_url":"https://www.baidu.com"
        }
    ],
    "action_configs": [
        {
            "action_type": "APPROVE",
            "is_need_reason": true,
            "is_reason_required": true,
            "is_need_attachment": true,
            "next_status": "APPROVED"
        },
        {
            "action_type": "REJECT",
            "action_name": "@i18n@5",
            "next_status": "REJECTED"
        }
    ],
    "action_callback": {
        "action_callback_url":"https://www.baidu.com",
        "action_callback_token":"",
        "action_callback_key":"",
        "action_context":""
    },
    "i18n_resources":[
        {
            "locale":"en_us",
            "is_default":true,
            "texts":{
                "@i18n@1":"Temporary release",
                "@i18n@2":"Disapproval",
                "@i18n@3":"Need to modify",
                "@i18n@4":"From OA,please access through the internal network.",
                "@i18n@5":"Cancel"
            }
        }
    ]
}'
```

发送成功后，Lark审批机器人会发送一条消息卡片到审批人。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/20eac5b1d3cac5e331b677d92d561a3e_olG78vjSO0.png?lazyload=true&width=707&height=249)

你也可以利用Lark开放平台的能力，自建一个审批的机器人，推送相关审批信息。

## 更新审批bot消息

审批人完成审批操作之后，你可以调用 **更新审批bot消息** 接口，更新审批bot消息。

请求示例如下：
:::note
参数说明可参考[更新审批bot消息](/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN)。
:::
```
curl --location --request POST 'https://www.larksuite.com/approval/openapi/v1/message/update' \
--header 'Authorization: Bearer t-790b33160a8af2387c93dad2f61e07d8d9f4f2e1' \
--header 'Content-Type: application/json; charset=utf-8' \
--data-raw '{
    "message_id":"7119390774281977859",
    "status":"APPROVED"
}'
```

接口调用成功后，Lark审批机器人中消息卡片的状态会发生改变。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/812f312b55a8e97f0408181b552081e3_TQU8NMgDn6.png?lazyload=true&width=715&height=201)
