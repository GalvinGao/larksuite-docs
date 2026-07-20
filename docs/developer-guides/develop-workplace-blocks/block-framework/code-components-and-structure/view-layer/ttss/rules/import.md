---
document_id: '7180270043522727942'
directory_id: '7179507279662006278'
title: '@import'
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/rules/import
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- '@ rules'
- '@import'
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/rules/import
---

# @import

`@import`规则，用于从其他样式表导入样式规则。这些规则必须先于所有其他类型的规则，它不是一个嵌套语句，`@import`不能在条件组的规则中使用。

## 语法

```css
@import <string>;
```

其中，`<string>`表示[字符串](/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/string)数据类型。

## 示例

```css
@import 'custom.ttss';
@import './custom.ttss';
@import '/pages/vard/index.ttss';
```
