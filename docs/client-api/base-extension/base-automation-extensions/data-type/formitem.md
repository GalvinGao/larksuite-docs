---
document_id: '7260082411118215173'
directory_id: '7258197168736600070'
title: FormItem
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/data-type/formitem
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- Data Type
- FormItem
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/data-type/formitem
---

# FormItem
定义自动化表单的数据结构。
参数             | 类型                                                                        | 是否必填 | 说明                                                                                                                                              |
| -------------- | ------------------------------------------------------------------------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| itemId         | string                                                                    | 是    | 表单 id，运行时会作为参数传给 `execute` 函数，其中`stepId`、`tableId`、`blockId`、`fieldId`和`fieldIds`为保留词，请勿用做表单id。                                                 |
| label          | string                                                                    | 是    | 表单文案。                                                                                                                                            |
| component      | Component | 是    | 表单组件，如[Input](/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/input) 、[SingleSelect](/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/component/singleselect)。                                                     |
| componentProps | object                                                                    | 否    | 表单组件的属性。                                                                                                                                         |
| required       | boolean                                                                   | 否    | 该表单项是否必填，默认为 `false`。                                                                                                                            |
| tooltip        | string                                                                    | 否    | 提示信息，展示在label右侧的图标处，需要hover展示。                                                                                                                   |
| help           | string                                                                    | 否    | 帮助信息，展示在组件下方。                                                                                                                                    |
| displayBy      | string                                                                    | 否    | 控制组件是否显示，语法为由`key:value`组成的字符串，支持`&&` `||` `!` 和`  () ` 运算符，当displayBy表达式为`true`时显示。其中`key`为`SingleSelect`、`MultipleSelect`组件的`itemId`，`value`值为用户使用该组件时选择的值。MultipleSelect支持多个值，不区分顺序，值与值之间使用,间隔开即可。


## 示例代码

### displayBy控制组件显示/隐藏
```js
import { basekit, Component } from '@lark-opdev/block-basekit-server-api';

basekit.addAction({
  formItems: [
    {
      itemId: 'transportation',
      label: '交通工具',
      component: Component.SingleSelect,
      componentProps: {
        options: [
          {
            label: '飞机',
            value: 'plane',
          },
          {
            label: '火车',
            value: 'train',
          },
        ],
      },
    },
    {
      itemId: 'flightNumber',
      label: '航班号',
      component: Component.Input,
      // 当交通工具为飞机时才展示该组件
      displayBy: 'transportation:plane',
    },
  ],
  // ...
});
// ...
```

