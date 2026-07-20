---
document_id: '7260082411118116869'
directory_id: '7258197168736665606'
title: table.onFieldModify
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onfieldmodify
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.onFieldModify
document_type: GuideDocumentType
updated_at: 2024-05-15T02:45:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_onfieldmodify
---

# table.onFieldModify
监听字段变化事件。将返回一个取消监听函数。

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
onFieldModify((event) => {})
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: auto">
    <col style="width: 100px">
    <col style="width: 150px">
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
    "fieldId": string,
    "fieldType": number,
    "tableId": string
  },
}
        </md-td>
        <md-td>否</md-td>
        <md-td>被编辑字段的信息</md-td>
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

const off = table.onFieldModify((event) => { // 监听字段变化事件。
  off();
  console.log('event:', event);
})

const fieldId = await table.addField({ // 新增字段不会触发字段变化事件
  type: FieldType.Text,
  name: '新增-4'
})

table.setField(fieldId,{ //修改字段名字
  name:'修改-1'
})
```


