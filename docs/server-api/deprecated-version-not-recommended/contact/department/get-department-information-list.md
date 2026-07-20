---
document_id: '6965400907878219781'
directory_id: '6907567266537242625'
title: 获取部门信息列表
full_path: /uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/list
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Contact
- Department
- Get Department Information List
document_type: ReferenceDocumentType
updated_at: 2022-03-11T11:44:44Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/list
---

# 获取部门信息列表

该接口用于获取当前部门子部门列表。[常见问题答疑](/document/ugTN1YjL4UTN24CO1UjN/uQzN1YjL0cTN24CN3UjN)。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=contact&version=v3&resource=department&method=list)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
- 使用 user_access_token 时，返回该用户组织架构可见性范围（[登陆企业管理后台进行权限配置](https://www.larksuite.com/admin/security/permission/visibility)）内的所有可见部门。当进行递归查询时，只筛查最多1000个部门的可见性。

- 使用 
 tenant_access_token 则基于应用的通讯录权限范围进行权限校验与过滤。由于 
 parent_department_id 是非必填参数，填与不填存在<b>两种数据权限校验与返回</b>情况：
<br> <br>1、请求设置了 
 parent_department_id 为A（根部门0），会检验A是否在通讯录权限内，若在( parent_department_id=0 时会校验是否为全员权限），则返回部门下子部门列表（根据fetch_child决定是否递归），否则返回无部门通讯录权限错误码。
<br> <br>2、请求未带 
 parent_department_id 参数，如通讯录范围为全员权限，只返回根部门ID(部门ID为0)，否则返回根据通讯录范围配置的部门ID及子部门(根据 
 fetch_child 决定是否递归)。
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
      <md-td>https://open.larksuite.com/open-apis/contact/v3/departments</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>GET</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
      <md-th>
            权限要求
            <md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip>
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
      </md-th>
      <md-td>
            <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
            <md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
            <md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
            <md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
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
        <md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
        <md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
        <md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
        <md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
        <md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
        <md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
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
或
<md-tag mode="inline" type="token-user">user_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"

[了解更多：获取与使用access_token](/document/ukTMukTMukTM/uMTNz4yM1MjLzUzM)

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
	否
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
	<md-text type="field-name" >department_id_type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	此次调用中使用的部门ID的类型

**示例值**："open_department_id"

**可选值有**：
- `department_id`：以自定义department_id来标识部门
- `open_department_id`：以open_department_id来标识部门

**默认值**：`open_department_id`
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >parent_department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	父部门的ID，填上获取部门下所有子部门，此处填写的 ID 必须是 department_id_type 指定的 ID。

**示例值**："od-4e6ac4d14bcd5071a37a39de902c7141"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >fetch_child</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	是否递归获取子部门

**示例值**：是否递归获取子部门，默认值：false
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >page_token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果

**示例值**："AQD9/Rn9eij9Pm39ED40/RD/cIFmu77WxpxPB/2oHfQLZ%2BG8JG6tK7%2BZnHiT7COhD2hMSICh/eBl7cpzU6JEC3J7COKNe4jrQ8ExwBCR"
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >page_size</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	分页大小

**示例值**：10

**数据校验规则**：
- 最大值：`50`
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


<md-tr>
	<md-td>
	<md-text type="field-name" >data</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >\-</md-text>
	</md-td>
	<md-td>
	\-
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >has_more</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否还有更多项
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >page_token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >items</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >department\[\]</md-text>
	</md-td>
	<md-td>
	 
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门名称

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_name</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >department_i18n_name</md-text>
	</md-td>
	<md-td>
	国际化的部门名称

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >zh_cn</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门的中文名
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ja_jp</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门的日文名
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >en_us</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门的英文名
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >parent_department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	父部门的ID

* 创建根部门，该参数值为 “0”

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	本部门的自定义部门ID

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_department_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门的open_id
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >leader_user_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门主管用户ID

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >chat_id</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门群ID

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >order</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	部门的排序，即部门在其同级部门的展示顺序

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >unit_ids</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string\[\]</md-text>
	</md-td>
	<md-td>
	部门单位自定义ID列表，当前只支持一个

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >member_count</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >int</md-text>
	</md-td>
	<md-td>
	部门下用户的个数

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.organize:readonly" desc="获取部门组织架构信息" support_app_types="custom,isv" tags="">获取部门组织架构信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >department_status</md-text>
	</md-td>
	<md-td>
	部门状态

**字段权限要求（满足任一）**：
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="">以应用身份读取通讯录</md-perm>
<md-perm name="contact:department.base:readonly" desc="获取部门基础信息" support_app_types="custom,isv" tags="">获取部门基础信息</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_deleted</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否被删除
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
    "data": {
        "has_more": true,
        "page_token": "AQD9/Rn9eij9Pm39ED40/RD/cIFmu77WxpxPB/2oHfQLZ%2BG8JG6tK7%2BZnHiT7COhD2hMSICh/eBl7cpzU6JEC3J7COKNe4jrQ8ExwBCR",
        "items": [
            {
                "name": "DemoName",
                "i18n_name": {
                    "zh_cn": "Demo名称",
                    "ja_jp": "デモ名",
                    "en_us": "Demo Name"
                },
                "parent_department_id": "D067",
                "department_id": "D096",
                "open_department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
                "leader_user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
                "chat_id": "oc_5ad11d72b830411d72b836c20",
                "order": "100",
                "unit_ids": [
                    "custom_unit_id"
                ],
                "member_count": 100,
                "status": {
                    "is_deleted": false
                }
            }
        ]
    }
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
  <md-td>43010</md-td>
  <md-td>big dept forbid recursion error</md-td>
  <md-td>超大部门不允许进行查询。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40003</md-td>
  <md-td>internal error</md-td>
  <md-td>内部错误，请提供 X-Request-Id向客服反馈。[联系客服](https://applink.larksuite.com/client/helpdesk/open?id=6626260912531570952&extra=%7B%22channel%22%3A14%2C%22created_at%22%3A1614493146%2C%22scenario_id%22%3A6885151765134622721%2C%22signature%22%3A%22ca94c408b966dc1de2083e5bbcd418294c146e98%22%7D)。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40008</md-td>
  <md-td>dept Info is null error</md-td>
  <md-td>部门的信息不能为空</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>40014</md-td>
  <md-td>no parent dept authority error</md-td>
  <md-td>没有父部门权限。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40011</md-td>
  <md-td>page size is invalid</md-td>
  <md-td>无效的分页参数</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>40012</md-td>
  <md-td>page token is invalid error</md-td>
  <md-td>page token无效。</md-td>
</md-tr>


<md-tr>
  <md-td>401</md-td>
  <md-td>42008</md-td>
  <md-td>tenant id is invalid error</md-td>
  <md-td>请检查请求租户是否为合法租户。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>43007</md-td>
  <md-td>duplicated department custom id error</md-td>
  <md-td>部门自定义ID 企业内重复</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>40004</md-td>
  <md-td>no dept authority error</md-td>
  <md-td>操作的部门需在通讯录权限范围中，[了解更多](/document/ukTMukTMukTM/uETNz4SM1MjLxUzM/v3/guides/scope_authority)</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




