---
document_id: '7270779605450997766'
directory_id: '7270719284443693061'
title: RecordChangeset
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/RecordChangeset
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- Record
- RecordChangeset
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/RecordChangeset
---

# RecordChangeset
Record 变更数据，是下面三种类型之一：
- insert 类型，表示在某个路径下插入新的数据

| **名称** | **数据类型**              | **是否必填** | **描述**               |
| ------ | --------------------- | -------- | -------------------- |
| type   | 'insert'              | 是        | insert 类型的 changeset |
| data   | object                | 是        | changeset 数据         |
| ∟path  | (string \| number)[] | 是        | insert 的路径           |
| ∟value | any                   | 是        | insert 的数据           |
- remove 类型，表示将某个路径下的数据删除

| **名称** | **数据类型**              | **是否必填** | **描述**               |
| ------ | --------------------- | -------- | -------------------- |
| type   | 'remove'              | 是        | remove 类型的 changeset |
| data   | object                | 是        | changeset 数据         |
| ∟path  | (string \| number)[] | 是        | remove 的路径           |
- replace 类型，表示将某个路径下的数据替换成新数据

| **名称** | **数据类型**              | **是否必填** | **描述**                |
| ------ | --------------------- | -------- | --------------------- |
| type   | 'replace'             | 是        | replace 类型的 changeset |
| data   | object                | 是        | changeset 数据          |
| ∟path  | (string \|  number)[] | 是        | replace 的路径           |
| ∟value | any                   | 是        | replace 的数据
