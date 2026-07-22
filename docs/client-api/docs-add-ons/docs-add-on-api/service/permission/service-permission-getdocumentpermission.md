---
document_id: '7270779605447426054'
directory_id: '7270719451987705862'
title: Service.Permission.getDocumentPermission
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Permission.getDocumentPermission
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Permission
- Service.Permission.getDocumentPermission
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Permission.getDocumentPermission
---

# Service.Permission.getDocumentPermission
获取当前用户对某篇文档的权限，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

| **名称** | **数据类型**                                                                    | **是否必填** | **描述**  |
| ------ | --------------------------------------------------------------------------- | -------- | ------- |
| docRef | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 指定的文档引用 |

## 输出

异步返回一个 [DocumentPermission](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentPermission)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
 const ref = await DocMiniApp.getActiveDocumentRef();
const permission = await DocMiniApp.Service.Permission.getDocumentPermission(ref);
console.log('debug', permission);
```

### 返回示例

```json
{
    "editable": true,
    "copyable": true,
    "commentable": true
}
```
