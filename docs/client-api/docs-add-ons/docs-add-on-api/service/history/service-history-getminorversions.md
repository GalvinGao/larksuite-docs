---
document_id: '7270779605447196678'
directory_id: '7270719284443578373'
title: Service.History.getMinorVersions
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.History.getMinorversions
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- History
- Service.History.getMinorversions
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.History.getMinorversions
---

# Service.History.getMinorVersions
获取历史记录的小版本列表，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可写 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

无需传入参数。
  

## 输出

Promise<any[]>
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const minorVersions = await DocMiniApp.Service.History.getMinorVersions();
console.log('debug', minorVersions);
```
