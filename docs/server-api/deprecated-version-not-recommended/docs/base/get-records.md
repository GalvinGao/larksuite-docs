---
document_id: '7073819570391269381'
directory_id: '7524627067896528902'
title: 检索记录
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/get
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Docs
- Base
- Get records
document_type: ReferenceDocumentType
updated_at: 2025-07-16T07:28:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/get
---

# 检索记录

该接口用于根据 record_id 的值检索现有记录。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.table.record&method=get)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">
该接口为历史版本接口，已不推荐使用。你可使用新版[批量获取记录](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_get)接口。
</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id |
| HTTP Method | GET |
| 接口频率限制 | [20 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br><md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv" tags="">查看、评论和导出多维表格</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom,isv" tags="">获取用户 user ID</md-perm><br><md-perm name="contact:user.base:readonly" desc="获取用户基本信息" support_app_types="custom,isv" tags="">获取用户基本信息</md-perm><br><md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm><br><md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm><br><md-perm name="contact:contact:readonly_as_app" desc="以应用身份读取通讯录" support_app_types="custom,isv" tags="history">以应用身份读取通讯录</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>或<br><md-tag mode="inline" type="token-user">user_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |


::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
:::

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
	base app token

**示例值**："bascnCMII2ORej2RItqpZZUNMIe"
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
	table id

**示例值**："tblxI2tWaxP5dG7p"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >record_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	单条记录的 id

**示例值**："recn0hoyXL"
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
	<md-text type="field-name" >text_field_as_array</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	多行文本字段数据是否以数组形式返回。true 表示以数组形式返回。默认为 false

**示例值**：true
	</md-dt-td>
</md-dt-tr>


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
	<md-text type="field-name" >display_formula_ref</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	控制公式、查找引用是否显示完整原样的返回结果。默认为 false

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >with_shared_url</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	控制是否返回该记录的链接，即 record_url 参数。默认为 false，即不返回

**示例值**：true
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >automatic_fields</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >boolean</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	控制是否返回自动计算的字段，例如 `created_by`/`created_time`/`last_modified_by`/`last_modified_time`，true 表示返回。默认为 false

**示例值**：true
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
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
	记录
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
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
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
<md-perm name="contact:contact:access_as_app" desc="以应用身份访问通讯录" support_app_types="custom,isv" tags="history,offline">以应用身份访问通讯录</md-perm>
<md-perm name="contact:contact:readonly" desc="读取通讯录" support_app_types="custom,isv" tags="history,offline">读取通讯录</md-perm>
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
    "code":0,
    "data":{
        "record":{
            "fields":{
                "人力耗时":64,
                "人力评估":"8",
                "任务执行人":[
                    {
                        "email":"huangpaopao@larksuite.com",
                        "en_name":"Paopao Huang",
                        "id":"ou_5fb00e0112212cc7012fe3a697336989",
                        "name":"黄泡泡"
                    }
                ],
                "任务描述":"我是最大的功能开发🥕",
                "任务附件":[
                    {
                        "file_token":"boxcnkQWfV4XbHwzDngmezMGzXe",
                        "name":"2.gif",
                        "size":10250625,
                        "tmp_url":"https://open.larksuite.com/open-apis/drive/v1/medias/batch_get_tmp_download_url?file_tokens=boxcnkQWfV4XbHwzDngmezMGzXe",
                        "type":"image/gif",
                        "url":"https://open.larksuite.com/open-apis/drive/v1/medias/boxcnkQWfV4XbHwzDngmezMGzXe/download"
                    }
                ],
                "对应 OKR":[
                    {
                        "text":"新功能评审",
                        "type":"text"
                    }
                ],
                "截止日期":1612108800000,
                "文档地址":{
                    "link":"https://bytedance.larksuite.com/drive/home/",
                    "text":"文档备份"
                },
                "是否完成":false,
                "状态":"开发中",
                "相关部门":[
                    "研发"
                ],
                "多行文本":[ // text_field_as_array 为true时的结构
                    {
                        "text":"hello",
                        "type":"text"
                    },
                    {
                        "mentionType":"User",
                        "mentionNotify":false,
                        "name":"test",
                        "text":"@test",
                        "token":"ou_sfsdfsdfsdfsdfdsfsdfdsf",
                        "type":"mention"
                    },
                    {
                        "link":"https://test-sasdfsfsd.larksuite.com/base/basbcq2aFvW8nFJpfOXa1111111",
                        "mentionType":"Bitable",
                        "text":"未命名多维表格",
                        "token":"basbcq2aFvW8nFJpfOXalx57ffb",
                        "type":"mention"
                    },
                    {
                        "text":"测试链接标题",
                        "link":"https://www.baidu.com/",
                        "type":"url"
                    }
                ],
                "单向关联":[
                    {
                        "type":"text",
                        "table_id":"tbltAvx3DYBw7PVj",
                        "record_ids":[
                          "recl1IWVnB"
                        ],
                        "text":"第一行"
                    }
                ],
                "双向关联":[
                    {
                        "table_id":"tbltAvx3DYBw7PVj",
                        "record_ids":[
                          "recl1IWVnB",
                          "recrJk7SXT"
                        ],
                        "text":"第一行,第二行",
                        "type":"text"
                    }
                ]
            },
            "record_id":"recn0hoyXL",
            "record_url": "https://bytedance.larksuite.com/record/1sfvuxxxxxxxxxxxxxKdupE5Q"
        }
    },
    "msg":"Success"
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
| 200 | 1254030 | TooLargeResponse | 响应体过大 |
| 400 | 1254036 | Base is copying, please try again later. | 多维表格副本复制中，稍后重试 |
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
| 200 | 1254072 | Failed to convert phone field, please make sure it is correct. | 电话字段格式错误 |
| 200 | 1254100 | TableExceedLimit | 数据表数量超限, 限制300个 |
| 200 | 1254101 | ViewExceedLimit | 视图数量超限, 限制200个 |
| 200 | 1254102 | FileExceedLimit | 超限 |
| 200 | 1254103 | RecordExceedLimit | 记录数量超限, 限制20,000条 |
| 200 | 1254104 | RecordAddOnceExceedLimit | 单次添加记录数量超限, 限制500条 |
| 200 | 1254130 | TooLargeCell | 格子内容过大 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 同一个数据表(table) 不支持并发调用写接口，请检查是否存在并发调用写接口。写接口包括：新增、修改、删除记录；新增、修改、删除字段；修改表单；修改视图等。 |
| 200 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 200 | 1254302 | Permission denied. | 无访问权限, 常由表格开启了高级权限造成, 请在高级权限设置中添加一个包含应用的群, 给予这个群读写权限 |
| 200 | 1254303 | AttachPermNotAllow | 附件无权限 |
| 200 | 1255001 | InternalError | 内部错误，有疑问可咨询客服 |
| 200 | 1255002 | RpcError | 内部错误，有疑问可咨询客服 |
| 200 | 1255003 | MarshalError | 序列化错误，有疑问可咨询客服 |
| 200 | 1255004 | UmMarshalError | 反序列化错误 |
| 200 | 1255005 | ConvError | 内部错误，有疑问可咨询客服处 |
| 504 | 1255040 | 请求超时 | 进行重试 |
| 400 | 1254607 | Data not ready, please try again later | 该报错一般是由于前置操作未执行完成，或本次操作数据太大，服务器计算超时导致。遇到该错误码时，建议等待一段时间后重试。通常有以下几种原因：<br>- **编辑操作频繁**：开发者对多维表格的编辑操作非常频繁。可能会导致由于等待前置操作处理完成耗时过长而超时的情况。多维表格底层对数据表的处理基于版本维度的串行方式，不支持并发。因此，并发请求时容易出现此类错误，不建议开发者对单个数据表进行并发请求。<br>- **批量操作负载重**：开发者在多维表格中进行批量新增、删除等操作时，如果数据表的数据量非常大，可能会导致单次请求耗时过长，最终导致请求超时。建议开发者适当降低批量请求的 page_size 以减少请求耗时。<br>- **资源分配与计算开销**：资源分配是基于单文档维度的，如果读接口涉及公式计算、排序等计算逻辑，会占用较多资源。例如，并发读取一个文档下的多个数据表也可能导致该文档阻塞。 |





