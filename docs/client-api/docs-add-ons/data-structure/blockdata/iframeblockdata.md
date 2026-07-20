---
document_id: '7270779605450424326'
directory_id: '7270719284443430917'
title: IframeBlockData
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/IframeBlockData
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- BlockData
- IframeBlockData
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/IframeBlockData
---

# IframeBlockData
内嵌网页 Block 的数据结构，继承于 [BlockData](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/BlockData/blockdata)

| **名称**    | **数据类型** | **是否必填** | **描述**                 |
| --------- | -------- | -------- | ---------------------- |
| component | object   | 是        | /                      |
|  ∟type    | string   | 是        | Iframe 网页类型            |
|  ∟url     | string   | 是        | 目标网页 URL，该值是 encode 过的
