---
document_id: '7176841141353512966'
directory_id: '7161758872830820357'
title: 如何获取自己的 Union ID
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id
breadcrumb:
- Developer Guides
- FAQ
- Trouble Shooting
- How to Obtain Union ID
document_type: GuideDocumentType
updated_at: 2024-03-18T02:16:17Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id
---

# 如何获取自己的 Union ID？


### 前提条件

- [已开通「通过手机号或邮箱获取用户 ID」权限（`contact:user.id:readonly`）](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-the-99991672-error)

### 1. 打开 API 调试台，并找到「通过手机号或邮箱获取用户 ID」API

打开Lark开放平台 [API 调试台](https://open.larksuite.com/api-explorer?from=op_doc_tab)，并在左侧 API 目录中找到「通讯录」下的「通过手机号或邮箱获取用户 ID」，点击该 API 切换当前调试 API 为「通过手机号或邮箱获取用户 ID」。

> 可以在API 列表顶部的搜索框输入「通过手机号或邮箱获取用户 ID」来快速定位。

![CN01_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b1c06c5a4141d32aa1819c0c4d66284a_xrQXjdigBS.png?lazyload=true&width=3050&height=1413)

### 2. 获取鉴权凭证，并设置参数

1. 点击 API 调试台左侧「查看鉴权凭证」中 tenant_access_token 中的「点击获取」（如果之前已经获取过，则可以点击刷新按钮刷新鉴权凭证。

2. 点击右侧参数列表，将查询参数 Tab 中的 *user_id_type* 参数设置为 *union_id*。

3. 切换至请求体 Tab，将请求体中的 ID 删除，并修改 *mobiles* 参数，设为你自己的手机号。

![CN02_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f63f40991eb8909a399ea26acc731b65_xUyBoNsKqC.png?lazyload=true&width=3056&height=1460)

### 3. 调试，并获得 Union ID

点击右侧「开始调试」，调用接口。调用成功后，在下方响应体中即可拿到你自己的 Union ID。

![CN03_副本.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eb0f485e2ea1f00d5b7d78a30264fc45_H0uTtoavts.png?lazyload=true&width=2982&height=1360)
