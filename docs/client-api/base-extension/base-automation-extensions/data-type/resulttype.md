---
document_id: '7260081693314498566'
directory_id: '7258197168736600070'
title: ResultType
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/data-type/resulttype
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- Data Type
- ResultType
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:48Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/data-type/resulttype
---

# ResultType
自动化插件返回类型的数据类型。
| 字段         | 类型     | 是否必填 | 说明                                                                                                                                              |
| ---------- | ------ | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| type       | string | 是    | 说明该字段的类型，支持 `Object`, `String`, `Number`, `Boolean`, `Array`。                                                                                    |
| properties | object | 否    | 对象的属性，其中的 key 需要与 `execute` 中返回值的 key 一致。                                                                                                        |
| label      | string | 是    | 显示在自动化中的文案。                                                                                                                                      |
| displayBy  | string | 否    | 控制组件是否显示，语法为由`key:value`组成的字符串，支持`&&` `||` `!` 和`  () ` 运算符，当表达式为`true`时显示。其中`key`为`SingleSelect`、`MultipleSelect`组件的`itemId`，`value`值为组件选择的值。 |
其中不同 `type` 字段支持回写的字段类型如下：
| 类型        | 支持回写的字段类型               |
| --------- | ----------------------- |
| `String`  | 多行文本、多选、单选、超链接的文本。       |
| `Number`  | 多行文本、多选、单选、数字、超链接的文本、进度。 |
| `Boolean` | 复选框。                     |
| `Array`   | 附件、多选。                   |

# 示例代码

## 返回附件

```js
import { basekit, Component, uploadAttachments, ParamType, StructureType } from '@lark-opdev/block-basekit-server-api';
import fs from 'fs';
import path from 'path';
basekit.addAction({
  execute: async function (args, context) {
    const name = 'demo.png';
    const file = await fs.readFileSync(path.join(__dirname, name));
    const attachments = await uploadAttachments([
      {
        name,
        file,
      }
    ], {
      context,
      env: 'Lark',
    });
    return {
      attachments,
    };
  },
  resultType: {
    type: ParamType.Object,
    properties: {
      attachments: {
        type: ParamType.Array,
        structureType: StructureType.Attachment,
        label: '附件',
      }
    },
  },
  // ...
});
// ...
```
