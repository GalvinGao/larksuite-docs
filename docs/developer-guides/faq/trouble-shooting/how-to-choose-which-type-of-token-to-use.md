---
document_id: '7166859792995991558'
directory_id: '7161758872830820357'
title: 如何选择使用哪种类型的 Token
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use
breadcrumb:
- Developer Guides
- FAQ
- Trouble Shooting
- How to choose which type of Token to use
document_type: GuideDocumentType
updated_at: 2023-08-01T03:16:06Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use
---

# 如何选择使用哪种类型的 Token

在Lark开放平台上调用 OpenAPI 时，对于部分接口会同时支持 Tenant Access Token 和 User Access Token。关于不同类型的 Token ，可以查看 [获取访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) 的详细说明。

## Tenant Access Token

Tenant Access Token 代表使用**应用的身份**操作 OpenAPI，API 所能操作的数据资源范围受限于**应用的身份所能操作的资源范围**。

如果你的业务逻辑不需要操作用户的数据资源，仅需操作应用自己拥有的资源（比如在应用自己的文档目录空间下创建云文档），则推荐使用 Tenant Access Token，无需额外申请授权。


## User Access Token

User Access Token 代表使用**应用的使用者的身份**操作 OpenAPI，API 所能操作的数据资源范围受限于**用户的身份所能操作的资源范围**。


如果你的业务逻辑需要操作用户的数据资源（例如需要在用户的文档目录空间下创建云文档），则推荐使用 User Access Token，无需额外申请授权。如果使用 Tenant Access Token，则需额外在资源层面为应用添加相应的授权。
