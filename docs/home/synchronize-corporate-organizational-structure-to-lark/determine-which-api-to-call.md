---
document_id: '7074952334766096390'
directory_id: '7073442394955546629'
title: 确定要调用的API
full_path: /home/synchronize-corporate-organizational-structure-to-feishu/determine-which-api-to-call
breadcrumb:
- Home
- Synchronize corporate organizational structure to Lark
- Determine which API to call
document_type: GuideDocumentType
updated_at: 2023-05-16T08:43:51Z
source_url: https://open.larksuite.com/document/home/synchronize-corporate-organizational-structure-to-feishu/determine-which-api-to-call
---

#  确定需要调用的API
一般情况下，你需要同时维护部门和用户两份数据。

在Lark初始化过程中，通常你需要先完成部门树的创建，再完成用户的创建。

在增量数据同步过程中，你需要完成由 HR 系统触发的部门信息修改和用户信息修改。

因此，你需要调用如下服务端API：

## 创建和更新部门 API

:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)**（选择其一）</md-td></md-th>


</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[创建部门](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create)</md-text>

`POST` /open-apis/contact/v3/departments
  
>向通讯录中创建部门

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>



</md-tr>
  
  <md-tr>

<md-td>

<md-text type="field-name" >[修改部门部分信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/patch)</md-text>

`PATCH` /open-apis/contact/v3/departments/:department_id
>更新通讯录中部门的信息中的任一个字段

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>



</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[更新部门所有信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update)</md-text>

  `PUT` /open-apis/contact/v3/departments/:department_id
  
>用于更新当前部门所有信息

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>
  
 
</md-tr>
  
  <md-tr>

<md-td>

<md-text type="field-name" >[删除部门](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/delete)</md-text>

  `DELETE` /open-apis/contact/v3/departments/:department_id
  
>向通讯录中删除部门

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>


</md-tr>
  </md-tbody>

</md-table>

:::

##  创建和更新用户信息API
:::html

<md-table>

<md-thead>

<tr>

<md-th style="width: 70%;"><md-td>**[方法 (API)](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)**</md-td></md-th>

<md-th style="width: 10%;">权限要求（满足任一）</md-th>

<md-th style="width: 10%;"><md-td>**[访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)**（选择其一）</md-td></md-th>


</tr>

</md-thead>

<md-tbody>

<md-tr>

<md-td>

<md-text type="field-name" >[创建用户](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/create)

   `POST` /open-apis/contact/v3/users
  
   > 向通讯录创建一个用户，可以理解为员工入职
  </md-text>

</md-td>

<md-td><md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>



</md-tr>



<md-tr>

<md-td>

<md-text type="field-name" >[修改用户部分信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/patch)</md-text>
  
`PATCH` /open-apis/contact/v3/users/:user_id
>更新通讯录中用户的字段，未传递的参数不会更新

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新用户基本信息</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>
<md-tag type="token-user">user_access_token</md-tag>

</md-td>



</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[更新用户所有信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/update)</md-text>
  
`PUT` /open-apis/contact/v3/users/:user_id
>更新通讯录中用户的字段

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>




</md-tr>

<md-tr>

<md-td>

<md-text type="field-name" >[删除用户](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/delete)</md-text>
  
`DELETE` /open-apis/contact/v3/users/:user_id

>向通讯录删除一个用户信息，可以理解为员工离职

</md-td>

<md-td>

<md-perm href="/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN">更新通讯录</md-perm>

</md-td>

<md-td>

<md-tag type="token-tenant">tenant_access_token</md-tag>

</md-td>




</md-tr>

</md-tbody>

</md-table>

:::


##  选择权限与访问凭证
从上述所需方法可看到如需调用这些API，需要提前获取应用权限和访问凭证：

-  [应用权限](/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN) 中必须选择 `更新通讯录` 权限；
-  [访问凭证](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM) 中必须获取 `tenant_access_token`

具体的申请和获取方法参见下一步。
