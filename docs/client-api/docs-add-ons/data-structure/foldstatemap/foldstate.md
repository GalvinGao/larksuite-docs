---
document_id: '7270779605451210758'
directory_id: '7270719284443447301'
title: FoldState
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/FoldState
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- FoldStateMap
- FoldState
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/FoldState
---

# FoldState
Block 的折叠信息
| **名称**          | **数据类型** | **是否必填** | **描述**                                      |
| --------------- | -------- | -------- | ------------------------------------------- |
| closestFolderId | number   | 否        | 被折叠 heading/list 或者 parent 拥有，指向 block id |
| folded          | boolean  | 否        | heading/list 类型 block record 中的 folded 字段   |
| foldedBy        | number   | 否        | 被谁折叠了，指向折叠 heading、list的 block id           |
| foldedByType    | string   | 否        | 被谁折叠了，指向折叠 heading、list的 block type         |
| id              | number   | 是        | block id                                    |
| show            | boolean  | 是        | 是否展示，没有被折叠的内容，show 为 true                   |
| type            | string   | 是        | block type
