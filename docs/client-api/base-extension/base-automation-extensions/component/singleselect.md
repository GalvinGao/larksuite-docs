---
document_id: '7260082411118100485'
directory_id: '7258197168736583686'
title: SingleSelect
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/singleselect
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- Component
- SingleSelect
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/singleselect
---

# SingleSelect

下拉单选组件，用户手动选择下拉项里的值，`componentProps` 支持以下参数。
| 参数          | 类型                  | 说明                              |
| ----------- | ------------------- | ------------------------------- |
| placeholder | string              | 输入框提示文字                         |
| options     | { label, value }[] | 选项数据，其中`label`为展示文案，`value`为实际值 |

<br><br><br>
`SingleSelect`组件传递给`execute`函数入参的数据结构。
| 类型             | 说明                   |
| -------------- | -------------------- |
| string|number | 选中的 option 的 value 值 |

# 示例代码

## 调用示例

```js
import { basekit, Component } from '@lark-opdev/block-basekit-server-api';
basekit.addAction({
    formItems: [
      {
        itemId: 'province',
        label: '单选',
        component: Component.SingleSelect,
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
      const { province = '' } = args;
    },
    // ...
});
// ..
```
