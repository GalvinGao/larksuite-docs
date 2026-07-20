---
document_id: '7055272807039762438'
directory_id: '7050040770682830854'
title: 删除用户组
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/delete
breadcrumb:
- Server API
- Contacts
- User group
- Delete User Group
document_type: ReferenceDocumentType
updated_at: 2022-03-16T13:35:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/delete
---

# 删除用户组

通过该接口可删除企业中的用户组，请注意删除用户组时应用的通讯录权限范围需为“全部员工”，否则会删除失败，[点击了解通讯录权限范围](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=group&method=delete)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 请求
:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>HTTP URL</md-th>
      <md-td>https://open.larksuite.com/open-apis/contact/v3/group/:group_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>DELETE</md-td>
    </md-tr>

   <md-tr>
     <md-th>支持的应用类型</md-th>
      <md-td>
	  <md-app-support types="custom"></md-app-support>
      </md-td>
   </md-tr>


    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
      </md-th>
      <md-td>
<md-perm name="contact:group" desc="更新用户组信息" support_app_types="custom" tags="">更新用户组信息</md-perm>
</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
### 请求头
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 18%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
       <md-th style="width: 15%;">必填</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>Authorization</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      	<md-td>
<md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)

</md-td>
</md-tr>
</md-tbody>
</md-table>
:::



### 路径参数
:::html
<md-table>
  <md-thead>
      <tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th >描述</md-th>
      </tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >group_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	需删除的用户组ID

**示例值**："g1837191"
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::





## 响应



### 响应体
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 40%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 30%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >code</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	错误码，非 0 表示失败
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >msg</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	错误描述
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 响应体示例

```json
{
    "code": 0,
    "msg": "success",
    "data": {}
}
```



### 错误码
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">HTTP状态码</md-th>
            <md-th style="width: 15%;">错误码</md-th>
            <md-th style="width: 30%;">描述</md-th>
            <md-th style="width: 30%;">排查建议</md-th>
        </md-tr>
    </md-thead>
  <md-tbody>

<md-tr>
  <md-td>500</md-td>
  <md-td>40003</md-td>
  <md-td>internal error</md-td>
  <md-td>内部错误，请提供 X-Request-Id向客服反馈。[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42002</md-td>
  <md-td>invalid group_id</md-td>
  <md-td>用户组 ID 无效</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>42009</md-td>
  <md-td>no user group authority error</md-td>
  <md-td>缺少用户组权限。应用的通讯录权限范围需包含该用户组或为“全部员工”，[点击了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42017</md-td>
  <md-td>group has member not allow delete</md-td>
  <md-td>用户组中还有成员，删除之前需先移除用户组成员</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42015</md-td>
  <md-td>user group disable</md-td>
  <md-td>用户组功能未开启，请联系Lark客服处理，[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




