---
document_id: '7069302182251888645'
directory_id: '7069296428547686405'
title: 身份验证相关
full_path: /ugTN1YjL4UTN24CO1UjN/uMzN1YjLzcTN24yM3UjN
breadcrumb:
- Developer Guides
- FAQ
- Server-side develop
- Questions on authentication
document_type: GuideDocumentType
updated_at: 2022-02-27T08:23:14Z
source_url: https://open.larksuite.com/document/ugTN1YjL4UTN24CO1UjN/uMzN1YjLzcTN24yM3UjN
---

# 身份验证相关问题
**1. 小程序的code可以在服务端使用吗？**

答：小程序的 code（通过[请求身份验证](/document/ukTMukTMukTM/ukzN4UjL5cDO14SO3gTN)获得）不可以在服务端接口（如[获取登录用户身份](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/authen/access_token)）里使用。

**2. 如何通过Lark，实现应用免登？**

答：请见[参考文档](/document/ukTMukTMukTM/uETOwYjLxkDM24SM5AjN)，查看如何实现应用免登。

**3. 通过Lark进行应用免登的时候，出现登录失败怎么办？**

答：如果提示的信息是“请求非法”，请**检查对应的重定向地址正确，并确认其配置在应用的重定向 URL 选项里**。如果提示的信息是“无应用权限”，建议**检查对应的用户有没有使用该应用的权限**。

