---
document_id: '7270779605450850310'
directory_id: '7270719451987705862'
title: Service.Permission.onDocumentPermissionChange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Permission.onDocumentPermissionChange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Permission
- Service.Permission.onDocumentPermissionChange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Permission.onDocumentPermissionChange
---

# Service.Permission.onDocumentPermissionChange
监听用户对某篇文档的权限变化，该方法为同步调用。
  
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
