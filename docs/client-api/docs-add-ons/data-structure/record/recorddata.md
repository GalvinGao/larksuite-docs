---
document_id: '7270779605450719238'
directory_id: '7270719284443693061'
title: RecordData
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/RecordData
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- Record
- RecordData
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/RecordData
---

# RecordData
RecordData 采用Map结构，key为string | number类型， value可以为任意类型。
```js
export interface RecordData {
    [key: string | number]: any;
}
```
