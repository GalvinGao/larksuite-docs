---
document_id: '7180270043522023430'
directory_id: '7179507279661973510'
title: number
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/number
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Basic Data Type
- number
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/basic-data-type/number
---

# number

## 介绍

`<number>`数据类型代表一个数字，可为整数或小数，它没有任何单位，并不是一个`TTSS`尺寸。

## 实例

### 有效数字

```
12          正整数
4.01        正小数
-456.8      负小数
0.0         零
+0.0        带正号的零
-0.0        带符号的零
.60         点前的数字可以省略
10e3        科学计数法
-3.4e-2     科学计数法最复杂的情况
```

### 非法数字

```
12.         点后需有数字
+-12.2      只能有一个符号
12.1.1      只能有一个点
```
