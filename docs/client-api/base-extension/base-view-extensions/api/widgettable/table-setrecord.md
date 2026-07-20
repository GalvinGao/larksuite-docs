---
document_id: '7260081693314596870'
directory_id: '7258197168736665606'
title: table.setRecord
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setrecord
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.setRecord
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:33Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setrecord
---

# table.setRecord
修改一条记录。

## 权限要求
:::html
<md-alert type="warn">

<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
setRecord(recordId, recordValues)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: 300px">
    <col style="width: 100px">
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
        <md-td>需要修改的记录的id</md-td>
      </md-tr>
      <md-tr>
      	<md-td>recordValues</md-td>
        <md-td>
        {<br>
&nbsp;&nbsp;fields: {<br>
&nbsp;&nbsp;&nbsp;&nbsp;[fieldId: string]: [IOpenCellValue](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iopencellvalue)<br>
&nbsp;&nbsp;}<br>
}
        
        </md-td>
        <md-td>否</md-td>
        <md-td>需要修改的记录的字段和它们的值</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise字符串，被修改的记录id。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList(); // 获取所有行id
const field = await table.getFieldByName('多行文本'); // 选择多行文本字段

const res = await table.setRecord(recordIds[0],{
    fields: {
     [field.id]: [
        {
                type: 'text',
                text: '12345'
            }
         ]
    }
})
```


### 返回示例
res:
```js
'recqrO5hl1'
```
