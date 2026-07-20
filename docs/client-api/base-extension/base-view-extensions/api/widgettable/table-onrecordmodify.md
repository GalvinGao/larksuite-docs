---
document_id: '7260082411118526469'
directory_id: '7258197168736665606'
title: table.onRecordModify
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onrecordmodify
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.onRecordModify
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onrecordmodify
---

# table.onRecordModify
监听记录修改事件，将返回一个取消监听函数。如果记录修改前后是一样的，则不会触发回调函数。

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
onRecordModify((event) => {})
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: auto">
    <col style="width: 150px">
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
      	<md-td>event</md-td>
        <md-td>
```js
{
  "name": string,
  "type": "publish",
  "data": {
    "recordId": string,
    "fieldIds": string[]
  },
}         
        </md-td>
        <md-td>否</md-td>
        <md-td>fieldIds为修改的字段id数组</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
取消监听的函数。
## 示例代码

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const recordIds = await table.getRecordIdList();
const field = await table.getFieldByName('多行文本')

const off = table.onRecordModify((event) => { // 监听记录修改事件
  off();
  console.log('event:', event);
})


table.setRecord(recordIds[0],{ // 修改某条记录的多行文本字段
  fields:{
    [field.id]:[{
      type:'text',
      text:'1234'+Math.random()
    }]
  }
})
```

