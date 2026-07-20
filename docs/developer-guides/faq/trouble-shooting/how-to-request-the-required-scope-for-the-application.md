---
document_id: '7166859792995975174'
directory_id: '7161758872830820357'
title: 如何为应用申请所需权限
full_path: /uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-the-99991672-error
breadcrumb:
- Developer Guides
- FAQ
- Trouble Shooting
- How to request the required scope for the application
document_type: GuideDocumentType
updated_at: 2024-03-18T02:16:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-fix-the-99991672-error
---

# 如何为应用申请所需权限？

## 步骤一：明确需要申请的权限

当你在调用接口时，出现了 99991672 报错时，Lark开放平台将通过返回结果的 `error.permission_violations` 字段告知具体所需申请的权限。

以如下报错为例，你的应用需要拥有 `im:chat:readonly` 或 `im:chat` 中任一权限，才能调用此接口。

```
{
  "code": 99991672,
  "msg": "Access denied. One of the following scopes is required: [im:chat:readonly,im:chat]",
  "error": {
    "helps": [
      {
        "url": "/ssl:ttdoc/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list",
        "description": "Learn more about scopes and how to add them: [im:chat:readonly,im:chat]"
      }
    ],
    "subcode": 20709003,
    "log_id": "xxxxxxxxx",
    "permission_violations": [
      {
        "type": "action_scope_required",
        "subject": "im:chat:readonly"
      },
      {
        "type": "action_scope_required",
        "subject": "im:chat"
      }
    ]
  }
}
```

## 步骤二：访问需要申请权限的应用

在[Lark开放平台开发者后台](https://open.larksuite.com/app)中选择需要申请权限的应用，点击进入应用管理页面。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f485799371a6baac151b42163f35630e_YtQMeH6uPU.png?height=782&lazyload=true&width=1920)

## 步骤三：申请权限

:::html
<md-alert type="tip">
按照**最小权限原则**（仅申请开发应用所必需的最小权限集合）申请所需权限，可以帮助管理员快速了解你所申请的权限和场景，加快审批节奏。
</md-alert>
:::

在应用的 **开发配置** > **权限管理** 页面，找到 **权限配置** 区域，选择权限并点击 **批量开通**。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a4fbe5ead7a6dbfb67596699ba1ab0d9_jlNpf8hHWX.png?height=842&lazyload=true&width=2870)


## 步骤四：创建版本并申请发布

:::note
出于数据安全的原因，添加完成权限后，你还不能使用权限，需要 **创建版本并通过企业管理员审核并发布后，才能使用该权限**。
:::

1. 在应用左侧导航栏中，选择 **应用发布** > **版本管理与发布**。


2. 在版本管理页，点击右上角的 **创建版本**，创建一个新的应用版本。
	
   ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/eee1e58035f85861e9b11f2c94cb8295_kMAoPj2NsH.png?height=1058&lazyload=true&width=2882)

3. 在创建应用版本的页面填写应用的基本信息和备注信息。
	
    详细的信息将有助于帮助管理员理解你的应用变更，并通过审核。


	
   ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e7d0b818da81e309f3864e86cba15957_1t6p3Sz6wr.png?height=1238&lazyload=true&width=2274)

4. 填写完成后点击 **保存**，将会自动跳转到版本详情页，确认当前版本的基本信息无误后，点击 **申请线上发布**。


	
   ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/47238c673a7433716eb1fbb50809ba67_0F2micDv2y.png?height=946&lazyload=true&width=2328)

## 步骤五：联系所在企业应用管理员通过审核

应用提交发布后，需要等待所在企业应用管理员通过审核后才能发布，你可以联系对应管理员为你通过审核。审核通过后，便可以再次尝试调用 OpenAPI。应用审核通过截图如下：


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5c47779eeff6eef4778458e7c294216e_XsFaID78zh.png?lazyload=true&width=1146&height=400)
