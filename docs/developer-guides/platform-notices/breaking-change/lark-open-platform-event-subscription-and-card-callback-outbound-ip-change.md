---
document_id: '7683803285446266293'
directory_id: '7077912803110010885'
title: Lark 开放平台事件订阅与卡片回调出口 IP 变更
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/event-subscription-and-card-callback-outbound-ip-change
breadcrumb:
- Developer Guides
- Platform Notices
- Breaking change
- Lark Open Platform Event Subscription and Card Callback Outbound IP Change
document_type: GuideDocumentType
updated_at: 2026-09-16T11:45:24Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/event-subscription-and-card-callback-outbound-ip-change
---

# Lark 开放平台事件订阅与卡片回调出口 IP 变更

## 变更事项
事件与回调将于 2026年9月20日中午 12:00 (UTC+8) 变更出口 IP：

### 变更后的出口 IP 清单：
- 3.225.193.57
- 34.196.227.34
- 123.58.10.238
- 123.58.10.239
- 220.243.131.172
- 220.243.131.173
- 101.126.59.7
- 101.126.59.8
- 101.126.59.9
- 52.77.167.223
- 54.251.9.210
- 147.154.83.101
- 130.35.74.238
- 130.35.80.240
- 130.35.76.80
- 30.35.51.132
- 130.35.149.114
- 129.80.92.158
- 130.35.120.87
- 130.35.215.44
- 64.181.151.44
- 64.181.139.105
- 64.181.142.91
- 131.186.14.180
- 194.195.195.232
- 194.195.195.201

## 潜在影响
已配置[事件订阅](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-subscription-configure-/choose-a-subscription-mode/send-notifications-to-developers-server)或者[处理卡片回调](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/handle-card-callbacks)的应用。已经在本地配置了网络安全组的应用，可能会因为网络拦截导致丢失部分事件与回调的请求。

## 解决方案
Lark 开放平台向订阅应用事件或者回调的服务器发送请求时，是通过特定 IP 发送出去的，已经在本地配置了网络安全组的开发者，请提前修改安全组策略，放行以上新增的出口 IP，以避免事件或回调被防火墙拦截。可以从[获取事件出口 IP](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-v1/outbound_ip/list)接口获取 Lark 开放平台所有的出口 IP。
