---
document_id: '7270779605451194374'
directory_id: '7270719284443660293'
title: BlockSnapshot
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- BlockSnapshot
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot
---

# BlockSnapshot
Block 的快照信息对象。
| **名称**         | **数据类型**                                                                         | **是否必填** | **描述**                                                                                            |
| -------------- | -------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------- |
| id             | [BlockId](/document/uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/blockid)          | 是        | Block id，[见【数据结构定义 - BlockId】](/document/uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/blockid)      |
| type           | [BlockType](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockType)        | 是        | Block 的类型，[见【数据结构定义 - BlockType】](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockType)   |
| data           | [BlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/blockdata)        | 是        | Block 的详细数据，[见【数据结构定义 - BlockData】](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/blockdata) |
| parent         | [BlockId](/document/uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/blockid)          | 否        | Block 的父 Block id                                                                                 |
| children       | [BlockId](/document/uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/blockid)[]       | 是        | Block 所有子 Block id                                                                                |
| childSnapshots | [BlockSnapshot](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockSnapshot)[] | 是        | Block 所有子 Block Snapshot                                                                          |
| childIndex     | number                                                                           | 是        | Block 所在父 Block 的索引位置                                                                             |
| recordId       | string                                                                           | 是       | Block 对应的 record id                                                                               |
| ref            | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef)       | 是        | Block 对应的 Block 引用
