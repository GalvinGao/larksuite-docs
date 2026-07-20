---
document_id: '7260082411118493701'
directory_id: '7258197168736649222'
title: view.getType
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_gettype
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetView
- view.getType
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_gettype
---

# view.getType
获取视图类型。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
promise数字：视图类型。
视图类型如下：
```js
ViewType {
    NotSupport = 0,
    Grid = 1, // 表格
    Kanban = 2, //看板
    Form = 3, // 表单
    Gallery = 4, //画册
    Gantt = 5, //甘特图
    Hierarchy = 6,
    Calendar = 7, //日历
    WidgetView = 100
}
```
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId);
const viewMetaList = await table.getViewMetaList();
const view = await table.getViewById(viewMetaList[0].id);

const res = await view.getType();

```


### 返回示例
res:
```js
1
```
