---
document_id: '7270779605450752006'
directory_id: '7270719284443185157'
title: Service.Fold.getFoldStateMap
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fold.getFoldStateMap
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Fold
- Service.Fold.getFoldStateMap
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fold.getFoldStateMap
---

# Service.Fold.getFoldStateMap
获取整篇文档的 block 的折叠信息，该方法为异步调用。
  
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

| **名称** | **数据类型**                                                                    | **是否必填** | **描述**  |
| ------ | --------------------------------------------------------------------------- | -------- | ------- |
| docRef | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 指定的文档引用 |
  

## 输出

异步返回一个 [FoldStateMap](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/FoldStateMap/foldstatemap)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const documentRef = await DocMiniApp.getActiveDocumentRef();
const foldStateMap = await DocMiniApp.Service.Fold.getFoldStateMap(documentRef);
console.log('debug', foldStateMap);
```

### 返回示例

```json
{
    "2": {
        "show": true,
        "type": "isv",
        "id": 2
    },
    "3": {
        "id": 3,
        "type": "heading9",
        "show": true,
        "folded": true
    },
    "4": {
        "id": 4,
        "type": "text",
        "closestFolderId": 3,
        "foldedBy": 3,
        "foldedByType": "heading9",
        "show": false,
        "folded": false
    }
}
```
