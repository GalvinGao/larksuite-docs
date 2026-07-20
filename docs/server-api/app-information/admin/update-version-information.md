---
document_id: '7070695615567249414'
directory_id: '6907567266537291777'
title: 更新应用审核状态
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/patch
breadcrumb:
- Server API
- App Information
- Admin
- Update version information
document_type: ReferenceDocumentType
updated_at: 2022-03-03T02:30:14Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/patch
---

# 更新应用审核状态

通过接口来更新应用版本的审核结果：通过后应用可以直接上架；拒绝后则开发者可以看到拒绝理由，并在修改后再次申请发布。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application.app_version&method=patch)

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
      <md-td>https://open.larksuite.com/open-apis/application/v6/applications/:app_id/app_versions/:version_id</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>PATCH</md-td>
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
<md-perm name="application:application.app_version" desc="更新应用版本信息" support_app_types="custom" tags="">⁣更新应用版本信息</md-perm>
</md-td>
    </md-tr>
    <md-tr>
      <md-th>
            字段权限要求
</md-th>
      <md-td>

	<md-alert type="tip" icon="none">	
	该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请
	</md-alert>


<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
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
     <md-tr>
      <md-td>Content-Type</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td>**固定值**："application/json; charset=utf-8"</md-td>
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
	<md-text type="field-name" >app_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	应用 id

**示例值**："cli_9f3ca975326b501b"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >version_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	唯一标识应用版本的 ID

**示例值**："oav_d317f090b7258ad0372aa53963cda70d"
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 查询参数
:::html
<md-table>
  <md-thead>
      <tr>
      <md-th style="width: 30%;">名称</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th style="width: 15%;">必填</md-th>
      <md-th >描述</md-th>
      </tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >user_id_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	
用户 ID 类型

**示例值**："open_id"

**可选值有**：
- `open_id`：用户的 open id
- `union_id`：用户的 union id
- `user_id`：用户的 user id

**默认值**：`open_id`

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >operator_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	操作者的 open_id

**示例值**："ou_4065981088f8ef67a504ba8bd6b24d85"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >reject_reason</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	当修改版本状态为被驳回时，这一项必填

**示例值**："拒绝理由"
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 请求体
:::html
引用类型：<md-text type="field-type" >application.app_version</md-text>
:::
:::html
<md-table>
  <md-thead>
      <md-tr>
      <md-th style="width: 40%;">名称</md-th>
      <md-th style="width: 20%;">类型</md-th>
      <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 30%;">描述</md-th>
      </md-tr>
  </md-thead>
  <md-tbody>

<md-tr>
	<md-td>
	<md-text type="field-name" >status</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	版本状态

**示例值**：1

**可选值有**：
- `0`：未知状态
- `1`：审核通过
- `2`：审核拒绝
- `3`：审核中
- `4`：未提交审核
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 请求体示例

```json
{
    "status": 1
}
```



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
    "msg": "success"
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
  <md-td>400</md-td>
  <md-td>210503</md-td>
  <md-td>invalid app_id</md-td>
  <md-td>请检查请求路径中的 app_id 是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>210504</md-td>
  <md-td>no such app in tenant</md-td>
  <md-td>请检查被查询应用与当前调用接口应用是否在同一企业内</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>210505</md-td>
  <md-td>target app not a custom app</md-td>
  <md-td>请检查被查询应用是否是自建应用</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>210506</md-td>
  <md-td>no such app</md-td>
  <md-td>请检查请求路径中的 app_id 是否存在</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>210507</md-td>
  <md-td>no such user_id</md-td>
  <md-td>请检查传入的用户 id 是否存在</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>211002</md-td>
  <md-td>no such version_id</md-td>
  <md-td>请检查路径中的 version_id 是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>211003</md-td>
  <md-td>no such version of desired app</md-td>
  <md-td>请检查 version_id 是否属于 app_id 对应应用</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>211004</md-td>
  <md-td>no authority for quota limit</md-td>
  <md-td>检查是否是企业版/旗舰版租户</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>211005</md-td>
  <md-td>invalid app id</md-td>
  <md-td>检查 app id</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>211006</md-td>
  <md-td>invalid department id</md-td>
  <md-td>检查 department id</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




