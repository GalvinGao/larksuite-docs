---
document_id: '7152073576123744261'
directory_id: '6921376028544499713'
title: 常见问题
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/faq
breadcrumb:
- Server API
- Docs
- Space
- FAQ
document_type: GuideDocumentType
updated_at: 2022-10-08T09:37:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/faq
---

# 云空间常见问题
## 1. 云空间文件夹单层文件的个数限制。

|  |
| --- |
| 云空间中文件夹单层节点上限是1500个，超过限制[新建在线文档](/document/ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN)、[复制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy)、[移动文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/move)、[新建文件夹](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/create_folder)以及[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all)等接口会返回失败，错误码为1062507，如果有这类需求，可以考虑将文件新建在不同文件夹中。 |

  
## 2. 云空间文件接口的并发限制。

|  |
| --- |
| 云空间文件夹中不支持并发调用[新建在线文档](/document/ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN)、[复制文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy)、[移动文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/move)、[新建文件夹](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/create_folder)、[删除文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete)以及[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all)等接口，会返回失败，错误码为1061045，重试可成功，用户应该尽量避免并发调用的场景。 |

  
## 3. 如何让应用（tenant_access_token）访问个人云空间中的文件夹？

|  |
| --- |
| 需要应用启用[机器人能力](/document/home/interactive-session-based-robot/create-app-request-permission)。打开Lark软件，创建新的群组，将应用添加为群机器人。在Lark云文档，我的空间中找到对应的文件夹，将文件夹分享给刚刚新建的群组。 |

  
## 4. 如何获取文件夹、文件以及各种类型的在线文档token信息？

|  |
| --- |
| 参考[云文档常见问题](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)。 |

