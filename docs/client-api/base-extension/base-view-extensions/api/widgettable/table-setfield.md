---
document_id: '7260082411118870533'
directory_id: '7258197168736665606'
title: table.setField
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setfield
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.setField
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_setfield
---

# table.setField
修改字段属性。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
setField(fieldId, fieldConfig)
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
        <md-td>需要被修改的字段id</md-td>
      </md-tr>
      <md-tr>
      	<md-td>fieldConfig</md-td>
        <md-td>[IAddFieldConfig](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iaddfieldconfig)</md-td>
        <md-td>是</md-td>
        <md-td>需要被修改的字段的属性</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise布尔值：被修改的字段id。
## 示例代码
### 调用示例

```js
    const selection = await bitable.base.getSelection();
    const table = await bitable.base.getTableById(selection.tableId);// 获取当前数据表实例
    const field = await table.getFieldByName('多行文本');

    const res = await table.setField(field.id,{
        name: '修改后的字段名',
    })
```


### 返回示例
res:
```js
'fldgyVuQbo'
```
