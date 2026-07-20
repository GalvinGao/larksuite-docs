---
document_id: '7026663896463638533'
directory_id: '7005451235847634950'
title: 如何选择使用哪种 ID？
full_path: /home/user-identity-introduction/user-id
breadcrumb:
- Developer Guides
- Platform Introduction
- Basic Concepts
- User Identification
- How to choose ID
document_type: GuideDocumentType
updated_at: 2023-09-14T06:20:18Z
source_url: https://open.larksuite.com/document/home/user-identity-introduction/user-id
---

# 如何选择使用哪种 ID

在开发者开发一个应用时合理的 ID 选择逻辑应该如下：

- 创建的应用**不存在跨应用的数据关联互通**场景，可以使用 Open ID 。

- 需**跨应用做数据关联**，并且这些应用的开发者归属同一个组织，可以使用 Union ID。

:::note
由于在产品演进过程中可能会需要做跨应用互通，在应用开发时推荐开发者优先使用 Union ID。
:::
  
- 明确需要使用用户的租户身份标识，例如在**组织人员管理** 场景中（创建成员/组/角色），可以使用 User ID，其与通讯录业务中的 Member ID 两者含义一样。

:::note
仅支持**自建应用**申请 User ID，需先在[开发者后台](https://open.larksuite.com/app)申请「**获取用户 User ID**」权限。
:::
  
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dd5f0ff509b4b29b3d3666ffdefa4a8c_j91iiJ8U01.png?height=1610&lazyload=true&maxWidth=550&width=2772)
 
