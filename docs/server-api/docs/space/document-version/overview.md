---
document_id: '7242158539798855686'
directory_id: '7239713799464452102'
title: 概述
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/overview
breadcrumb:
- Server API
- Docs
- Space
- Document Version
- Overview
document_type: GuideDocumentType
updated_at: 2024-03-26T06:08:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/overview
---

# 概述
## 功能简介

支持用户手动进行生成文档版本，并针对这些版本进行高效管理，主要包括：
- 创建版本：添加一个新的文档版本。
- 管理版本：可删除文档版本。
- 版本阅读：获取版本文档内容。

## 资源定义


| 字段名          | 类型     | 描述        | 备注                                                                  |
| ------------ | ------ | --------- | ------------------------------------------------------------------- |
| `name`         | `string` | 版本文档标题    | -- |                                                                    |
| `version`      | `string` | 版本文档版本号   | -- |                                                                     |
| `parent_token` | `string` | 源文档token  | -- |                                                                      |
| `owner_id`     | `string` | 版本文档所有者id | -- |                                                                    |
| `creator_id`   | `string` | 版本文档创建者id | -- |                                                                      |
| `create_time`  | `int`  | 版本文档创建时间  | -- |                                                                      |
| `update_time`  | `int`  | 版本文档更新时间  | -- |                                                                      |
| `status`       | `string` | 版本文档状态    | 枚举值：<br>-   `StatusExist`：正常状态<br>-   `StatusDeleted`：删除状态<br>-   `StatusTrash`：回收站状态 |
| `obj_type`     | `string` | 版本文档类型    | 枚举值，目前支持 docx、sheet                                      |
| `parent_type`  | `string` | 源文档类型     | 枚举值，目前支持 docx、sheet                                            |
