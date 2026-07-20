---
document_id: '7281229133446070277'
directory_id: '7281117330120753158'
title: UI.switchBlock
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/ui/ui_switchblock
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- API
- UI
- UI.switchBlock
document_type: GuideDocumentType
updated_at: 2023-09-22T02:26:40Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/api/ui/ui_switchblock
---

# ui.switchBlock
切换当前选中的表格

## 权限要求
:::html
<md-alert type="warn">
<md-perm name="bitable:app" desc="查看、评论、编辑和管理多维表格" support_app_types="custom,isv">查看、评论、编辑和管理多维表格</md-perm>

:::


## 输入
```
switchBlock(tableId)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 150px;">
    <col style="width: auto">
    <col style="width: auto">
    <col style="width: auto">
  </colgroup>
	<md-thead> 
      <md-tr>
      	<md-th>名称</md-th>
        <md-th>数据类型</md-th>
        <md-th>是否必填</md-th>
        <md-th>描述</md-th>
      </md-tr>
  </md-thead> 
  	<md-tbody>
      <md-tr>
      	<md-td>tableId</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>需要切换的数据表id等</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
Promise布尔值。
## 示例代码
### 调用示例

```js
const res = await bitable.ui.switchBlock('tbl7pHAfewssa8bw')
```


### 返回示例
res:
```js
true
```
