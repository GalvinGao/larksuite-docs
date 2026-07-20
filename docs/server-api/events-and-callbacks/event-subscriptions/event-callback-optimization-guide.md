---
document_id: '7280101141710913541'
directory_id: '7394375283862142981'
title: 事件回调优化指南
full_path: /uAjLw4CM/ukTMukTMukTM/event-subscription-guide/event-callback-optimization-guide
breadcrumb:
- Server API
- Events and callbacks
- Event subscriptions
- Event Callback Optimization Guide
document_type: GuideDocumentType
updated_at: 2023-11-13T03:10:22Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/event-subscription-guide/event-callback-optimization-guide
---

# 事件回调优化指南
## 为什么我会收到通知？

为了帮助开发者更好的使用Lark开放能力，Lark开放平台为开发者提供了事件回调健康度监测能力。对于事件回调成功率不足 100 % 的情况下，将会通过开发者小助手推送卡片消息，提醒开发者优化服务。
收到通知是因为你的事件回调在过去一周里，出现了事件回调四次重试均未成功处理的情况。

## 优化策略

你可以参考以下策略优化你的事件回调服务。

  
1.**使用位于中国大陆的服务器**

Lark事件回调服务需要你在 3 秒内恢复 HTTP 200 状态码用以判断请求是否成功。使用位于中国大陆的服务器可以有效减少在链路上所消耗的时间，为服务内部处理逻辑提供余量，从而减少你的服务器返回响应超时的可能性。

2. **将耗时操作异步处理**

对于某些耗时的操作，你可能无法在 3 秒内回复请求，从而导致事件回调认为请求失败，再次请求服务。你可以将某些耗时操作转换为异步处理，先返回 HTTP200，并在随后的异步逻辑中处理请求。

3. **退订不再使用的事件回调**

订阅过多事件，会导致你的事件回调服务处理较多的请求，退订不再需要的事件，可以帮助你减少服务处理的事件数量，降低服务的负载，从而提升系统整体的处理性能。
