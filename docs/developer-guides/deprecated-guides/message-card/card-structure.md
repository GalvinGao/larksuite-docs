---
document_id: '6967331173081972741'
directory_id: '6907567266537603073'
title: 卡片结构
full_path: /ukTMukTMukTM/ugTNwUjL4UDM14CO1ATN
breadcrumb:
- Developer Guides
- Deprecated Guides
- Message Card
- Card Structure
document_type: GuideDocumentType
updated_at: 2024-07-24T08:06:00Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ugTNwUjL4UDM14CO1ATN
---

# 卡片结构

::: note
本文是消息卡片结构的专题说明，关于消息卡片的发送接口请参考[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)。
:::

消息卡片内容是通过一段 JSON 描述的，可作为 `card` 字段值使用，一段简单的消息卡片可以描述如下:

```json
{
	"chat_id": "oc_abcdefg1234567890",
	"msg_type": "interactive",
	"root_id": "om_4*********************ad8",
	"update_multi": false,
	"card": {
		"config": {
			"wide_screen_mode": true
		},
		"header": {
			"title": {
				"tag": "plain_text",
				"content": "this is header"
			}
		},
		"elements": [{
			"tag": "div",
			"text": {
				"tag": "plain_text",
				"content": "This is a very very very very very very very long text;"
			}
		}, {
			"tag": "action",
			"actions": [{
				"tag": "button",
				"text": {
					"tag": "plain_text",
					"content": "Read"
				},
				"type": "default"
			}]
		}]
	}
}
```

**展示效果：**<br>

![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/425051b2f3af6c9f529cedd5bda2508a.png?lazyload=true&width=1640&height=339)

消息卡片整体被包含在以 card 为字段名的字段下，详细可配字段和描述如下：

| 模块     | 必须 | 描述                                                         |
| -------- | ---- | ------------------------------------------------------------ |
| config | 否   | 卡片配置，参考 [卡片配置-新版](/document/ukTMukTMukTM/uAjNwUjLwYDM14CM2ATN) |
| header   | 否   | 配置卡片标题部分，参考 [卡片标题-新版](/document/ukTMukTMukTM/ukTNwUjL5UDM14SO1ATN) |
| elements | 是   | 配置卡片主体内容，参考 [卡片内容-新版](/document/ukTMukTMukTM/uEjNwUjLxYDM14SM2ATN) |
