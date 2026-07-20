---
document_id: '7073823165957701638'
directory_id: '7073460768595214341'
title: 打开任务tab页
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-todo/open-the-task-tab-page
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open Task
- Open the task tab page
document_type: GuideDocumentType
updated_at: 2022-03-11T16:42:53Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-todo/open-the-task-tab-page
---

# 打开任务tab页
::: note 
从Lark 4.9.0 版本开始支持。
:::

## 使用场景
唤起任务tab页，跳转到指定tab。

**什么是「任务」**：任务是一个轻量级的团队任务管理工具，可有效帮助成员：集中管理任务、追踪任务进度、推进团队协作。
## 协议
`https://applink.larksuite.com/client/todo/view`

##  参数
| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|tab|是         |String类型，枚举值包括：<br> `all` 进行中；<br> `assign_to_me` 由我处理；<br> `assign_from_me` 我分配的；<br> `followed`  我关注的；<br> `completed` 已完成|

## 使用示例
####  1. 打开任务的“进行中”tab页

[https://applink.larksuite.com/client/todo/view?tab=all](https://applink.larksuite.com/client/todo/view?tab=all)

#### 2. 打开任务的“我分配的”tab页

[https://applink.larksuite.com/client/todo/view?tab=assign_from_me](https://applink.larksuite.com/client/todo/view?tab=assign_from_me)

#### 3. 打开任务的“我关注的”tab页

[https://applink.larksuite.com/client/todo/view?tab=followed](https://applink.larksuite.com/client/todo/view?tab=followed)


