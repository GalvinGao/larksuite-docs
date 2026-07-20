---
document_id: '7026663896463654917'
directory_id: '7005451235847634950'
title: 如何获取不同的用户 ID
full_path: /home/user-identity-introduction/open-id
breadcrumb:
- Developer Guides
- Platform Introduction
- Basic Concepts
- User Identification
- How to get ID
document_type: GuideDocumentType
updated_at: 2024-02-08T08:15:37Z
source_url: https://open.larksuite.com/document/home/user-identity-introduction/open-id
---

# 如何获取 User ID、Open ID 和 Union ID？

本文介绍获取用户 ID 最便捷的两种方式。

## 方法一：管理后台查看 User ID

:::note
适用于具备**租户管理员**权限的用户。
:::

登录 [管理后台](https://www.larksuite.com/admin) ，在 **组织架构** > **成员与部门** 中查看用户详情，字段 **用户 ID** 的值即为 User ID。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/01aca24cc32f1df6ad53607f3e2ad5ef_M35F6QofdY.png?height=1530&lazyload=true&maxWidth=600&width=2882)

## 方法二：调试台调用接口获取三种 ID
  
通过调用 [通过手机号或邮箱获取用户 ID](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id) 接口，你可基于 **手机号**/**邮箱** 批量获取三种类型的用户 ID。

### 前提条件

- 已创建应用
- 已开通「**通过手机号或邮箱获取用户 ID**」权限
- 若需获取 User ID，需另外开通「**获取用户 User ID**」权限。

:::note
如何开通权限，参考 [如何为应用申请所需权限](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-the-99991672-error#0acf431b)。
:::
  
### 操作步骤

1. 打开 [API 调试台](https://open.larksuite.com/api-explorer/cli_a278b89588fb100d?apiName=batch_get_id&from=op_doc_tab&project=contact&resource=user&version=v3)，在左侧 API 目录中找到「**通讯录**」下的「**通过手机号或邮箱获取用户 ID**」，点击该 API 切换当前调试 API 为「通过手机号或邮箱获取用户 ID」。

	可以在 API 列表顶部的搜索框输入「通过手机号或邮箱获取用户 ID」来快速定位。

2. 点击 API 调试台左侧 **查看鉴权凭证** 中 tenant_access_token 中的 **点击获取**。

	如果之前获取过的 token 已经失效，可点击刷新图标获取新的鉴权凭证。

3. 点击右侧参数列表，将 **查询参数** 中的 **user_id_type** 参数设置为需要获取的 ID 类型。

	支持 user_id、open_id 和 union_id 三种类型。
    
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/01d96f77d423d00e4aebe1af55e8f505_9tRL1cni0Z.png?height=1350&lazyload=true&maxWidth=600&width=1722)

4. 切换至 **请求体** Tab，将请求体中的示例 ID 删除，并修改为需要查询的手机号或 Email。

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/00c1a952000cf88c4418ff2753672156_gXTNDjPRHQ.png?height=758&lazyload=true&maxWidth=600&width=956)

5. 点击右侧 **开始调试**，调用成功后，在下方**响应体**中即可获取到查询的 User ID。
响应体中返回的用户 ID 类型由查询参数中设置的 **user_id_type** 参数决定。

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1e5fcd10601df75bd895d0a7830fe63f_STy2AeQlV5.png?height=1140&lazyload=true&maxWidth=600&width=1828)
