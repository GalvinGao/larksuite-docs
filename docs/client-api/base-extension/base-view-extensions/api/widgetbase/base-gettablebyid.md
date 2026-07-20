---
document_id: '7260082411118592005'
directory_id: '7258197168736698374'
title: base.getTableById
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_gettablebyid
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.getTableById
document_type: GuideDocumentType
updated_at: 2024-05-15T02:45:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_gettablebyid
---

# base.getTableById
根据id获取数据表实例。

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
getTableById(tableId)
```

| 名称     | 数据类型 |  是否必填 | 描述 |
| ----------- | ----------- | ------- | --------- |
| tableId      | string       | 是      |	当前base下某个数据表id      |

## 输出
Promise[数据表实例](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgettable)。获取tableId对应的表格失败时抛出错误。
## 示例代码

```js
await bitable.base.getTableById('t_id');
```

