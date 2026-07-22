---
document_id: '7270779605446983686'
directory_id: '7270719284443611141'
title: Env.PresentationMode.isInPresentationMode
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.PresentationMode.isInPresentationMode
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Env
- PresentationMode
- Env.PresentationMode.isInPresentationMode
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.PresentationMode.isInPresentationMode
---

# Env.PresentationMode.isInPresentationMode
判断当前小应用是否在演示模式中，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

无需传入参数。
  

## 输出

异步返回一个 boolean值
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const isInPresentationMode = await DocMiniApp.Env.PresentationMode.isInPresentationMode();
console.log('debug', isInPresentationMode);
```

### 返回示例

```
false
```
