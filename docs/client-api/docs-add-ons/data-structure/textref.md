---
document_id: '7270779605451571206'
directory_id: '7270719284443660293'
title: TextRef
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/TextRef
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- TextRef
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/TextRef
---

# TextRef
文本的引用对象。

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| blockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是 | 对应的 Block 引用 |
| range | [number, number] | 是 | 文本范围，假设数组为 [start, end]，则：<br>- start > end 时，是非法值，会降级为 [start, start]<br>- start = end 时，表示文本位置<br>- start < end 时，表示文本范围<br>start 和 end 超出 Block 的文本内容时，则设置为文本内容的最后位置。<br></md-td> |

