---
document_id: '7260082411118395397'
directory_id: '7258197168736583686'
title: MultipleSelect
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/multipleselect
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- Component
- MultipleSelect
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/multipleselect
---

# MultipleSelect

下拉多选组件，用户手动选择下拉项里的值，`componentProps` 支持以下参数。
| 参数          | 类型                  | 是否必填 | 说明                              |
| ----------- | ------------------- | ---- | ------------------------------- |
| placeholder | string              | 否    | 输入框提示文字                         |
| options     | { label, value }[] | 是    | 选项数据，其中`label`为展示文案，`value`为实际值 |
<br><br><br>
`MultipleSelect`组件传递给`execute`函数入参的数据结构。
| 类型                | 说明                    |
| ----------------- | --------------------- |
| string|number[] | 选中的 options 的 value 值 |

# 示例代码

## 调用示例

```js
import { basekit, Component } from '@lark-opdev/block-basekit-server-api';
basekit.addAction({
    formItems: [
      {
        itemId: 'provinces',
        label: '单选',
        component: Component.MultipleSelect,
        componentProps: {
          options: [
              {
                label: '北京',
                value: 'bj',
              },
              {
                label: '上海',
                value: 'sh'
              }
            ]
        }
      },
    ],
    execute: async function(args, context) {
      const { provinces = [] } = args;
    },
    // ...
});
// ..
```
