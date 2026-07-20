---
document_id: '7260082411118428165'
directory_id: '7258197168736665606'
title: table.getFieldById
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getfieldbyid
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getFieldById
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:00Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getfieldbyid
---

# base.getFieldById
获取某个 field 元信息。

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
getFieldById(fieldId)
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
      	<md-td>fieldId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>字段的id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise[字段实例](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetfield)

## 示例代码

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId); // 获取当前table

const fieldList = await table.getFieldList();// 获取当前table下所有字段

const field = await table.getFieldById(fieldList[0].id);

```
