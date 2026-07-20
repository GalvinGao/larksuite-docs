---
document_id: '7270779605450522630'
directory_id: '7270719284443660293'
title: BlockRef
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- BlockRef
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef
---

# BlockRef
Block 的引用对象，通过 BlockId 来引用，或通过 parentRef + index 来引用。
> 请不要直接用读取 BlockRef 的属性，因为 BlockRef 只代表了引用而非实际数据，它的某些属性可能是空的。如果需要读取 Block 数据，请通过 DocMiniApp.Block.getBlock 接口拿到实际数据。

**名称**    | **数据类型**                                                                                                     | **是否必填** | **描述**                 |
| --------- | ------------------------------------------------------------------------------------------------------------ | -------- | ---------------------- |
| docRef    | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 对应文档的引用                |
| blockId   | [BlockId](/document/uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/blockid)     | 否        | 引用的 Block id           |
| parentRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef)                                     | 否        | 父 Block 的引用            |
| index     | number                                                                                                       | 否        | 引用 Block 在 父 Block 的位置
