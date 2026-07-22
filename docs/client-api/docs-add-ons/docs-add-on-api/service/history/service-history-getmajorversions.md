---
document_id: '7270779700748926981'
directory_id: '7270719284443578373'
title: Service.History.getMajorVersions
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.History.getMajorVersions
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- History
- Service.History.getMajorVersions
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.History.getMajorVersions
---

# Service.History.getMajorVersions
获取历史记录的大版本列表，该方法为异步调用。
  
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
const majorVersions = await DocMiniApp.Service.History.getMajorVersions();
console.log('debug', majorVersions);
```
