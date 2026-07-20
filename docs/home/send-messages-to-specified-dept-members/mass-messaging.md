---
document_id: '7074952334766030854'
directory_id: '7073442394955628549'
title: 消息群发
full_path: /home/mass-messaging-to-designated-departments/mass-messaging
breadcrumb:
- Home
- Send Messages to Specified Dept. Members
- Mass messaging
document_type: GuideDocumentType
updated_at: 2023-05-16T03:11:42Z
source_url: https://open.larksuite.com/document/home/mass-messaging-to-designated-departments/mass-messaging
---

# 消息群发
**向部门每个成员推送单聊卡片:**

这里要用租户应用身份

```bash
curl -X POST -H '{"Authorization":"Bearer {{{tenant_access_token}}}"}' -H 'Content-Type: application/json' https://open.larksuite.com/open-apis/message/v4/batch_send/ '{
    "department_ids": [
        "{{department_id}}",
        "{{department_id}}"
    ],
    "msg_type": "text",
    "content": {
        "text": "你好同学"
    }
}' 
``` 
会返回如下内容：


```json 
{
    "code": 0,
    "msg": "ok",
    "data":{
      "invalid_department_ids": [
      ],
      "message_id": "xxx"
      }
} 
``` 
```invalid_department_ids``` 为空的话，说明传入的```department id``` 部门发送成功了，否则需要检查id是否有误。

这样，就完成了消息群发，即向部门所有同学推送单聊消息。

**消息频控策略限制：**

对于部门内的消息群发，如果部门有N人，会记为N次消息请求，受到Lark整体的消息[频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)限制。如果部门成员比较多，推送周期将比较久。
|     |   | *powered by guilin* |
| :-------- | -------:|  -------: |
