---
document_id: '7132687771717926917'
directory_id: '7072190414392344582'
title: API 常见问题
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/wiki-qa
breadcrumb:
- Server API
- Docs
- Wiki
- API FAQ
document_type: GuideDocumentType
updated_at: 2022-08-17T03:51:23Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/wiki-qa
---

# API 常见问题

## 如何读取、编辑节点内容
Wiki 的节点支持多种文档类型，包括文档、电子表格、多维表格、思维笔记和文件等等。你可以通过 API，对部分类型的节点进行编辑，目前支持通过 API 编辑的类型有：文档、电子表格、多维表格。

### 操作步骤
通过调用 [获取节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node) 接口，可以从返回值中获取到 `obj_type` 和 `obj_token`。
根据节点的类型，`obj_type` 可能是 doc、sheet、bitable 中的一种。`obj_token` 则是对应的文档唯一标识，你可以通过对应的 token，分别调用各业务的接口，进行内容的增删改查。详情可参考：
[云文档常见问题](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)。



## 如何使用 tenant_access_token 调用知识库 API
::: html
<md-td>知识库 API 中，除了 [创建知识库](/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/create) 以外，大部分接口都支持使用 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag> 进行调用。</md-td>
:::

### 操作步骤
::: html
<md-td>在使用 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag> 之前，请确保这个应用已经被添加为知识库成员之一，否则会返回无权限错误。

目前唯一可将应用添加为知识库成员的方法为：
- 获取到任一有权限添加知识库成员的知识库管理员的 <md-tag mode="inline" type="token-user">user_access_token</md-tag>
- 使用该 token 调用 [添加知识空间成员](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-member/create) 接口，将应用添加为知识库的成员之一。并确保该应用作为知识库成员，有权限获取知识库的节点列表和节点内容
- 确保上述步骤正确完成后，你可以使用 <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag> 调用知识库的相关接口
</md-td>
:::
