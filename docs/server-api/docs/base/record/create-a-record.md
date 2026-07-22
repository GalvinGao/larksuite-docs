---
document_id: '7073819570391220229'
directory_id: '7072190414392360966'
title: 新增记录
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/create
breadcrumb:
- Server API
- Docs
- Base
- Record
- Create a record
document_type: ReferenceDocumentType
updated_at: 2026-03-12T13:49:02Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/create
---

# 新增记录

在多维表格数据表中新增一条记录。
{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.table.record&method=create)

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

## 前提条件

调用此接口前，请确保当前调用身份（tenant_access_token 或 user_access_token）已有多维表格的编辑等文档权限，否则接口将返回 HTTP 403 或 400 状态码。


## 注意事项

从其它数据源同步的数据表，不支持对记录进行增加、删除、和修改操作。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records |
| HTTP Method | POST |
| 接口频率限制 | [50 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="base:record:create" desc="新增记录" support_app_types="custom,isv" tags="">新增记录</md-perm><br><md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom,isv" tags="">获取用户 user ID</md-perm><br><md-perm name="contact:user.base:readonly" desc="获取用户基本信息" support_app_types="custom,isv" tags="">获取用户基本信息</md-perm><br><md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="history">以应用身份读取通讯录</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |




### 路径参数
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
	<md-text type="field-name" >app_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格 App 的唯一标识。不同形态的多维表格，其 app_token 的获取方式不同：
- 如果多维表格的 URL 以 ==**larksuite.com/base**== 开头，该多维表格的 `app_token` 是 base 之后的字符串

- 如果多维表格的 URL 以 ==**larksuite.com/wiki**== 开头，你需调用知识库相关[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)接口获取多维表格的 app_token。当 `obj_type` 的值为 `bitable` 时，`obj_token` 字段的值才是多维表格的 `app_token`


**示例值**："bascng7vrxcxpig7geggXiCtadY"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >table_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格数据表的唯一标识。获取方式：
- 你可通过多维表格 URL 获取 `table_id`，下图高亮部分即为当前数据表的 `table_id`
- 也可通过[列出数据表](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list)接口获取 `table_id`

   ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d4750d909d5a27493f6f5fe633d7cc95_CkBmQkbGch.png?height=765&lazyload=true&maxWidth=300&width=1731)

**示例值**："tblUa9vcYjWQYJCj"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 查询参数
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 35%;">名称</md-dt-th>
      <md-dt-th style="width: 13%;">类型</md-dt-th>
      <md-dt-th style="width: 15%;" filters="是,否" >必填</md-dt-th>
      <md-dt-th style="width: 37%;" >描述</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >user_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	用户 ID 类型

**示例值**：open_id

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)</md-enum-item>
<md-enum-item key="union_id" >标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)</md-enum-item>
<md-enum-item key="user_id" >标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)</md-enum-item>
</md-enum>

**默认值**：`open_id`

**当值为 `user_id`，字段权限要求**：
<md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >client_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。

**示例值**：fe599b60-450f-46ff-b2ef-9f6675625b97
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >ignore_consistency_check</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	是否忽略一致性读写检查，默认为 false，即在进行读写操作时，系统将确保读取到的数据和写入的数据是一致的。可选值：

- true：忽略读写一致性检查，提高性能，但可能会导致某些节点的数据不同步，出现暂时不一致
- false：开启读写一致性检查，确保数据在读写过程中一致

**示例值**：true
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
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
	<md-text type="field-name" >fields</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, union&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	要新增的记录的数据。你需先指定数据表中的字段（即指定列），再传入正确格式的数据作为一条记录。

**注意**：

该接口支持的字段类型及其描述如下所示：
- 文本： 填写字符串格式的值
- 数字：填写数字格式的值
- 单选：填写选项值，对于新的选项值，将会创建一个新的选项
- 多选：填写多个选项值，对于新的选项值，将会创建一个新的选项。如果填写多个相同的新选项值，将会创建多个相同的选项
- 日期：填写毫秒级时间戳
- 复选框：填写 true 或 false
- 条码
- 人员：填写用户的[open_id](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)、[union_id](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id) 或 [user_id](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)，类型需要与 user_id_type 指定的类型一致
- 电话号码：填写文本内容
- 超链接：参考以下示例，text 为文本值，link 为 URL 链接
- 附件：填写附件 token，需要先调用[上传素材](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_all)或[分片上传素材](/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare)接口将附件上传至该多维表格中
- 单向关联：填写被关联表的记录 ID
- 双向关联：填写被关联表的记录 ID
- 地理位置：填写经纬度坐标

不同类型字段的数据结构请参考[多维表格记录数据结构](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/bitable-record-data-structure-overview)。

