---
document_id: '7260082411118821381'
directory_id: '7258197168736649222'
title: view.getFieldMetaList
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_getfieldmetalist
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetView
- view.getFieldMetaList
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/view/view_getfieldmetalist
---

# view.getFieldMetaList
获取字段元信息列表，按当前视图里的字段顺序排列。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输出
Promise视图元信息对象数组。
## 示例代码
### 调用示例

```js
const selection = await bitable.base.getSelection();
const table = await bitable.base.getTableById(selection.tableId); // 获取当前table实例
const viewMetaList = await table.getViewMetaList(); // 获取talbe的视图元信息列表
const view = await table.getViewById(viewMetaList[0].id); // 根据视图id获取一个视图实例

const res = await view.getFieldMetaList()
console.log(res)
```


### 返回示例
res:
```js
[
  {
    "id": "fldgyVuQbo",
    "type": 1,
    "name": "多行文本",
    "property": {}
  },
  {
    "id": "fldcOsdR3H",
    "type": 17,
    "name": "附件",
    "property": {}
  },
  {
    "id": "fldNq8pk5c",
    "type": 3,
    "name": "单选",
    "property": {
      "options": []
    }
  }
]
```
