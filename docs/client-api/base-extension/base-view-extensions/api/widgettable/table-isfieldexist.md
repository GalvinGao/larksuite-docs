---
document_id: '7260082411118854149'
directory_id: '7258197168736665606'
title: table.isFieldExist
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_isfieldexist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.isFieldExist
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_isfieldexist
---

# table.isFieldExist
字段是否存在。

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
isFieldExist(fieldId)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: auto">
    <col style="width: 100px">
    <col style="width: 200px">
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
      	<md-td>fieldId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>字段id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise布尔值。
## 示例代码
### 调用示例
```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const field = await table.getFieldByName('多行文本');


const res = await table.isFieldExist(field.id)
console.log(res) // true
```
### 返回示例
res:
```js
true
```
