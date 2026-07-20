---
document_id: '7270779605450031110'
directory_id: '7270719284443299845'
title: Interaction.onDataChange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.onDataChange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Interaction
- Interaction.onDataChange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.onDataChange
---

# Interaction.onDataChange
监听云文档小组件的 Interaction 数据变化，该方法为异步调用。
  
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
<md-td>- 正文小组件
- 全屏视图
- 模态框视图
- 弹窗视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

Interaction 数据变化监听器
| **名称**  | **数据类型**                 | **是否必填** | **描述**              |
| ------- | ------------------------ | -------- | ------------------- |
| handler | InteractionChangeHandler | 是        | Interaction 数据变化监听器 |

### InteractionChangeHandler

```js
type InteractionChangeHandler = (interactionData: InteractionData) => void;
```
| **名称**          | **数据类型**        | **是否必填** | **描述**            |
| --------------- | --------------- | -------- | ----------------- |
| interactionData | InteractionData | 是        | 变更的Interaction 数据 |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const docRef = await DocMiniApp.getActiveDocumentRef();
 
const interactionDataChangehandler = (interactionData: InteractionData) => {
        //对data做处理
   };
useEffect(() => {
        DocMiniApp.Interaction.onDataChange(interactionDataChangehandler);
    return () => {
        DocMiniApp.Interaction.offDataChange(interactionDataChangehandler);    
    };
});
```
