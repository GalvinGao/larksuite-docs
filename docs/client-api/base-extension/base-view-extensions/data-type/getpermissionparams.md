---
document_id: '7260082411118608389'
directory_id: '7258197168736714758'
title: GetPermissionParams
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/getpermissionparams
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- Data Type
- GetPermissionParams
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:35Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/getpermissionparams
---

# GetPermissionParams
相关api：[base.getPermission](/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/base_getbasepermission)。

```js
type GetPermissionParams = BasePermissionParams | 
  TablePermissionParams | 
  RecordPermissionParams | 
  FieldPermissionParams | 
  CellPermissionParams;

interface BasePermissionParams {
    entity: PermissionEntity.Base;
    type: BaseOperation;
}
interface TablePermissionParams {
    entity: PermissionEntity.Table;
    param: {
        tableId?: string;
    };
    type: OperationType;
}
interface RecordPermissionParams {
    entity: PermissionEntity.Record;
    param: {
        tableId: string;
        recordId?: string;
    };
    type: OperationType;
}
interface FieldPermissionParams {
    entity: PermissionEntity.Field;
    param: {
        tableId: string;
        fieldId?: string;
    };
    type: OperationType;
}
  
interface CellPermissionParams {
    entity: PermissionEntity.Cell;
    param: {
        tableId: string;
        recordId?: string;
        fieldId?: string;
    };
    type: OperationType;
}
 ```
 - 操作类型：`OperationType`
```js
enum OperationType {
    Visible = "visible",
    Editable = "editable",
    Addable = "addable",
    Deletable = "deletable",
    Copyable = "copyable",
    Printable = "printable",
    Manageable = "manageable"
}
```
