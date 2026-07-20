---
document_id: '7260082411118690309'
directory_id: '7258197168736649222'
title: view.getVisibleRecordIdList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_getvisiblerecordidlist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetView
- view.getVisibleRecordIdList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_getvisiblerecordidlist
---

# view.getVisibleRecordIdList
获取可见的记录id列表。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise字符串数组。
## 示例代码
### 调用示例

```js
// 示例代码
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId); // 获取当前数据表实例
const viewMetaList = await table.getViewMetaList(); // 获取视图元信息列表
const view = await table.getViewById(viewMetaList[0].id); // 根据id随机获取一个视图实例

const res = await view.getVisibleRecordIdList()
console.log(res)
```


### 返回示例
res:
```js
[
  "rec1i6gIHj",
  "rec8Do63xI",
  "recFNRH5QF"
]
```
