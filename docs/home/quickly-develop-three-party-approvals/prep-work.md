---
document_id: '7139727755097669637'
directory_id: '7137610938320961542'
title: 准备工作
full_path: /home/quickly-develop-three-party-approvals/prep-work
breadcrumb:
- Home
- Quickly develop three-party approvals
- Prep work
document_type: GuideDocumentType
updated_at: 2023-05-15T02:36:20Z
source_url: https://open.larksuite.com/document/home/quickly-develop-three-party-approvals/prep-work
---

# 准备工作

在当前这个业务场景下，你可能会使用到以下能力资源。

##   了解相关 OpenAPI

:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 50%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 25%;">权限要求</md-th>

<md-th style="width: 25%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)**</md-td></md-th>


</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[三方审批定义创建](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/create)</md-text>

`POST` /open-apis/approval/v4/external_approvals
  

</md-td>

<md-td>

 <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
</md-tr>
 <md-tr>

<md-td>

<md-text type="field-name" >[三方审批实例同步](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create)</md-text>

`POST` /open-apis/approval/v4/external_instances
  

</md-td>

<md-td>

 <md-perm name="approval:approval" desc="查看、创建、更新、删除审批应用相关信息" support_app_types="custom,isv" tags="">查看、创建、更新、删除审批应用相关信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
</md-tr>
  <md-tr>

<md-td>

<md-text type="field-name" >[三方审批实例校验](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/check)</md-text>

`POST` /open-apis/approval/v4/external_instances/check
  

</md-td>

<md-td>

  <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
</md-tr>
 <md-tr>

<md-td>

<md-text type="field-name" >[获取三方审批任务状态](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_task/list)</md-text>

`POST` /open-apis/approval/v4/external_tasks
  


</md-td>

<md-td>

 <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
</md-tr>
  
<md-tr>

<md-td>

<md-text type="field-name" >[发送审批Bot消息](/document/ukTMukTMukTM/ugDNyYjL4QjM24CO0IjN)</md-text>

`POST` /approval/openapi/v1/message/send
  


</md-td>

<md-td>

 <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>


</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
</md-tr>
  <md-tr>

<md-td>

<md-text type="field-name" >[更新审批Bot消息](/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN)</md-text>

`POST` /approval/openapi/v1/message/update
  


</md-td>

<md-td>

  <md-perm name="approval:approval:readonly" desc="访问审批应用" support_app_types="custom,isv" tags="">访问审批应用</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
</md-tr>
  
  </md-tbody>

</md-table>

:::



## 了解API调试台

为了方便开发者测试和调用各类接口，Lark提供了基于[API调试台](https://open.larksuite.com/api-explorer)工具.通过API调试台，你无需编写代码，即可快捷完成服务端的接口调用。

此外，我们提供了以下几点功能，提升你的接口调试效率：

- **自动获取鉴权凭证**：可一键获取应用token，每次调试无需再额外发起请求获取。
- **内置应用权限申请**：调试接口所需权限一目了然，无需跳转开发者后台，在API调试台内即可快捷申请。
- **接口调试示例代码**：提供多语言示例代码，点击复制即可快速复用至业务代码中。

![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/96ea605deef8b9d4ac3111b86a3e4b68_wTC28DeYWs.gif?lazyload=true&width=1100&height=510)
