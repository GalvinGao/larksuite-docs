---
document_id: '7260081693314646022'
directory_id: '7258197168736583686'
title: Radio
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/radio
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- Component
- Radio
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/radio
---

# Radio
单选框组件，`componentProps` 支持以下参数。
| 参数           | 类型                  | 是否必填 | 说明                              |
| ------------ | ------------------- | ---- | ------------------------------- |
| placeholder  | string              | 否    | 提示文字。                            |
| defaultValue | string              | 否    | 默认值。                             |
| options      | { label, value }[] | 是    | 选项数据，其中`label`为展示文案，`value`为实际值。 |
<br><br>
`Radio`组件传递给`execute`函数入参的数据结构。
| 类型     | 说明                   |
| ------ | -------------------- |
 | string | 选中的 option 的 value 值。 |

# 示例代码

## 调用示例

```js
import { basekit, Component } from '@lark-opdev/block-basekit-server-api';
basekit.addAction({
    formItems: [
      {
        itemId: 'radio',
        label: 'Radio',
        component: Component.Radio,
        componentProps: {
            options: [
                {
                    label: 'A',
                    value: 'A',
                },
                {
                    label: 'B',
                    value: 'B',
                }
            ]
        }
      },
    ],
    execute: async function(args, context) {
      const { radio } = args;
    },
    // ...
});
// ..
```
