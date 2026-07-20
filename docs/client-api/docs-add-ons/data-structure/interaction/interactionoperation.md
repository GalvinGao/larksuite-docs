---
document_id: '7270779605450932230'
directory_id: '7270719284443168773'
title: InteractionOperation
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/InteractionOperation
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- Interaction
- InteractionOperation
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/InteractionOperation
---

# InteractionOperation
Interaction 变更数据op
| **名称** | **数据类型**                 | **是否必填** | **描述**                      |
| ------ | ------------------------ | -------- | --------------------------- |
| p:     | InteractionOperationPath | 是        | Interaction Operation 的操作路径 |
| action | object                   | 否        | Interaction Operation 的操作   |
| ∟li    | any                      | 否        | 在list中插入元素                  |
| ∟si    | any                      | 否        | 对string插入字符                 |
| ∟sd    | any                      | 否        | 删除string中字符                 |
| ∟ld    | any                      | 否        | 删除list中元素                   |
| ∟od    | any                      | 否        | 删除object中的元素                |
| ∟na    | number                   | 否        | 加上数字，如果你想减去，加上一个负数          |
| ∟lm    | number                   | 否        | 移动list中的元素                  |
Interaction Operation 的操作路径
| **名称**                   | **数据类型**              | **描述**                      |
| ------------------------ | --------------------- | --------------------------- |
| InteractionOperationPath | (string \| number)[] | Interaction Operation 的操作路径
