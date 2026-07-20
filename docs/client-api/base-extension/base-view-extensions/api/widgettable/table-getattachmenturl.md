---
document_id: '7260082411118723077'
directory_id: '7258197168736665606'
title: table.getAttachmentUrl
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getattachmenturl
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getAttachmentUrl
document_type: GuideDocumentType
updated_at: 2023-07-26T11:08:02Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getattachmenturl
---

# table.getAttachmentUrl
获取附件的 url，在开启高级权限的文档下，需要传入 fieldId 和 recordId 来获取 attachment url, 普通文档这两个参数可传可不传都可以获取对应的 url。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::


## 输入
```
getAttachmentUrl(token, fieldId, recordId)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 100px;">
    <col style="width: auto">
    <col style="width: auto">
    <col style="width: 400px">
  </colgroup>
	<md-thead> 
      <md-tr>
      	<md-th>名称</md-th>
        <md-th>数据类型</md-th>
        <md-th>是否必填</md-th>
        <md-th>描述</md-th>
      </md-tr>
  </md-thead> 
  	<md-tbody>
      <md-tr>
      	<md-td>token</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>文件的token，table.batchUploadFile的返回值，或者从某个附件字段获取</md-td>
      </md-tr>
       <md-tr>
      	<md-td>fieldId</md-td>
        <md-td>string</md-td>
        <md-td>高级权限文档下必传；普通文档可以不传</md-td>
        <md-td>字段Id</md-td>
      </md-tr>
       <md-tr>
      	<md-td>recordId</md-td>
        <md-td>string</md-td>
        <md-td>高级权限文档下必传；普通文档可以不传</md-td>
        <md-td>记录id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise字符串
## 示例代码
### 调用示例

```js
const file = new File(['Hello, World!'], 'hello.txt', { type: 'text/plain' }); //创建文件
const tokens = await bitable.base.batchUploadFile([file]);// 上传文件

const res = await table.getAttachmentUrl(tokens[0])
```

### 返回示例
res:
```
'https://internal-api-drive-stream.lark.cn/space/api/box/stream/download/authcode/?code=MWM2MWQ3NDA5MGE0ZGMyMmY2ZDI5NzFjNGRiZWE3ZjhfMzJiMzBhYzZlMDdjNmRkZDAxYzU1MjE5YmQyNTBiNjhfSUQ6NzI1NDkwMDIxMTc3OTQyMDE2Ml8xNjg5MTYzMDkwOjE2ODkyNDk0OTBfVjM'
```
