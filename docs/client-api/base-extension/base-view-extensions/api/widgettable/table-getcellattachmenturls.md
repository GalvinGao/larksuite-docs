---
document_id: '7260082411118166021'
directory_id: '7258197168736665606'
title: table.getCellAttactmentUrls
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getcellattachmenturls
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getCellAttachmentUrls
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getcellattachmenturls
---

# table.getCellAttachmentUrls
获取附单元格件的 url，需要传入 附件token， fieldId 和 recordId 来获取 attachment url。

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
getCellAttachmentUrls(token, fieldId, recordId)
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
        <md-td>string[]</md-td>
        <md-td>是</md-td>
        <md-td>单元格内某些文件的token；一般是table.batchUploadFile的返回值，或者从某个附件字段获取</md-td>
      </md-tr>
       <md-tr>
      	<md-td>fieldId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>字段Id</md-td>
      </md-tr>
       <md-tr>
      	<md-td>recordId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>记录id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise字符串数组：token对应的url数组。
## 调用示例

```js

const file = new File(['Hello, World!'], 'hello.txt', { type: 'text/plain' });
const file2 = new File(['Hello, World!2'], 'hello2.txt', { type: 'text/plain' }); //创建文件
const tokens = await bitable.base.batchUploadFile([file,file2]);// 上传文件
const field = await table.getFieldByName('附件');
const records =await table.getRecordIdList();

await table.setCellValue(field.id,records[0],[ // 设置单元格的值
    {
        type:file.type,
        name: file.name,
        size: file.size,
        token: tokens[0],
        timeStamp: new Date().getTime(),
    },
    {
        type:file2.type,
        name: file2.name,
        size: file2.size,
        token: tokens[1],
        timeStamp: new Date().getTime(),
    }
])

console.log('tokens',tokens) // 查看文件上传的token

const res = await table.getCellAttachmentUrls(tokens,field.id,records[0])

console.log(res); // ['https://internal.....', 'https://internal.....']


```
