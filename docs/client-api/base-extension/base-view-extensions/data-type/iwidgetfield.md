---
document_id: '7260082411118034949'
directory_id: '7258197168736714758'
title: IWidgetField
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetfield
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- Data Type
- IWidgetField
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:31Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/iwidgetfield
---

# IWidgetField
字段实例，类型如下：
<br><br>

```js
interface IWidgetField{
    id: string;
    tableId: string;
    /** 获取字段名 */
    getName(): Promise<string>;
    /** 获取字段类型 */
    getType(): Promise<FieldType>;
    /** 获取公式代理列类型 */
    getProxyType(): Promise<FieldType | void>;
    /** 获取 cellValue 并转化为 string 格式 */
    getCellString(recordId: string): Promise<string>;
    /** 获取当前 field meta 信息 */
    getMeta(): Promise<IFieldMeta>;
    /** 获取整列不为空的 cellValue */
    getFieldValueList(): Promise<(IFieldValue | IUndefinedFieldValue)[]>;
}

```
