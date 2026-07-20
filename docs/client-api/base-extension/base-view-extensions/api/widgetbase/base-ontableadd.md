---
document_id: '7260081693314629638'
directory_id: '7258197168736698374'
title: base.onTableAdd
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_ontableadd
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- WidgetBase
- base.onTableAdd
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:16Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_ontableadd
---

# base.onTableAdd
监听 Table 添加事件，将返回一个取消监听函数。

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
const off = base.onTableAdd((event) => {})
```

| 名称     | 数据类型 |  是否必填 | 描述 |
| ----------- | ----------- | ------- | --------- |
| event      | {data:{}}       | 否      |	data为一个空对象，后续将支持更多信息。      |

## 输出
取消监听的函数。
## 示例代码

```js
const off = base.onTableAdd((event) => {
	off(); // 监听一次数据表新增事件
	console.log('新增了一个数据表')
})

```