**示例值**：{
    "人员": [
        {
            "id": "ou_2910013f1e6456f16a0ce75ede9abcef"
        }
    ]
}
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
	"fields": {
		"多行文本": "多行文本内容",
        "条码":"qawqe",
		"数字": 100,
        "货币":3,
        "评分":3,
        "进度":0.25,
		"单选": "选项1",
		"多选": ["选项1", "选项2"],
		"日期": 1674206443000,
		"复选框": true,
		"人员": [{
			"id": "ou_2910013f1e6456f16a0ce75ede950a0a"
		}, {
			"id": "ou_e04138c9633dd0d2ea166d79f548ab5d"
		}],
        "群组":[
            {
                "id": "oc_cd07f55f14d6f4a4f1b51504e7e97f48"
            }
        ],
		"电话号码": "13026162666",
		"超链接": {
			"text": "Lark多维表格官网",
			"link": "https://www.larksuite.com/product/base"
		},
		"附件": [{
			"file_token": "DRiFbwaKsoZaLax4WKZbEGCccoe"
		}, {
			"file_token": "BZk3bL1Enoy4pzxaPL9bNeKqcLe"
		}, {
			"file_token": "EmL4bhjFFovrt9xZgaSbjJk9c1b"
		}, {
			"file_token": "Vl3FbVkvnowlgpxpqsAbBrtFcrd"
		}],
		"单向关联": ["recHTLvO7x", "recbS8zb2m"],
		"双向关联": ["recHTLvO7x", "recbS8zb2m"],
		"地理位置": "116.397755,39.903179"
	}
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
	<md-text type="field-name" >record</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >app.table.record</md-text>
	</md-dt-td>
	<md-dt-td>
	新增的记录的内容
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >fields</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >map&lt;string, union&gt;</md-text>
	</md-dt-td>
	<md-dt-td>
	数据表的字段，即数据表的列

