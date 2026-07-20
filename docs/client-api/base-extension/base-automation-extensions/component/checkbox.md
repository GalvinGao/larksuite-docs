---
document_id: '7260082411118297093'
directory_id: '7258197168736583686'
title: Checkbox
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/checkbox
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- Component
- Checkbox
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/checkbox
---

# Checkbox

复选框组件，`componentProps` 无需传入参数。<br><br>

`Checkbox`组件传递给`execute`函数入参的数据结构。
| 类型      | 说明                  |
| ------- | ------------------- |
| boolean | 用户选中时为true，否则为false |

# 示例代码

## 调用示例

```js
import { basekit, Component } from '@lark-opdev/block-basekit-server-api';
basekit.addAction({
    formItems: [
      {
        itemId: 'checkbox',
        label: '是否启用',
        component: Component.Checkbox,
      },
    ],
    execute: async function(args, context) {
      const { checkbox } = args;
    },
    // ...
});
// ..
```
