---
document_id: '7260081693314744326'
directory_id: '7258197168736616454'
title: t
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/api/t
breadcrumb:
- Client API
- Base Extension
- Base Automation Extensions
- API
- t
document_type: GuideDocumentType
updated_at: 2023-07-26T11:07:51Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/api/t
---

# t
翻译函数。用于设置多语，详情请参考[多维表格自动化插件开发指南](/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/base-automation-extension-development-guide)中的国际化。<br>本函数也可以设置表单的帮助、提示文案。




## 输入
```
t(key, param)
```

:::html
<md-table>
  <colgroup>
    <col style="width: 100px">
    <col style="width: auto">
    <col style="width: 80px">
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
      	<md-td>key</md-td>
        <md-td>string</md-td>
        <md-td>是</md-td>
        <md-td>国际化资源中的某种语言的key，详情参考[多维表格自动化插件开发指南](/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/base-automation-extension-development-guide)。</md-td>
      </md-tr>
       <md-tr>
      	<md-td>param</md-td>
        <md-td>
          ```js
{
    [key: string]: TLink | TIcon | THighlight;
}
         </md-td>
        <md-td>否</md-td>
        <md-td>控制文字的链接，图标，高亮。
         只能用于basekit.addAction入参的description字段、basekit.addAction字段的formItems字段的help，tooltip属性。
         </md-td>
      </md-tr>
    </md-tbody> 
</md-table>
:::
```js
type TLink = {
    type: 'link';
    value: string;
    link: string;
};
type TIcon = {
    type: 'icon';
    value?: string;
    /** URL that the hyperlink points to */
    link?: string;
    /** URL of the icon  */
    src: string;
};
type THighlight = {
    type: 'highlight';
    value: string;
    color: string;
};
```


## 输出
国际化的字符串或对象，详情请参考[多维表格自动化插件开发指南](/document/uAjLw4CM/uYjL24iN/base-extensions/base-automation-extensions/base-automation-extension-development-guide)中的国际化。

## 示例代码
```js
import { basekit, Component, ParamType, getBasekitUrl, t } from '@lark-opdev/block-basekit-server-api';

basekit.setI18n({
  messages: {
    'zh-CN': {
      description: "查看{doc}",
      usage: '说明文档',
      help: '这是图标{icon}',
      tooltip_111: '这是{highlight}',
    },
  }
});


basekit.addAction({
  description: t('description', {
    // 将文案中 doc 变量替换成可跳转链接
    doc: {
      type: 'link',
      value: t('usage'),
      link: 'https://your.doc.com',
    },
  }),
  formItems: [
    {
      itemId: 'text',
      label: '测试',
      help: t('help', {
        // 将文案中 icon 变量替换成可跳转的图标
        icon: {
          type: 'icon',
          src: 'https://your.img.com',
          link: 'https://your.link.com'
        }
      }),
      tooltip: t('tooltip_111', {
        // 将文案中的 highlight 变量替换成高亮格式文案
        highlight: {
          type: 'highlight',
          value: '高亮文案',
          color: 'red',
        }
      }),
      required: true,
      component: Component.Input,
      componentProps: {
        mode: 'textarea',
        placeholder: '测试',
      }
    },
    //....
```
