---
document_id: '7493459220907819014'
directory_id: '7077912803110010885'
title: Lark 开放平台事件与回调新增出口 IP
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/new-outbound-ips-for-events-and-callbacks
breadcrumb:
- Developer Guides
- Platform Notices
- Breaking change
- New Outbound IPs Added for Lark Open Platform Events and Callbacks
document_type: GuideDocumentType
updated_at: 2025-04-24T09:44:50Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/new-outbound-ips-for-events-and-callbacks
---

# Lark 开放平台事件与回调新增出口 IP
## 变更事项

**事件与回调**将于 **2025年5月26日** 在原出口 IP 不变的前提下，新增两个出口 IP：
- 52.77.167.223
- 54.251.9.210
  

## 潜在影响

已配置[事件订阅](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)或者[回调订阅](/document/uAjLw4CM/ukTMukTMukTM/event-subscription-guide/callback-subscription/callback-overview)的应用。已经在本地配置了网络安全组的应用，可能会因为网络拦截导致丢失部分事件与回调的请求。
  
## 解决方案

Lark 开放平台向订阅应用事件或者回调的服务器发送请求时，是通过特定 IP 发送出去的，已经在本地配置了网络安全组的开发者，请提前修改安全组策略，放行以上新增的出口 IP，以避免事件或回调被防火墙拦截。可以从[获取事件出口 IP](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-v1/outbound_ip/list)接口获取 Lark 开放平台所有的出口 IP。
