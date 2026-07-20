---
document_id: '7260082411118460933'
directory_id: '7258197168736698374'
title: base.getPermission
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_getbasepermission
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.getPermission
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:16Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_getbasepermission
---

# base.getPermission
获取指定实体（Base/Table/Field/Record/Cell）的权限信息，true 代表有权限，false 代表无权限。

:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::



## 输入
```
getPermission(params)
```

| 名称     | 数据类型 |  是否必填 | 描述 |
| ----------- | ----------- | ------- | --------- |
| params      | [GetPermissionParams](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/getpermissionparams)| 是 |	需要获取权限的实体和权限      |

## 输出
Promise布尔值，为true的时候表示拥有权限。
## 示例代码
- 判断base实体编辑权限

```js
const permision = await bitable.base.getPermission({
  entity: PermissionEntity.Base, // 指定base实体
  type: OperationType.Editable, // 编辑权限
})
console.log(permision) // true
```

- 判断数据表编辑权限
```js
const selection = await bitable.base.getSelection() // 获取当前所选的信息
const p2 = await bitable.base.getPermission({
  entity: PermissionEntity.Table, // 判断数据表的权限
  type: OperationType.Editable, // 编辑权限
  param: {
    tableId: selection.tableId, // 指定数据表id
  }
})
console.log(p2) // true
```

- 判断字段编辑权限
```js
const selection = await bitable.base.getSelection() // 获取当前所选的信息
const table = await bitable.base.getTableById(selection.tableId);// 获取table实例
const fieldMetaList = await table.getFieldMetaList();

const p2 = await bitable.base.getPermission({
  entity: PermissionEntity.Field, // 判断字段的权限
  type: OperationType.Editable, // 编辑权限
  param: {
    tableId: selection.tableId, // 指定数据表id
    fieldId: fieldMetaList[0].id, // 指定字段id
  }
})
console.log(p2) // true

```



