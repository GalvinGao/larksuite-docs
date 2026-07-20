---
document_id: '7260082411118788613'
directory_id: '7258197168736665606'
title: table.deleteField
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_deletefield
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.deleteField
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_deletefield
---

# table.deleteField
根据字段id删除某个字段。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
deleteField(fieldId)
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
        <md-td>需要被删除的字段id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise布尔值：删除成功为true。
## 示例代码
### 调用示例

```js
    const selection = await bitable.base.getSelection();
    const table = await bitable.base.getTableById(selection.tableId); // 获取数据表实例
    const field = await table.getFieldByName('多行文本 2');// 获取字段实例

    const res = await table.deleteField(field.id)

```


### 返回示例
res:
```js
true
```
