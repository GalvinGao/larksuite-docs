---
document_id: '7270779605451063302'
directory_id: '7270719284443660293'
title: ExtendedSelectionItem
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/extendedselectionitem
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- ExtendedSelectionItem
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/extendedselectionitem
---

# ExtendedSelectionItem
扩展选区信息，可以分为块级选区信息和文本选区信息。
| **名称**        | **数据类型**            | **是否必填** | **描述**                                           |
| ------------- | ------------------- | -------- | ------------------------------------------------ |
| type          | block \| text   | 是        | 选区类型，block 表示块选区，text 表示文本选区                     |
| ref           | BlockRef \| TextRef | 是        | 选区对应的 block 引用，BlockRef 表示 Block 的引用，Text 表示文本引用 |
| blockId       | BlockId             | 是        | 选区项的 Block id                                    |
| blockSnapshot | BlockSnapshot       | 是        | 选区项的 Block Snapshot
