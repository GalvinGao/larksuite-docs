---
document_id: '7130175416736415750'
directory_id: '7120048623526117381'
title: 新增协作者
full_path: /uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/create
breadcrumb:
- Server API
- Docs
- Base
- Advanced Permission
- Member
- Create member
document_type: ReferenceDocumentType
updated_at: 2025-07-28T02:01:58Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/create
---

# 新增协作者

新增多维表格高级权限中自定义角色的协作者。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=bitable&version=v1&resource=app.role.member&method=create)

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

要调用协作者相关接口，你需确保多维表格已开启高级权限并设置了自定义角色。你可通过[更新多维表格元数据](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/update)接口开启高级权限，通过[新增自定义角色](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/create)接口设置自定义角色。


## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members |
| HTTP Method | POST |
| 接口频率限制 | [10 次/秒](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 支持的应用类型 | <md-app-support types="custom,isv"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip><br><div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div> | <md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理多维表格</md-perm><br>base:collaborator:create |

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


**示例值**："appbcbWCzen6D8dezhoCH2RpMAh"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >role_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	多维表格高级权限中自定义角色的唯一标识，以 rol 开头。获取方式：通过[列出自定义角色](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/list)接口获取。

**示例值**："roljRpwIUt"
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
	<md-text type="field-name" >member_id_type</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	协作者 ID 的类型

**示例值**：open_id

**可选值有**：
<md-enum>
<md-enum-item key="open_id" >以 open_id 来识别协作者。获取方式参考[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)</md-enum-item>
<md-enum-item key="union_id" >以 union_id 来识别协作者。获取方式参考[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)</md-enum-item>
<md-enum-item key="user_id" >以 user_id 来识别协作者。获取方式参考[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)</md-enum-item>
<md-enum-item key="chat_id" >以 chat_id 来识别协作者。获取方式参考[群 ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)</md-enum-item>
<md-enum-item key="department_id" >以 department_id 来识别协作者。调用前，请确保应用有部门的可见性，参考[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。获取 department_id 方式参考[部门资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)</md-enum-item>
<md-enum-item key="open_department_id" >以 open_department_id 来识别协作者。调用前，请确保应用有部门的可见性，参考[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。获取 open_department_id 方式参考[部门资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)</md-enum-item>
</md-enum>

**默认值**：`open_id`
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
	<md-text type="field-name" >member_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	高级权限中自定义角色协作者的 ID，需与查询参数中 member_id_type 的类型需一致。获取 ID 方式参考 member_id_type 参数描述。

**示例值**："ou_7dab8a3d3cdcc9da365777c7ad535d62"
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::





### 请求体示例
:::html
<md-code-json>
{
    "member_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62"
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


  </md-dt-tbody>
</md-dt-table>
:::



### 响应体示例
:::html
<md-code-json>
{
    "code": 0,
    "msg": "success",
    "data": {}
}
</md-code-json>
:::



### 错误码

| HTTP状态码 | 错误码 | 描述 | 排查建议 |
| --- | --- | --- | --- |
| 200 | 1254000 | WrongRequestJson | 请求体错误 |
| 200 | 1254001 | WrongRequestBody | 请求体错误 |
| 200 | 1254002 | Fail | 导致报 1254002 错误码的场景较多，请参考以下建议排查：<br>- 如果单次操作的内容变更较大，请尝试在单次操作中减少数据量<br>- 如果你并发调用了接口，请尝试控制请求间隔，稍后重试<br>- 如果在知识库（wiki）中创建多维表格，请检查你是否使用了知识库[创建知识空间节点](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/create)接口创建多维表格。在此场景下不能使用[创建多维表格](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create)接口<br>- 请检查接口参数是否有误。例如，在分页查询多维表格时，传递了无效的 page_token，或传递了错误的数据表的 table_id<br>- 如果该报错偶尔发生，可能是服务器超时或不稳定，请重试解决 |
| 200 | 1254003 | WrongBaseToken | app_token 错误。app_token 是多维表格 App 的唯一标识。不同形态的多维表格，其 `app_token` 的获取方式不同：<br>- 如果多维表格的 URL 以 ==**larksuite.com/base**== 开头，该多维表格的 `app_token` 是 base 之后的字符串<br>- 如果多维表格的 URL 以 ==**larksuite.com/wiki**== 开头，你需调用知识库相关[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)接口获取多维表格的 app_token。当 `obj_type` 的值为 `bitable` 时，`obj_token` 字段的值才是多维表格的 `app_token` |
| 200 | 1254010 | ReqConvError | 请求错误 |
| 400 | 1254032 | InvalidRoleName | 自定义角色名无效 |
| 400 | 1254033 | RoleNameDuplicated | 自定义角色名重复 |
| 400 | 1254036 | Base is copying, please try again later. | 复制多维表格为异步操作，该错误码表示当前多维表格仍在复制中，在复制期间无法操作当前多维表格。需要等待复制完成后再操作 |
| 200 | 1254040 | BaseTokenNotFound | app_token 不存在。不同形态的多维表格，其 `app_token` 的获取方式不同：<br>- 如果多维表格的 URL 以 ==**larksuite.com/base**== 开头，该多维表格的 `app_token` 是 base 之后的字符串<br>- 如果多维表格的 URL 以 ==**larksuite.com/wiki**== 开头，你需调用知识库相关[获取知识空间节点信息](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node)接口获取多维表格的 app_token。当 `obj_type` 的值为 `bitable` 时，`obj_token` 字段的值才是多维表格的 `app_token` |
| 404 | 1254047 | RoleIdNotFound | role_id 不存在。role_id 是多维表格高级权限中自定义角色的唯一标识，以 `rol` 开头。可通过[列出自定义角色](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/list)接口获取。 |
| 404 | 1254048 | MemberNotFound | 协作者 ID 不存在。请检查：<br>- 查询参数 member_id_type 与 member_id 的类型是否保持一致；<br>- member_id 填写是否正确。不同类型的 ID 获取方式不同：<br>- open_id：参考[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)<br>- union_id：参考[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)<br>- user_id：参考[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)<br>- chat_id：参考[群 ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)<br>- department_id：调用前，请确保应用有部门的可见性，参考[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。获取 department_id 方式参考[部门资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)<br>- open_department_id：调用前，请确保应用有部门的可见性，参考[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。获取 open_department_id 方式参考[部门资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview) |
| 400 | 1254110 | RoleExceedLimit | 自定义角色数量超限，限制 30 条 |
| 400 | 1254111 | MemberExceedLimit | 自定义角色的协作者数量超限，限制 200 个 |
| 200 | 1254290 | TooManyRequest | 请求过快，稍后重试 |
| 200 | 1254291 | Write conflict | 在同一个数据表中，并发调用了读写接口或请求过快，出现冲突。请参考以下建议解决：<br>- 确保没有并发调用多维表格读写相关接口<br>- 若操作量较大，建议在接口与接口之间增加 0.5 或 1 秒的延迟，也可在报错中增加重试逻辑，确保业务的稳定性<br>- 对于写接口，可以将接口中的查询参数 `ignore_consistency_check` 设置为 true，表示在读写操作时，暂时忽略一致性检查，以提高性能 |
| 400 | 1254301 | OperationTypeError | 多维表格未开启高级权限或不支持开启高级权限 |
| 403 | 1254302 | Permission denied. | 调用身份缺少多维表格的高级权限。你需给予调用身份数据表的 **可管理** 权限或多维表格的 **可管理** 等权限，再重新调用。具体步骤如下所示：<br>- 对用户授予高级权限，你可在 **多维表格高级权限设置** 中添加用户，为用户开通足够权限；或在多维表格页面右上方 **分享** 入口为当前用户添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/df3911b4f747d75914f35a46962d667d_dAsfLjv3QC.png?height=546&lazyload=true&maxWidth=550)<br>- 对应用授予高级权限，你需通过多维表格页面右上方 **「...」** -> **「...更多」** ->**「添加文档应用」** 入口为应用添加可管理权限。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/22c027f63c540592d3ca8f41d48bb107_CSas7OYJBR.png?height=1994&lazyload=true&maxWidth=550&width=3278)<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9f3353931fafeea16a39f0eb887db175_0tjzC9P3zU.png?height=728&lazyload=true&maxWidth=550&width=890)<br>**注意**：<br>在 **添加文档应用** 前，你需确保目标应用至少开通了一个多维表格的 [API 权限](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。否则你将无法在文档应用窗口搜索到目标应用。<br>- 你也可以在 **多维表格高级权限设置** 中添加用户或一个包含应用的群组，给予这个群自定义的读写等权限。 |
| 200 | 1255001 | InternalError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255002 | RpcError | 内部错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255003 | MarshalError | 序列化错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 200 | 1255004 | UmMarshalError | 反序列化错误，请联系[技术支持](https://applink.larksuite.com/TLJpeNdW) |
| 504 | 1255040 | Request timed out, please try again later | 请求超时，进行重试 |





