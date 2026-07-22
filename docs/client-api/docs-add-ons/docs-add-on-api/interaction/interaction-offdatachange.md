---
document_id: '7270779605450899462'
directory_id: '7270719284443299845'
title: Interaction.offDataChange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.offDataChange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Interaction
- Interaction.offDataChange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.offDataChange
---

# Interaction.offDataChange
取消云文档小组件的 Interaction 数据变化监听，该方法为异步调用。

## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | - 正文小组件<br>- 全屏视图<br>- 模态框视图<br>- 弹窗视图 | - PC<br>- 移动端 | 演示模式 |



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
 
const interactionDataChangehandler = (interactionData: any) => {
        //对data做处理
   };
useEffect(() => {
    DocMiniApp.Interaction.onDataChange(interactionDataChangehandler);
    return () => {
        DocMiniApp.Interaction.offDataChange(interactionDataChangehandler);    
    };
});
```
