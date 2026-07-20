---
document_id: '7260082411118542853'
directory_id: '7258197168736665606'
title: table.getViewById
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getviewbyid
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getViewById
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getviewbyid
---

# table.getViewById
根据id获取视图实例[IWidgetView](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetview)。

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
getViewById(viewId)
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
      	<md-td>viewId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>视图id</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise视图实例[IWidgetView](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetview)。

## 示例代码

```js
const viewMetaList = await table.getViewMetaList();// 获取数据表视图元信息列表

const res = await table.getViewById(viewMetaList[0].id);
```
