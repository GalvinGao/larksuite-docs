---
document_id: '7031483915610570757'
directory_id: '7031445675029594117'
title: 更新文档公共设置
full_path: /uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/patch
breadcrumb:
- Server API
- Docs
- Permission
- Setting v1
- Update document sharing settings
document_type: ReferenceDocumentType
updated_at: 2022-03-13T13:40:21Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/patch
---

# 更新文档公共设置

该接口用于根据 filetoken 更新文档的公共设置。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=drive&version=v1&resource=permission.public&method=patch)

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
      <md-td>https://open.larksuite.com/open-apis/drive/v1/permissions/:token/public</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>PATCH</md-td>
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
<md-perm name="drive:file" desc="上传、下载文件到云空间" support_app_types="custom,isv" tags="">上传、下载文件到云空间</md-perm>
<md-perm name="wiki:wiki" desc="查看、编辑和管理知识库" support_app_types="custom,isv" tags="">查看、编辑和管理知识库</md-perm>
<md-perm name="drive:drive" desc="查看、评论、编辑和管理云空间中所有文件" support_app_types="custom,isv" tags="">查看、评论、编辑和管理云空间中所有文件</md-perm>
<md-perm name="docs:doc" desc="查看、评论、编辑和管理文档" support_app_types="custom,isv" tags="">查看、评论、编辑和管理文档</md-perm>
<md-perm name="sheets:spreadsheet" desc="查看、评论、编辑和管理电子表格" support_app_types="custom,isv" tags="">查看、评论、编辑和管理电子表格</md-perm>
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
<md-tr>
<md-td>Content-Type</md-td>
<md-td>string</md-td>
<md-td>是</md-td>
<md-td>**固定值**："application/json; charset=utf-8"</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

::: note
关于云文档接口的 AccessToken 调用说明详见 [云文档接口快速入门](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN)
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
	<md-text type="field-name" >token</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	文件的 token，获取方式见 [概述](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/files/guide/introduction)

**示例值**："doccnBKgoMyY5OMbUG6FioTXuBe"
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
	<md-text type="field-name" >type</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	是
	</md-td>
	<md-td>
	权限客体类型，放于query参数中，如：`?type=doc`

**示例值**："doc"

**可选值有**：
- `doc`：文档
- `sheet`：电子表格
- `file`：云空间文件
- `wiki`：知识库节点（部分支持）
- `bitable`：多维表格
- `docx`：文档（暂不支持）
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::

::: note
**提示** `wiki`: 知识库节点 暂不支持以下设置：
- `external_access`: 是否允许分享到租户外开关
- `share_entity`: 谁可以添加和管理协作者
- `invite_external`: 非所有权限者/所有者是否允许邀请外部人
- `link_share_entity`: 链接共享
  - `tenant_readable`: 获得链接的任何人可阅读
  - `tenant_editable`: 获得链接的任何人可编辑
:::

### 请求体

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
	<md-text type="field-name" >external_access</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	是否允许分享到租户外开关

**示例值**：true
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >security_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	可创建副本/打印/导出/复制设置

**示例值**："anyone_can_view"

**可选值有**：
- `anyone_can_view`：所有可访问此文档的用户
- `anyone_can_edit`：有编辑权限的用户
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >comment_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	可评论设置

**示例值**："anyone_can_view"

**可选值有**：
- `anyone_can_view`：所有可访问此文档的用户
- `anyone_can_edit`：有编辑权限的用户
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >share_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	谁可以添加和管理协作者

**示例值**："anyone"

**可选值有**：
- `anyone`：所有可阅读或编辑此文档的用户
- `same_tenant`：组织内所有可阅读或编辑此文档的用户
- `only_full_access`：只有所有权限者可以
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >link_share_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	链接共享

**示例值**："tenant_readable"

**可选值有**：
- `tenant_readable`：组织内获得链接的人可阅读
- `tenant_editable`：组织内获得链接的人可编辑
- `anyone_readable`：获得链接的任何人可阅读（仅`external_access=true`时有效）
- `anyone_editable`：获得链接的任何人可编辑（仅`external_access=true`时有效）
- `closed`：关闭链接分享
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	<md-text type="field-name" >invite_external</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	否
	</md-td>
	<md-td>
	非所有权限者/所有者是否允许邀请外部人

**示例值**：true
	</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::



### 请求体示例

```json
{
    "external_access": true,
    "security_entity": "anyone_can_view",
    "comment_entity": "anyone_can_view",
    "share_entity": "anyone",
    "link_share_entity": "tenant_readable",
    "invite_external": true
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
	&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >permission_public</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >permission_public</md-text>
	</md-td>
	<md-td>
	本次更新后的文档公共设置
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >external_access</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	是否允许分享到租户外开关
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >security_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	可创建副本/打印/导出/复制设置

**可选值有**：
- `anyone_can_view`：所有可访问此文档的用户
- `anyone_can_edit`：有编辑权限的用户
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >comment_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	可评论设置

**可选值有**：
- `anyone_can_view`：所有可访问此文档的用户
- `anyone_can_edit`：有编辑权限的用户
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >share_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	谁可以添加和管理协作者

**可选值有**：
- `anyone`：所有可阅读或编辑此文档的用户
- `same_tenant`：组织内所有可阅读或编辑此文档的用户
- `only_full_access`：只有所有权限者可以
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >link_share_entity</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >string</md-text>
	</md-td>
	<md-td>
	链接共享

**可选值有**：
- `tenant_readable`：组织内获得链接的人可阅读
- `tenant_editable`：组织内获得链接的人可编辑
- `anyone_readable`：获得链接的任何人可阅读（仅`external_access=true`时有效）
- `anyone_editable`：获得链接的任何人可编辑（仅`external_access=true`时有效）
- `closed`：关闭链接分享
	</md-td>
</md-tr>


<md-tr>
	<md-td>
	&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >invite_external</md-text>
	</md-td>
	<md-td>
	<md-text type="field-type" >boolean</md-text>
	</md-td>
	<md-td>
	非所有权限者/所有者是否允许邀请外部人
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
        "permission_public": {
            "external_access": true,
            "security_entity": "anyone_can_view",
            "comment_entity": "anyone_can_view",
            "share_entity": "anyone",
            "link_share_entity": "tenant_readable",
            "invite_external": true
        }
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
  <md-td>1061001</md-td>
  <md-td>internal error</md-td>
  <md-td>服务内部错误，包括超时，错误码没处理。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1061002</md-td>
  <md-td>params error.</md-td>
  <md-td>请检查请求参数是否正确，如：`member_id`是否正确、协作者是否真实存在等。</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1061003</md-td>
  <md-td>not found.</md-td>
  <md-td>请确认对应上传节点是否存在。</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1061004</md-td>
  <md-td>forbidden.</md-td>
  <md-td>请确认当前身份是否有对应上传节点的的权限，如用户是否有上传到指定doc的编辑权限。</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1061005</md-td>
  <md-td>auth failed.</md-td>
  <md-td>请使用正确身份访问该接口。</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1066001</md-td>
  <md-td>Internal Error</md-td>
  <md-td>服务内部错误，包括超时，错误码没处理。</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1066002</md-td>
  <md-td>Concurrency error, please retry</md-td>
  <md-td>服务内部错误，请重试。</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




