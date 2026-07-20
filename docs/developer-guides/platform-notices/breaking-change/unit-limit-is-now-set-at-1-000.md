---
document_id: '7260769267377291270'
directory_id: '7077912803110010885'
title: 单位数量上限设定为1000个
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/unit-limit-is-now-set-at-1000
breadcrumb:
- Developer Guides
- Platform Notices
- Breaking change
- Unit limit is now set at 1,000
document_type: GuideDocumentType
updated_at: 2023-08-01T02:58:46Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/breaking-change/unit-limit-is-now-set-at-1000
---

# 单位数量上限设定为1000个

### 变更事项

为了提供更稳定的系统服务，我们将约束单租户可以创建的单位数量上限，限制为不超过1000个。

是否跟随客户端版本：不跟版

预计生效时间：2023-08-15

### 潜在影响

若你的应用创建了超过1000个单位时，将会创建失败，并收到对应的错误码。

### 解决方案

删除不必要的单位，将单位数量控制在1000个以内。
