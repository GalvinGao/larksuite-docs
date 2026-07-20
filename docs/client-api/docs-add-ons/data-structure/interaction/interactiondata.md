---
document_id: '7270779605451309062'
directory_id: '7270719284443168773'
title: InteractionData
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/InteractionData
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- Interaction
- InteractionData
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/InteractionData
---

# InteractionData

InteractionData 采用Map结构，key为string | number类型， value可以为任意类型。

```js
export interface InteractionData {
    [key: string | number]: any;
}
```
