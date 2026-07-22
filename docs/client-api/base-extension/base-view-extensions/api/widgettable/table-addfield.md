---
document_id: '7260081693314809862'
directory_id: '7258197168736665606'
title: table.addField
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_addfield
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetTable
- table.addField
document_type: GuideDocumentType
updated_at: 2024-01-05T03:43:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/table/table_addfield
---

# table.addField
新增一列字段。字段名字重复将抛出错误。

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
addField(fieldConfig)
```

| 名称 | 数据类型 | 是否必填 | 描述 |
| --- | --- | --- | --- |
| fieldConfig | [IAddFieldConfig](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iaddfieldconfig) | 是 | 需要新增的字段和它的配置 |



## 输出
Promise字符串：新增的字段的id。
## 示例代码
### 调用示例

```js
  const selection = await bitable.base.getSelection();
  const table = await bitable.base.getTableById(selection.tableId); // 获取当前数据表实例

  const res = await table.addField({ // 新增一列多行文本类型的字段
    type: FieldType.Text,
    name: '多行文本2',
  })
```


### 返回示例
res:
```js
'fldILo3L59'
```
