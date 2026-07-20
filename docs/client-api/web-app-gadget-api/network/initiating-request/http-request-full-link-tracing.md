---
document_id: '7073691561007955973'
directory_id: '6907567266536456193'
title: HTTP请求全链路追踪
full_path: /uYjL24iN/ugDNugDNugDN/http-request-full-link-tracing
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- initiating Request
- HTTP request full link tracing
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:09Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDNugDNugDN/http-request-full-link-tracing
---

# HTTP请求全链路追踪
> 用于小程序所有 HTTP 请求的端到端链路追踪方式

::: note
可用版本：4.7+
:::
:::note
`x-request-id`、`x-tt-logid`作为小程序的保留header，如果使用，可能会导致header内容的覆盖。
:::

小程序在使用`tt.request`、`tt.uploadFile`、`tt.downloadFile`时，引擎会自动生成一个唯一的**请求ID**，并设置到请求header `x-request-id`、`x-tt-logid`内发送到服务端。

在上述接口的`SuccessCallback`、`FailCallback`、`Result`，均会新增字段`trace`，value为**请求ID**。


## 最佳实践
- 对于接收小程序request请求的后端服务，将header内的`x-request-id`作为上下文，用于问题的诊断和全链路追踪。
- 对于小程序业务，在自建的网络请求埋点或日志中，加入返回的`trace`，用于问题的诊断和全链路追踪。
- 当问题发生时，可根据客户端日志、埋点或业务日志、埋点或后端日志、埋点进行请求ID的查询与诊断。




