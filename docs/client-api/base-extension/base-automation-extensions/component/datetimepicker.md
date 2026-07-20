---
document_id: '7260082411118985221'
directory_id: '7258197168736583686'
title: DateTimePicker
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/datetimepicker
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- Component
- DateTimePicker
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/datetimepicker
---

# DateTimePicker

选择时间的组件，可引用多维表格中的时间字段，无需传入参数，`componentProps` 无需传入参数。<br>
  
`DateTimePicker`组件传递给`execute`函数入参的数据结构。
| 类型     | 说明       |
| ------ | -------- |
| number | 用户选择的时间戳 |

# 示例代码

## 调用示例

```js
import { basekit, Component } from '@lark-opdev/block-basekit-server-api';
basekit.addAction({
    formItems: [
      {
        itemId: 'date',
        label: '日期',
        component: Component.DateTimePicker,
      },
    ],
    execute: async function(args, context) {
      const { date } = args;
    },
    // ...
});
// ..
```
