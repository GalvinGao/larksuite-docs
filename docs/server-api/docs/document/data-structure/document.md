---
document_id: '7281558204885876742'
directory_id: '7281236599571759109'
title: 文档
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/data-structure/document
breadcrumb:
- Server API
- Docs
- Document
- Data Structure
- Document
document_type: GuideDocumentType
updated_at: 2023-09-22T08:04:27Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/data-structure/document
---

# Document

Document 表示一篇新版文档。多个 Block 间可以形成树状关系的层级内容，从结构来看新版文档就是一颗 Block 树，在 OpenAPI 中将这颗树称为 Document。

```json
"document": {
    "document_id": string,
    "revision_id": int,
    "title": string
}
```

| 名称         | 数据类型           | 属性        | 默认值         | 描述        |
| --------- | --------------- |  :-:   |  :-: | --------- |
|`document_id` | `string` | required |`-`| **文档唯一标识**，其也是文档根 `Block` 的 ID。 |
|`revision_id` | `int` | required |`-`| **文档版本 ID**，用以指定要查询或更新的文档版本。<br>如果多次调用接口，其返回版本 ID 未更改，则表示文档未更改。相反，版本 ID 发生更改通常意味着文档已更新， 但需要注意的是 ID 变更并非一定是文档内容发生变化，比如也有可能是文档被他人评论所致。 |
|`title` | `string` | required |`-`| **文档标题**，`Document` 只支持返回纯文本。 |
