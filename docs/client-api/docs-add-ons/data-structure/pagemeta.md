---
document_id: '7270779605450964998'
directory_id: '7270719284443660293'
title: PageMeta
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/PageMeta
breadcrumb:
- Client API
- Docs Add-ons
- Data Structure
- PageMeta
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/PageMeta
---

# PageMeta
文档的 Meta 信息
| **名称**               | **数据类型** | **是否必填** | **描述**      |
| -------------------- | -------- | -------- | ----------- |
| char_count           | number   | 是        | 字符数         |
| comments_count       | number   | 是        | 评论数         |
| comments_count_today | number   | 是        | 今日新增评论数     |
| create_timestamp     | number   | 是        | 文档创建时间      |
| like_count           | number   | 是        | 点赞数         |
| like_count_today     | number   | 是        | 今日新增点赞数     |
| owner_user           | object   | 是        | 文档创建者       |
|  ∟avatar_url         | string   | 是        | 用户的头像链接     |
|  ∟cn_name            | string   | 是        | 用户的中文名称     |
|  ∟en_name            | string   | 是        | 用户的英文名称     |
|  ∟id                 | string   | 是        | 用户的 user id |
| pv                   | number   | 是        | 阅读次数        |
| pv_today             | number   | 是        | 今日新增阅读次数    |
| uv                   | number   | 是        | 阅读人数        |
| uv_today             | number   | 是        | 今日新增阅读人数    |
| word_count           | number   | 是        | 单词数
