---
document_id: '7270779605450604550'
directory_id: '7270719284443283461'
title: Block.TextualBlock.highlightTexts
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/文档块-Block/textualblock/blocktextualblockhighlighttexts
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Block
- TextualBlock
- Block.TextualBlock.highlightTexts
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/文档块-Block/textualblock/blocktextualblockhighlighttexts
---

# Block.TextualBlock.highlightTexts
高亮文本，该方法为异步调用。
  
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

文本数据以及指定处理的文本范围，不指定默认全部
| **名称**           | **数据类型**                                                                                                                                               | **是否必填** | **描述**         |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | -------- | -------------- |
| highlightTextRef | HighlightTextRef[] | 是        | 需要高亮指定范围的文本集数据 |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
const selectedBlocks = await DocMiniApp.Selection.getSelectedBlocks(docRef);
const highlightRefs = selectedBlocks.map((item: any) => {
  if (item.type === 'text') {
    const textRef = { ...item.ref, range: [1, 3] };
    return { ...textRef, style: { color: 'R500' } };
  }
});
await DocMiniApp.Block.TextualBlock.highlightTexts(highlightRefs);
```
