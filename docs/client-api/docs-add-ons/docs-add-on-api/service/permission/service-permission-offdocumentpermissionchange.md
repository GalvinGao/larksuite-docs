---
document_id: '7270779605447278598'
directory_id: '7270719451987705862'
title: Service.Permission.offDocumentPermissionChange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Permission.offDocumentPermissionChange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Permission
- Service.Permission.offDocumentPermissionChange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Permission.offDocumentPermissionChange
---

# Service.Permission.offDocumentPermissionChange
取消用户对某篇文档的权限变化的监听，该方法为同步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

| **名称**   | **数据类型**                                                                                                | **是否必填** | **描述**          |
| -------- | ------------------------------------------------------------------------------------------------------- | -------- | --------------- |
| docRef   | [DocumentRef](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/DocumentRef) | 是        | 指定的文档引用         |
| callback | Function                                                                                                | 是        | 指定文档的权限变化时的回调函数 |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const documentRef = useRef<DocumentRef | null>(null);
const documentPermissionHandler = (permission: DocumentPermission) => {
    console.log('debug', permission);
}
useEffect(() => {
    const fn = async () => {
        documentRef.current = await DocMiniApp.getActiveDocumentRef();
        DocMiniApp.Service.Permission.onDocumentPermissionChange(documentRef.current, documentPermissionHandler);
    }
    fn();
    return () => {
        documentRef.current && DocMiniApp.Service.Permission.onDocumentPermissionChange(documentRef.current, documentPermissionHandler);
    }
});
```

### 返回示例

无
