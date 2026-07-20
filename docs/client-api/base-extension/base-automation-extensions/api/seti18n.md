---
document_id: '7260082411118772229'
directory_id: '7258197168736616454'
title: setI18n
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/api/seti18n
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- API
- setI18n
document_type: GuideDocumentType
updated_at: 2023-07-27T02:36:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/api/seti18n
---

# setI18n
国际化文案。需要结合t函数使用。



## 输入
```js
setI18n({
  defaultLocale: 'en-US',
  messages: {
      'zh-CN': zhCN,
      'en-US': enUS,
  },
})
```

:::html
<md-table>
  <colgroup>
    <col style="width: auto">
    <col style="width: auto">
    <col style="width: auto">
    <col style="width: auto">
  </colgroup>
	<md-thead> 
      <md-tr>
      	<md-th>名称</md-th>
        <md-th>数据类型</md-th>
        <md-th>是否必填</md-th>
        <md-th>描述</md-th>
      </md-tr>
  </md-thead> 
  	<md-tbody>
      <md-tr>
      	<md-td>defaultLocale</md-td>
        <md-td>string</md-td>
        <md-td>否</md-td>
        <md-td>默认语言环境，格式为lng-REGION如zh-CN、en-US等。值必须为message中的某个key。</md-td>
      </md-tr>
       <md-tr>
      	<md-td>messages</md-td>
        <md-td>
{<br>
&nbsp;&nbsp;[lang:string]: {<br>
&nbsp;&nbsp;&nbsp;&nbsp;[key:string]: string,<br>
&nbsp;&nbsp;},<br>
}
        </md-td>
        <md-td>否</md-td>
        <md-td>本地化的语言环境信息，lang的格式为lng-REGION，例如zh-CN、en-US。</md-td>
      </md-tr>
    </md-tbody>
</md-table>
:::



## 示例代码
```js
import { basekit, t } from '@lark-opdev/block-basekit-server-api';

const zhCN = {
    source_text: "源文本",
    result: "转换结果",
};
const enUS = {
    source_text: "Source Text",
    result: "Conversion Result"
};

basekit.setI18n({
  defaultLocale: 'en-US',
  messages: {
      'zh-CN': zhCN,
      'en-US': enUS,
  },
});
basekit.addAction({
    formItems: [
        {
            label: t('source_text'),
            itemId: 'text',
            required: true,
            component: Component.Input,
            componentProps: {
              mode: 'textarea',
              placeholder: '请输入源文本或选择引用列',
            }
        },
    ],
    resultType: {
        type: ParamType.Object,
        properties: {
            text: {
                type: ParamType.String,
                label: t('result')，
            }，
        }，
    }，
});
export default basekit;
