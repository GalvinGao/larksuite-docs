---
document_id: '7260082411118051333'
directory_id: '7258197168736698374'
title: base.onSelectionChange
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_onselectionchange
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.onSelectionChange
document_type: GuideDocumentType
updated_at: 2024-05-15T02:46:08Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_onselectionchange
---

# base.onSelectionChange
监听用户当前选中（表格，单元格，视图，记录，字段）事件，将返回一个取消监听函数。


## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::


## 输入
描述一下
```js
const off = base.onSelectionChange((event) => {})
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
      	<md-td>event</md-td>
        <md-td>Selection </md-td>
        <md-td>否</md-td>
        <md-td>当前所选的相关信息。</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::
Selection:
```js
{
  data: {
    baseId: string | null;
    tableId: string | null;
    viewId: string | null;
    fieldId: string | null;
    recordId: string | null;
}
}
```

## 输出
取消监听的函数。


## 示例代码

```js
const off = bitable.base.onSelectionChange((event) => {
	off(); // 取消监听所选数据表变化
	console.log('所选数据表变化',event)
  /**
  {
  "data": {
    "tableId": "tblrWsfkG3Mh3BlM",
    "viewId": null,
    "fieldId": null,
    "recordId": null,
    "baseId": "QtTUb1dWewBmtcsyacTcoEpUnzf"
  }
}
  */
})

```

