---
document_id: '7260082411119034373'
directory_id: '7258197168736665606'
title: table.getRecordById
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getrecordbyid
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getRecordById
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:52Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getrecordbyid
---

# table.getRecordById
获取指定记录 id 对应的记录。

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
getRecordById(recordId)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: auto">
    <col style="width: auto">
    <col style="width: auto">
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
      	<md-td>recordId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>记录id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise对象，包含该记录每个字段的值。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList(); // 获取recordId列表

const res = await table.getRecordById(recordIds[0]);

```


### 返回示例
```js
{
  "fields": {
    "fldgyVuQbo": [
      {
        "type": "text",
        "text": "123123"
      }
    ],
    "fld56pgsZo": [
      {
        "type": "text",
        "text": "22"
      }
    ]
  }
}
```
