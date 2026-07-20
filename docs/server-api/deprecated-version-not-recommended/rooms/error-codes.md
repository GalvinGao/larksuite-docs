---
document_id: '6967251478324330502'
directory_id: '6956136387256090630'
title: 会议室错误码
full_path: /ukTMukTMukTM/uMDOyUjLzgjM14yM4ITN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Rooms
- Error Codes
document_type: GuideDocumentType
updated_at: 2022-03-10T12:45:51Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uMDOyUjLzgjM14yM4ITN
---

# 会议室错误码

请求失败时，接口会返回非 0 错误码 code，含义及错误提示信息如下：

| **错误码** | **说明**                                                     | **错误信息**                             |
| ---------- | ------------------------------------------------------------ | ---------------------------------------- |
| 100001     | page token 格式非法                                          | page token invalid                       |
| 100002     | fields 中存在非法字段名                                      | Invalid field selection [非法 field]     |
| 100003     | 时间格式未遵循 [RFC3339](https://tools.ietf.org/html/rfc3339) 标准 | time format must follow RFC3339 standard |
| 100004     | building ID 非法                                             | building id invalid                      |
| 100005     | room ID 非法                                                 | room id invalid                          |
| 105001     | 内部错误                                                     | internal error                           |

