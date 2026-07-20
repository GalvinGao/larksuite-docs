---
document_id: '7260082411118919685'
directory_id: '7258197168736665606'
title: table.isViewExist
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_isviewexist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.isViewExist
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_isviewexist
---

# table.isViewExist
判断视图是否存在。

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
isViewExist(viewId)
```

:::html
<md-table>
  <colgroup>
    <col style="width: auto">
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
      	<md-td>viewId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>视图id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise布尔值，为true的时候表示视图存在。
## 示例代码
### 调用示例
```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const viewMetaList =await table.getViewMetaList();

const res = await table.isViewExist(viewMetaList[0].id)
console.log(res)//true
```

### 返回示例
res:
```js
true
```



