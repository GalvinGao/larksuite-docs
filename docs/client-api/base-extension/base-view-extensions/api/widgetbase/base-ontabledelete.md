---
document_id: '7260082411119067141'
directory_id: '7258197168736698374'
title: base.onTableDelete
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_ontabledelete
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.onTableDelete
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:12Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_ontabledelete
---

# base.onTableDelete
监听数据表删除事件，将返回一个取消监听函数。

## 权限要求
:::html
<md-alert type="warn">
开启以下任一权限
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>
<md-perm name="bitable:app:readonly" desc="查看、评论和导出多维表格" support_app_types="custom,isv">查看、评论和导出多维表格</md-perm>
</md-alert>
:::


## 输入
```js
const off = base.onTableDelete((event) => {})
```

| 名称     | 数据类型 |  是否必填 | 描述 |
| ----------- | ----------- | ------- | --------- |
| event      | {data:{}}       | 否      |	data为一个空对象，后续将支持更多信息。      |

## 输出
取消监听的函数。
## 示例代码

```js
const off = base.onTableDelete((event) => {
	off(); // 监听一次数据表删除事件
	console.log('删除了一个数据表')
})

```
