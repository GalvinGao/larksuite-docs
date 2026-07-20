---
document_id: '7199928167141376005'
directory_id: '7021712771402661894'
title: 常见问题
full_path: /uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval-common-problem/approval-events-faq
breadcrumb:
- Server API
- Approval
- Approval Events
- FAQs
document_type: GuideDocumentType
updated_at: 2023-02-15T03:03:47Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval-common-problem/approval-events-faq
---

# 常见问题

使用「Lark审批」相关API时遇到的常见问题

## 接收不到审批事件
**Q：订阅了审批事件，但是却没有收到可能得原因有哪些？**
- 如果是所有的都搜不到，可能是在订阅步骤中缺少了某一步，请重新按照步骤操作一下。
- 如果是某一条收不到，可以到 开发者后台 - 日志检索 中查看一下事件日志检索，可能是回调地址无法访问。
- 由于同一个审批事件在一个审批实例是有序的，例如如果监听了审批事件，如果收到了审批实例A的“审批实例状态变更”的“PENDING”消息，未及时返回（请查看事件的推送周期和频次），那么开放平台将不会继续发送审批实例A的其他“审批实例状态变更”消息。
- 如果开发者同时在使用多个应用（例如测试应用和线上应用），可能是只在测试应用订阅了，未在线上应用订阅。
