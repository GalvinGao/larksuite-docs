---
document_id: '7270779700748959749'
directory_id: '7270719284443529221'
title: Service.Fullscreen.exitFullscreen
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fullscreen.exitFullscreen
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Fullscreen
- Service.Fullscreen.exitFullscreen
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fullscreen.exitFullscreen
---

# Service.Fullscreen.exitFullscreen
使小应用退出全屏状态，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 全屏视图 | PC | \- |



## 输入

无需传入参数。
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.Service.Fullscreen.exitFullscreen();
```

### 返回示例

无
