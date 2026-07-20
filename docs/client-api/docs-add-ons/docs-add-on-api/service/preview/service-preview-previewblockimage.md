---
document_id: '7270779605450948614'
directory_id: '7270719284443414533'
title: Service.Preview.previewBlockImage
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Preview.previewBlockImage
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Preview
- Service.Preview.previewBlockImage
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Preview.previewBlockImage
---

# Service.Preview.previewBlockImage
以某个 Block 为入口唤起图片查看器，该方法为异步调用。
## 注意事项：
目前支持图片查看器的 BlockType 仅包括：
- IMAGE
- DIAGRAM
  
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
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const documentRef = await DocMiniApp.getActiveDocumentRef();
const blockRef = await DocMiniApp.getBlockRefById(documentRef, 6);
DocMiniApp.Service.Preview.previewBlockImage(blockRef);
```

### 返回示例

无
