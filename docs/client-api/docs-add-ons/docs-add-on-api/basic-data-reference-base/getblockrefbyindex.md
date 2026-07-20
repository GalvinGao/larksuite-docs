---
document_id: '7270779700748779525'
directory_id: '7270719284443561989'
title: getBlockRefByIndex
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getblockrefbyindex
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Basic Data Reference - Base
- getBlockRefByIndex
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/getblockrefbyindex
---

# getBlockRefByIndex
根据父 Block 引用以及索引获取指定 Block 的引用，该方法是同步调用。
  
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
<md-td>&nbsp;所有视图<br>[关于视图请参见概念说明](/document/uAjLw4CM/uYjL24iN/docs-add-on/02-cloud-doc-block-noun-explanation)
</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

父 Block 引用以及该block在父 Block 的位置索引
| **名称**         | **数据类型**                                                                                                  | **是否必填** | **描述**         |
| -------------- | --------------------------------------------------------------------------------------------------------- | -------- | -------------- |
| parentBlockRef | [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef) | 是        | 父 Block 引用     |
| childIndex     | number                                                                                                    | 是        | 在父 Block 的位置索引 |
  

## 输出

返回指定 Block 的引用，它是一个 [BlockRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/BlockRef)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = DocMiniApp.getBlockRefByIndex({ docRef, blockId: 1 }, 0);
console.log('debug',blockRef);
```

### 返回示例

```json
{
    "docRef":{"docToken":"docx token"},
    "parentRef":{"docRef":{"docToken":"docx token"},"blockId":1},
    "index":0
}
```
