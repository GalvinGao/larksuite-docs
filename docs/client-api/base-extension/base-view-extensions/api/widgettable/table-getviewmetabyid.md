---
document_id: '7260081693314613254'
directory_id: '7258197168736665606'
title: table.getViewMetaById
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getviewmetabyid
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.getViewMetaById
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_getviewmetabyid
---

# table.getViewMetaById
根据视图id获取视图元信息。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::




## 输出
字段元信息对象。
## 示例代码
### 调用示例

```js
const viewMetaList = await table.getViewMetaList(); // 获取数据表所有视图信息

const res = await table.getViewMetaById(viewMetaList[0].id);
```


### 返回示例
res:
```js
{
  "name": "表格",
  "id": "vewDUmweGB",
  "type": 1
}
```
