---
document_id: '7270779605450702854'
directory_id: '7270719284443611141'
title: Env.PresentationMode.getPresentationMode
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.PresentationMode.getPresentationMode
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Env
- PresentationMode
- Env.PresentationMode.getPresentationMode
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Env.PresentationMode.getPresentationMode
---

# Env.PresentationMode.getPresentationMode
获取当前小应用所在的文档演示模式 ，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

无需传入参数。
  

## 输出

异步返回文档演示模式类型
| **key**    | **value**  | **描述** |
| ---------- | ---------- | ------ |
| DOCUMENT   | document   | 文档演示时  |
| NONE       | none       | 非演示模式  |
| PAGINATION | pagination | 分页演示时  |
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const presentationMode = await DocMiniApp.Env.PresentationMode.getPresentationMode();
console.log('debug', presentationMode);
```

### 返回示例

```
'none'
```
