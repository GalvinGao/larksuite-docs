---
document_id: '7104193248416661510'
directory_id: '7072949713020534789'
title: 删除块
full_path: /ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete
breadcrumb:
- Server API
- Docs
- Document
- Block
- Delete Blocks
document_type: ReferenceDocumentType
updated_at: 2023-09-22T08:04:43Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete
---

# 删除块

指定需要操作的块，删除其指定范围的子块。如果操作成功，接口将返回应用删除操作后的文档版本号。{尝试一下}(url=/api/tools/api_explore/api_explore_config?project=docx&version=v1&resource=document.block.children&method=batch_delete)

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
**应用频率限制**：单个应用调用频率上限为每秒 3 次，超过该频率限制，接口将返回 HTTP 状态码 <font color="blue">400</font> 及错误码 <font color="blue">99991400</font>；

**文档频率限制**：单篇文档并发编辑上限为每秒 3 次，超过该频率限制，接口将返回 HTTP 状态码 <font color="blue">429</font>，编辑操作包括：
- 创建块
- 删除块
- 更新块
- 批量更新块

当请求被限频，应用需要处理限频状态码，并使用指数退避算法或其它一些频控策略降低对 API 的调用速率。
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
      <md-td>https://open.larksuite.com/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children/batch_delete</md-td>
    </md-tr>
    <md-tr>
      <md-th>HTTP Method</md-th>
      <md-td>DELETE</md-td>
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
            
      </md-th>
      <md-td>
            <md-perm name="docx:document" desc="创建及编辑新版文档" support_app_types="custom,isv" tags="">创建及编辑新版文档</md-perm>
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
或
<md-tag mode="inline" type="token-user">user_access_token</md-tag>

**值格式**："Bearer `access_token`"

**示例值**："Bearer u-7f1bcd13fc57d46bac21793a18e560"

[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use)

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
	<md-text type="field-name" >document_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	文档的唯一标识。对应新版文档 Token，[点击了解如何获取云文档 Token](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)。

**示例值**："doxcnePuYufKa49ISjhD8Ih0ikh"
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >block_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	父 Block 的唯一标识

**示例值**："doxcnO6UW6wAw2qIcYf4hZpFIth"
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
	<md-text type="field-name" >document_revision_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	否
	</md-dt-td>
	<md-dt-td>
	操作的文档版本，-1 表示文档最新版本。编辑文档需要持有文档的编辑权限。

**示例值**：-1

**默认值**：`-1`

**数据校验规则**：

- 最小值：`-1`
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
	操作的唯一标识，与接口返回值的 client_token 相对应，用于幂等的进行更新操作。此值为空表示将发起一次新的请求，此值非空表示幂等的进行更新操作。

**示例值**："fe599b60-450f-46ff-b2ef-9f6675625b97"
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
	<md-text type="field-name" >start_index</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	删除的起始索引（操作区间左闭右开）

**示例值**：0

**数据校验规则**：

- 最小值：`0`
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="0">
	<md-dt-td>
	<md-text type="field-name" >end_index</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	是
	</md-dt-td>
	<md-dt-td>
	删除的末尾索引（操作区间左闭右开）

**示例值**：1

**数据校验规则**：

- 最小值：`1`
	</md-dt-td>
</md-dt-tr>

  </md-dt-tbody>
</md-dt-table>
:::



