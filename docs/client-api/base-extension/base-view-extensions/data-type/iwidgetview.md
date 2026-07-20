---
document_id: '7260081693314826246'
directory_id: '7258197168736714758'
title: IWidgetView
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetview
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- Data Type
- IWidgetView
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:31Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetview
---

# IWidgetView

视图实例。类型如下：
<br><br>
```js
interface IWidgetView{
    id: string;
    tableId: string;
      /** 获取字段名 */
    getName(): Promise<string>;
    /** 获取视图类型 */
    getType(): Promise<ViewType>;
    /** 获取字段列表（有序） */
    getFieldMetaList(): Promise<IFieldMeta[]>;
    /** 获取记录 ID 列表 */
    getVisibleRecordIdList(): Promise<(string | undefined)[]>;
}
```
