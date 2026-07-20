---
document_id: '7055272807039418374'
directory_id: '7050040770682830854'
title: 创建用户组
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/create
breadcrumb:
- Server API
- Contacts
- User group
- Add User Group
document_type: ReferenceDocumentType
updated_at: 2022-08-30T10:15:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/create
---

# 创建用户组

使用该接口创建用户组，请注意创建用户组时应用的通讯录权限范围需为“全部员工”，否则会创建失败，[点击了解通讯录权限范围](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=group&method=create)

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
      <md-td>https://open.larksuite.com/open-apis/contact/v3/group</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>POST</md-td>
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
      <md-th style="width: 35%;">名称</md-th>
      <md-th style="width: 13%;">类型</md-th>
       <md-th style="width: 15%;" filters="是,否" >必填</md-th>
      <md-th  style="width: 37%;">描述</md-th>
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
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::



### 请求体

:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >group_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	自定义用户组ID，可在创建时自定义，不自定义则由系统自动生成，已创建用户组不允许修改 group_id 。

自定义group_id数据校验规则：

最大长度：64 字符

校验规则：数字、大小写字母的组合，不能包含空格

**示例值**："g122817"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	用户组的名字，企业内唯一，最大长度：100 字符

**示例值**："IT 外包组"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >description</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户组描述

**示例值**："IT服务人员的集合"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户组的类型。默认为1表示普通用户组

**示例值**：1

**可选值有**：
<md-enum>
<md-enum-item key="1" >普通用户组</md-enum-item>
</md-enum>

**默认值**：`1`
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体示例
:::html
<md-code-json>
{
    "group_id": "g122817",
    "name": "IT 外包组",
    "description": "IT服务人员的集合",
    "type": 1
}
</md-code-json>
:::



## 响应



### 响应体
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 52%;">描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>

<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >code</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	错误码，非 0 表示失败
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >msg</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	错误描述
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >data</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >\-</md-text>
	</md-dt-td>
	<md-dt-td>
	\-
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >group_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户组ID
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {
        "group_id": "g122817"
    }
}
</md-code-json>
:::



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
  <md-td>400</md-td>
  <md-td>42003</md-td>
  <md-td>group type invalid</md-td>
  <md-td>用户组类型无效</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42001</md-td>
  <md-td>group name empty</md-td>
  <md-td>用户组名字不得为空</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42013</md-td>
  <md-td>group name exceed limit</md-td>
  <md-td>用户组名字长度超过最大限制，最大限制100字符</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42014</md-td>
  <md-td>group description exceed limit</md-td>
  <md-td>用户组描述长度超过最大限制，最大限制500字符</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>42010</md-td>
  <md-td>not has all authority error</md-td>
  <md-td>应用通讯录权限范围需为全部员工，[点击了解通讯录权限范围](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>47005</md-td>
  <md-td>duplicate group id error</md-td>
  <md-td>用户组自定义 ID 重复，企业内唯一</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>47009</md-td>
  <md-td>duplicated name error</md-td>
  <md-td>用户组名称不得在企业内重复</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>42016</md-td>
  <md-td>user group number exceed limit</md-td>
  <md-td>用户组数量超过最大限制，单个企业最多可创建500个用户组</md-td>
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




