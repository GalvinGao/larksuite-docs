---
document_id: '7069302182251806725'
directory_id: '7069296428547686405'
title: 机器人相关
full_path: /ugTN1YjL4UTN24CO1UjN/uUzN1YjL1cTN24SN3UjN
breadcrumb:
- Developer Guides
- FAQ
- Server-side develop
- Questions on bot
document_type: GuideDocumentType
updated_at: 2022-03-04T11:13:28Z
source_url: https://open.larksuite.com/document/ugTN1YjL4UTN24CO1UjN/uUzN1YjL1cTN24SN3UjN
---

#  机器人应用相关问题
## 在开发者后台配置了机器人，但是为什么我的机器人没有出现对话框？

**答**：请确认你的应用配置了订阅事件网址，并且订阅了“接收消息”事件，才会出现对话框。

## 怎么实现机器人@人（@所有人、@指定人）

**答**：在机器人发送的普通文本消息（text）、富文本消息（post）、消息卡片（interactive）中，可以使用`at`标签实现@人效果。具体请求示意如下：<br>

**(1) 在[普通文本消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)中@人、@所有人:**<br>

`at`标签示意

```json 
// at 指定用户
<at user_id="ou_xxx">Name</at> //取值必须使用ou_xxxxx格式的 open_id 来at指定人
// at 所有人
<at user_id="all">所有人</at>
``` 

请求体中的`content` 示意：

```json 
{
	"content": " {\"text\":\"<at user_id=\\\"ou_xxxxxxxx\\\">Tom</at> text content\"}"
}
``` 
**(2) 在[富文本消息](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json#45e0953e)中@人、@所有人:**<br>

`at`标签示意

```json 
// at 指定用户
{
	"tag": "at",
	"user_id": "ou_xxxxxxx",//取值必须使用ou_xxxxx格式的 open_id 来at指定人
	"user_name": "tom"
}

// at 所有人
{
	"tag": "at",
	"user_id": "all",//取值使用"all"来at所有人
	"user_name": "所有人"
}
``` 
请求体中的`content` 示意：

```json 
{
  "zh_cn": {
    "title": "我是一个标题",
    "content": [
      [
        {
          "tag": "text",
          "text": "第一行 :"
        },
        {
          "tag": "at",
          "user_id": "ou_xxxxxx",//取值必须使用ou_xxxxx格式的 open_id 来at指定人
          "user_name": "tom"
        }
      ],
      [
        {
          "tag": "text",
          "text": "第二行:"
        },
        {
          "tag": "at",
          "user_id": "all",
          "user_name": "所有人"//取值使用"all"来at所有人
        }
      ]
    ]
  }
}
``` 
**(3) 在[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)中@人、@所有人:**<br>
可以使用消息卡片[Markdown](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)内容中的at人标签，标签示意：

```json 
// at 指定用户
	<at id=ou_xxx></at> //使用open_id at指定人
	<at id=b6xxxxg8></at> //使用user_id at指定人
	<at email=test@email.com></at> //使用邮箱地址 at指定人
// at 所有人
	<at id=all></at>
``` 

请求体中的`content` 示意：

```json 
{
  "config": {
    "wide_screen_mode": true
  },
  "header": {
    "title": {
      "tag": "plain_text",
      "content": "这是卡片标题内容"
    },
    "template": "blue"
  },
  "elements": [
    {
      "tag": "div",
      "text": {
        "content": "at所有人<at id=all></at> \nat指定人<at id=ou_xxxxxx></at>",
        "tag": "lark_md"
      }
    }
  ]
}
``` 
**注意：**
-   对于[自定义机器人](/document/ukTMukTMukTM/ucTM5YjL3ETO24yNxkjN)，由于不具有通讯录信息的访问权限，只支持使用 `open_id` @指定人，或 @所有人。不支持使用 `user_id`、`email` @指定人。[自建应用与商店应用](/document/home/app-types-introduction/self-built-apps-and-store-apps)则没有此限制。你可以[参考此教程](/document/home/user-identity-introduction/how-to-get)，以任意应用身份请求获取用户的`open_id`。
-   如果群主开启了`“仅群主和群管理员可@所有人”`配置，且机器人不是群主或管理员，则无法@所有人。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/aab2116c9e76e96c2b341022e039858f_ZbDJWADgDH.png)





## 自定义机器人的 webhook地址有 V1、V2 版本，如何兼容？

**答**：请参考帮助文档[如何在群聊中使用机器人](https://www.feishu.cn/hc/zh-CN/articles/360024984973-%E6%9C%BA%E5%99%A8%E4%BA%BA-%E5%9C%A8%E7%BE%A4%E8%81%8A%E4%B8%AD%E4%BD%BF%E7%94%A8%E6%9C%BA%E5%99%A8%E4%BA%BA#%E9%99%84%E5%BD%95%EF%BC%9A%E6%97%A7%E7%89%88%20webhook%20(%E8%87%AA%E5%AE%9A%E4%B9%89%E6%9C%BA%E5%99%A8%E4%BA%BA)%20%E4%BD%BF%E7%94%A8%E8%AF%B4%E6%98%8E%EF%BC%88%E6%8E%A8%E8%8D%90%E4%BD%BF%E7%94%A8%E6%96%B0%E7%89%88%E6%9C%AC%E6%9C%BA%E5%99%A8%E4%BA%BA%EF%BC%89)的附录部分“旧版 webhook (自定义机器人) 使用说明”。同时，我们推荐使用V2版本的自定义机器人。

## 配置使用 webhook 的自定义机器人时，参数text是否有长度要求？

**答**：建议 JSON 的长度不超过 30k，序列化后的 pb 不超过 100k，图片最好小于 10MB。

