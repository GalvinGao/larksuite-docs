---
document_id: '7260082411118149637'
directory_id: '7258197168736698374'
title: base.getTableByName
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/base/base_gettablebyname
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.getTableByName
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/base/base_gettablebyname
---

# base.getTableByName
根据表名获取数据表实例。

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
getTableByName(name)
```

| 名称     | 数据类型 |  是否必填 | 描述 |
| ----------- | ----------- | ------- | --------- |
| name      | string       | 是      |	当前base下某个数据表的表名     |

## 输出
Promise[数据表实例](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgettable)。获取name对应的数据表失败时抛出错误。
## 示例代码

```js
const table = await bitable.base.getTableByName('数据表')
```

