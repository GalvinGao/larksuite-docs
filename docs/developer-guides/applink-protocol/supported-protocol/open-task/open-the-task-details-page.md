---
document_id: '7348410643215679493'
directory_id: '7073460768595214341'
title: 打开任务详情页
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-todo/open-the-task-details-page
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open Task
- Open the task details page
document_type: GuideDocumentType
updated_at: 2024-03-26T03:09:39Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-todo/open-the-task-details-page
---

# 打开任务详情页
::: note 
从Lark 4.1.0 版本开始支持。
:::

## 使用场景
唤起任务详情页。<br>
**什么是「任务」**：任务是一个轻量级的团队任务管理工具，可有效帮助成员：集中管理任务、追踪任务进度、推进团队协作。

## 协议
[https://applink.larksuite.cn/client/todo/detail](https://applink.larksuite.cn/client/todo/detail)

## 参数
| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|guid|是         |全局唯一的taskId（global unique ID）,通过[任务的 OpenAPI](/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/overview) 获取|
|mode|否         |默认在im场景下，打开任务详情页面；<br>`mode=app`, 在任务tab中打开详情页面|

## 使用示例
1. 打开 guid=xxx 且 mode=app 的任务

`https://applink.larksuite.cn/client/todo/detail?guid=xxx&mode=app`
