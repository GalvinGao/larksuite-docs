---
document_id: '7270779605447376902'
directory_id: '7270719284443185157'
title: Service.Fold.toggleFold
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fold.toggleFold
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Fold
- Service.Fold.toggleFold
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fold.toggleFold
---

# Service.Fold.toggleFold
对某个 Block 设置指定的折叠状态，该方法为异步调用。
  
## 可用性说明
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>权限要求</md-th>
<md-th>视图可用说明</md-th>
<md-th>平台可用</md-th>
<md-th>场景</md-th></md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>可读</md-td>
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

| **名称**   | **数据类型**                                                                 | **是否必填** | **描述**       |
| -------- | ------------------------------------------------------------------------ | -------- | ------------ |
| blockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 指定的 Block 引用 |
| fold     | boolean                                                                  | 是        | 切换成折叠的状态     |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const documentRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = await DocMiniApp.getBlockRefById(documentRef, 3);
DocMiniApp.Service.Fold.toggleFold(blockRef, true);
```

### 返回示例

无
