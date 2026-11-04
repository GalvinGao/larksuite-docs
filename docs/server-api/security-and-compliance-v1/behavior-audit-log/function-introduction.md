---
document_id: '7291504938512793605'
directory_id: '7289368200171503622'
title: 功能介绍
full_path: /ukTMukTMukTM/uQjM5YjL0ITO24CNykjN/audit_log/
breadcrumb:
- Server API
- security_and_compliance
- Behavior audit log
- Function introduction
document_type: GuideDocumentType
updated_at: 2025-09-03T07:37:26Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQjM5YjL0ITO24CNykjN/audit_log/
---

# 功能介绍
:::warning
此功能内测中，仅限企业旗舰版申请，如需申请，请联系客户成功经理。
:::


企业内部审计工作应根据国家法律法规开展，Lark提供通知功能，为企业根据明确的审计政策和程序，向被审计单位或者被审计人员送达通知书。企业应该严格控制审计权限，仅授予恰当的审计人员以审计管理员的权限（[管理员创建管理员角色及分配权限](https://www.larksuite.com/hc/en-US/articles/360043595213)）；仅授予恰当的应用以审计API的权限（[申请 API 权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)）。

## 概述
行为审计API是开放平台基于Lark用户的行为操作事件而开放的API，用户可以使用该API，与自建应用或SIEM类工具对接，对组织内成员的敏感使用行为进行审计和统计分析；以确保组织成员使用时的安全合规，追溯安全事件。


## 行为审计API可以做什么
- 本API仅提供用户的行为操作日志，**不包括管理员行为日志，以及对消息内容/云文档内容等内容的审计能力**
- 本API可以查询组织成员在Lark内实际发生的审计日志，因此是只读的，没有用于行为审计事件的写入方法
- 本API所支持的数据是所有Lark内的行为事件的子集，我们将持续增加对更多行为事件和事件信息的支持，以满足更广范围的审计需求



## 名词解释
### 行为审计日志
一条行为审计日志，记录了【谁】在什么【时间】，什么【环境】下，对什么【对象】进行了什么【操作】。


## 限制策略
- 保留期限：行为审计日志最多支持最近180天内的查询
- 查询条件限制：最多支持30天跨度时间范围的查询请求