当前接口支持的字段类型请参考[接入指南](/document/uAjLw4CM/ukTMukTMukTM/bitable/notification#31f78a3c)

不同类型字段的数据结构请参考[数据结构概述](/document/uAjLw4CM/ukTMukTMukTM/bitable/development-guide/bitable-structure)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >record_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	一条记录的唯一标识 id [record_id 参数说明](/document/uAjLw4CM/ukTMukTMukTM/bitable/notification#15d8db94)
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >created_by</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >person</md-text>
	</md-dt-td>
	<md-dt-td>
	该记录的创建人
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户id，id类型等于user_id_type所指定的类型。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的中文名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >en_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的英文名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >email</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的邮箱
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >avatar_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	头像链接

**字段权限要求（满足任一）**：
<md-perm name="contact:user.base:readonly" desc="获取用户基本信息" support_app_types="custom,isv" tags="">获取用户基本信息</md-perm>
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="history">以应用身份读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >created_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	该记录的创建时间
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >last_modified_by</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >person</md-text>
	</md-dt-td>
	<md-dt-td>
	该记录最新一次更新的修改人
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户id，id类型等于user_id_type所指定的类型。
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的中文名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >en_name</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的英文名称
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >email</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	用户的邮箱
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="3">
	<md-dt-td>
	<md-text type="field-name" >avatar_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	头像链接

**字段权限要求（满足任一）**：
<md-perm name="contact:user.base:readonly" desc="获取用户基本信息" support_app_types="custom,isv" tags="">获取用户基本信息</md-perm>
<md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="history">以应用身份读取通讯录</md-perm>
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="2">
	<md-dt-td>
	<md-text type="field-name" >last_modified_time</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	该记录最近一次的更新时间
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
    "data": {
        "record": {
            "fields": {
                "条码": "qawqe",
                "人员": [
                    {
                        "id": "ou_2910013f1e6456f16a0ce75ede950a0a"
                    },
                    {
                        "id": "ou_e04138c9633dd0d2ea166d79f548ab5d"
                    }
                ],
                "单向关联": [
                    "recHTLvO7x",
                    "recbS8zb2m"
                ],
                "单选": "选项3",
                "双向关联": [
                    "recHTLvO7x",
                    "recbS8zb2m"
                ],
                "地理位置": "116.397755,39.903179",
                "复选框": true,
                "多行文本": "多行文本内容",
                "多选": [
                    "选项1",
                    "选项2"
                ],
                "数字": 100,
                "日期": 1674206443000,
                "电话号码": "13026162666",
                "索引": "索引列多行文本类型",
                "超链接": {
                    "link": "https://www.larksuite.com/product/base",
                    "text": "Lark多维表格官网"
                },
                "附件": [
                    {
                        "file_token": "Vl3FbVkvnowlgpxpqsAbBrtFcrd"
                    }
                ],
                "群组": [
                    {
                        "id": "oc_cd07f55f14d6f4a4f1b51504e7e97f48"
                    }
                ],
                "评分": 3,
                "货币": 3,
                "进度": 0.25
            },
            "id": "reclAqylTN",
            "record_id": "reclAqylTN"
        }
    },
    "msg": "success"
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 1254000 | WrongRequestJson | 请求体错误 |
| 200 | 1254001 | WrongRequestBody | 请求体错误 |
| 200 | 1254002 | Fail | 内部错误，有疑问可咨询客服 |
| 200 | 1254003 | WrongBaseToken | app_token 错误 |
| 200 | 1254004 | WrongTableId | table_id 错误 |
| 200 | 1254005 | WrongViewId | view_id 错误 |
| 200 | 1254006 | WrongRecordId | 检查 record_id |
| 200 | 1254007 | EmptyValue | 空值 |
| 200 | 1254008 | EmptyView | 空视图 |
| 200 | 1254009 | WrongFieldId | 字段 id 错误 |
| 200 | 1254010 | ReqConvError | 请求错误 |
| 400 | 1254015 | Field types do not match. | 字段类型和值不匹配 |
| 403 | 1254027 | UploadAttachNotAllowed | 附件未挂载, 禁止上传 |
| 200 | 1254030 | TooLargeResponse | 响应体过大 |
| 400 | 1254036 | Base is copying, please try again later. | 多维表格副本复制中，稍后重试 |
| 400 | 1254037 | Invalid client token, make sure that it complies with the specification. | 幂等键格式错误，需要传入 uuidv4 格式 |
| 200 | 1254040 | BaseTokenNotFound | app_token 不存在 |
| 200 | 1254041 | TableIdNotFound | table_id 不存在 |
| 200 | 1254042 | ViewIdNotFound | view_id 不存在 |
| 200 | 1254043 | RecordIdNotFound | record_id 不存在 |
| 200 | 1254044 | FieldIdNotFound | field_id  不存在 |
| 200 | 1254045 | FieldNameNotFound | 字段名字不存在 |
| 200 | 1254060 | TextFieldConvFail | 多行文本字段错误 |
| 200 | 1254061 | NumberFieldConvFail | 数字字段错误 |
| 200 | 1254062 | SingleSelectFieldConvFail | 单选字段错误 |
| 200 | 1254063 | MultiSelectFieldConvFail | 多选字段错误 |
| 200 | 1254064 | DatetimeFieldConvFail | 日期字段错误 |
| 200 | 1254065 | CheckboxFieldConvFail | 复选框字段错误 |
| 200 | 1254066 | UserFieldConvFail | 人员字段错误 |
| 200 | 1254067 | LinkFieldConvFail | 关联字段错误 |
| 200 | 1254068 | URLFieldConvFail | 超链接字段错误 |
| 200 | 1254069 | AttachFieldConvFail | 附件字段错误 |
| 200 | 1254072 | Failed to convert phone field, please make sure it is correct. | 电话字段错误 |
| 400 | 1254074 | The parameters of Duplex Link field are invalid and need to be filled with an array of string. | 双向关联字段格式非法 |
| 200 | 1254100 | TableExceedLimit | 数据表数量超限, 限制300个 |
| 200 | 1254101 | ViewExceedLimit | 视图数量超限, 限制200个 |
| 200 | 1254102 | FileExceedLimit | 超限 |
| 200 | 1254103 | RecordExceedLimit | 记录数量超限, 限制20,000条 |
| 200 | 1254104 | RecordAddOnceExceedLimit | 单次添加记录数量超限, 限制500条 |
| 200 | 1254105 | ColumnExceedLimit | 字段数量超限 |
| 200 | 1254106 | AttachExceedLimit | 附件过多 |
| 200 | 1254130 | TooLargeCell | 格子内容过大 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。 |
| 200 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 200 | 1254303 | The attachment does not belong to this base. | 附件无权限 |
| 200 | 1255001 | InternalError | 内部错误，有疑问可咨询客服 |
| 200 | 1255002 | RpcError | 内部错误，有疑问可咨询客服 |
| 200 | 1255003 | MarshalError | 序列化错误，有疑问可咨询客服 |
| 200 | 1255004 | UmMarshalError | 反序列化错误 |
| 200 | 1255005 | ConvError | 内部错误，有疑问可咨询客服处 |
| 400 | 1255006 | Client token conflict, please generate a new client token and try again. | 幂等键冲突，需要重新随机生成一个幂等键 |
| 504 | 1255040 | 请求超时 | 进行重试 |
| 400 | 1254607 | Data not ready, please try again later | 出现这个错误通常有两种情况：1. 上次提交的修改还没有处理完；2. 数据太大，服务器计算超时；<br>遇到这个错误码可以适当进行重试。 |
| 403 | 1254302 | Permission denied. | 无访问权限, 常由表格开启了高级权限造成, 请在高级权限设置中添加一个包含应用的群, 给予这个群读写权限 |
| 403 | 1254304 | Permission denied. | 仅企业版和旗舰版Lark支持行列权限 |
| 403 | 1254306 | The tenant or base owner is subject to base plan limits. | 联系租户管理员申请权益 |
| 403 | 1254608 | Same API requests are submitted repeatedly. | 确认本次请求的请求参数和上一次是否完全相同 |