### 请求体示例
:::html
<md-code-json>
{
    "start_index": 0,
    "end_index": 1
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
	<md-text type="field-name" >document_revision_id</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >int</md-text>
	</md-dt-td>
	<md-dt-td>
	当前删除操作成功后文档的版本号
	</md-dt-td>
</md-dt-tr>


<md-dt-tr level="1">
	<md-dt-td>
	<md-text type="field-name" >client_token</md-text>
	</md-dt-td>
	<md-dt-td>
	<md-text type="field-type" >string</md-text>
	</md-dt-td>
	<md-dt-td>
	操作的唯一标识，更新请求中使用此值表示幂等的进行此次更新
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
        "document_revision_id": 1,
        "client_token": "fe599b60-450f-46ff-b2ef-9f6675625b97"
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
  <md-td>400</md-td>
  <md-td>1770001</md-td>
  <md-td>invalid param</md-td>
  <md-td>确认传入的参数是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>404</md-td>
  <md-td>1770002</md-td>
  <md-td>not found</md-td>
  <md-td>确认文档是否已被删除</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770003</md-td>
  <md-td>resource deleted</md-td>
  <md-td>确认资源是否已被删除</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770004</md-td>
  <md-td>too many blocks in document</md-td>
  <md-td>确认文档 Block 数量是否超上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770005</md-td>
  <md-td>too deep level in document</md-td>
  <md-td>确认文档 Block 层级是否超上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770006</md-td>
  <md-td>schema mismatch</md-td>
  <md-td>确认文档结构是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770007</md-td>
  <md-td>too many children in block</md-td>
  <md-td>确认指定 Block 的 Children 数量是否超上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770008</md-td>
  <md-td>too big file size</md-td>
  <md-td>确认上传的文件尺寸是否超上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770010</md-td>
  <md-td>too many table column</md-td>
  <md-td>确认表格列数是否超上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770011</md-td>
  <md-td>too many table cell</md-td>
  <md-td>确认表格单元格数量是否超上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770012</md-td>
  <md-td>too many grid column</md-td>
  <md-td>确认 Grid 列数量是否超上限</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770013</md-td>
  <md-td>relation mismatch</md-td>
  <md-td>确认图片、文件等资源的关联关系是否正确，插入图片、文件需先上传素材</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770014</md-td>
  <md-td>parent children relation mismatch</md-td>
  <md-td>确认 Block 父子关系是否正确</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770015</md-td>
  <md-td>single edit with multi document</md-td>
  <md-td>确认 Block 所属文档与指定的 Document 是否相同</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770019</md-td>
  <md-td>repeated blockID in document</md-td>
  <md-td>确认 Document 中的 BlockID 是否有重复</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770020</md-td>
  <md-td>operation denied on copying document</md-td>
  <md-td>确认 Document 是否正在创建副本中</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770021</md-td>
  <md-td>too old document</md-td>
  <md-td>确认指定的 Document 版本过旧</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770022</md-td>
  <md-td>invalid page token</md-td>
  <md-td>确认查询参数中的 page_token 是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770024</md-td>
  <md-td>invalid operation</md-td>
  <md-td>确认操作是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770025</md-td>
  <md-td>operation and block not match</md-td>
  <md-td>确认指定 Block 应用对应操作是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770026</md-td>
  <md-td>row operation over range</md-td>
  <md-td>确认行操作下标是否越界</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770027</md-td>
  <md-td>column operation over range</md-td>
  <md-td>确认列操作下标是否越界</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770028</md-td>
  <md-td>block not support create children</md-td>
  <md-td>确认指定 Block 添加 Children 是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770029</md-td>
  <md-td>block not support to create</md-td>
  <md-td>确认指定 Block 是否支持创建</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770030</md-td>
  <md-td>invalid parent children relation</md-td>
  <md-td>确认指定操作其父子关系是否合法</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770031</md-td>
  <md-td>block not support to delete children</md-td>
  <md-td>确认指定 Block 是否支持删除 Children</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770033</md-td>
  <md-td>raw content size exceed limited</md-td>
  <md-td>纯文本内容大小超过限制</md-td>
</md-tr>


<md-tr>
  <md-td>400</md-td>
  <md-td>1770034</md-td>
  <md-td>operation count exceed limited</md-td>
  <md-td>当前请求中涉及单元格个数过多，请拆分成多次请求</md-td>
</md-tr>


<md-tr>
  <md-td>403</md-td>
  <md-td>1770032</md-td>
  <md-td>forbidden</md-td>
  <md-td>1. 确认操作者是否有该文档的权限。[点击了解什么是文档权限](/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#2431c595)
2. 确认操作者是否有被 mention 的文档的阅读权限。
3. 确认 mention 的用户是否在职且与当前操作者互为联系人。
4. 确认操作者是否具有群卡片的查看和分享权限。
5. 确认操作者是否具有访问指定的 Wiki 子目录的权限。
6. 确认操作者是否具有 OKR、ISV、Add-Ons 的查询权限。</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1771001</md-td>
  <md-td>server internal error</md-td>
  <md-td>服务器内部错误</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1771006</md-td>
  <md-td>mount folder failed</md-td>
  <md-td>挂载文档到云空间目录失败</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1771002</md-td>
  <md-td>gateway server internal error</md-td>
  <md-td>网关服务内部错误</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1771003</md-td>
  <md-td>gateway marshal error</md-td>
  <md-td>网关服务解析错误</md-td>
</md-tr>


<md-tr>
  <md-td>500</md-td>
  <md-td>1771004</md-td>
  <md-td>gateway unmarshal error</md-td>
  <md-td>网关服务反解析错误</md-td>
</md-tr>


<md-tr>
  <md-td>503</md-td>
  <md-td>1771005</md-td>
  <md-td>system under maintenance</md-td>
  <md-td>系统服务正在维护中</md-td>
</md-tr>

  </md-tbody>
</md-table>
:::




