---
document_id: '7270779700749025285'
directory_id: '7270719284443561989'
title: getBlockRefById
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getblockrefbyid
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- getBlockRefById
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getblockrefbyid
---

# getBlockRefById
根据 block id 获取指定 Block 的引用，该方法是同步调用。
  
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
<md-td>无需权限</md-td>
<md-td>&nbsp;所有视图<br>[关于视图请参见概念说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/02-cloud-doc-block-noun-explanation)</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

文档的引用以及文档块（Block）id。
| **名称**  | **数据类型**                                                                                                     | **是否必填** | **描述**       |
| ------- | ------------------------------------------------------------------------------------------------------------ | -------- | ------------ |
| docRef  | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 文档引用         |
| blockId | number                                                                                                       | 是        | 文档块（Block）id |
  

## 输出

返回指定指定 Block 的引用，它是一个 [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = DocMiniApp.getBlockRefById(docRef, 3);
console.log('debug',blockRef);
```

### 返回示例

```json
{
  "docRef":
  {
    "docToken": "MesfdeCCOoULp7x83UQcMlx****"
  },
  "blockId": 3
}
```
