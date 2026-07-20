---
document_id: '6965379541104410629'
directory_id: '6907567266541240322'
title: showPrompt
full_path: /uYjL24iN/uYTO4UjL2kDO14iN5gTN
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Interaction Feedback
- showPrompt
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:08Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYTO4UjL2kDO14iN5gTN
---

# showPrompt(Object object)

展示可输入内容的弹窗。


## 支持说明
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td><md-version>V3.17.0+</md-version></md-td>
      <md-td><md-version>V3.17.0+</md-version></md-td>
      <md-td><md-version>V3.17.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/prompt/prompt" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入


继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                title
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                标题，中文按照2个字符统计

**最大长度**：`30`  字符   

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                placeholder
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                Please enter your content here
            </md-td>
            <md-td>
                输入框内容为空时的提示文案
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                maxLength
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                140
            </md-td>
            <md-td>
                最大输入长度，设置为-1的时候不限制最大长度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                confirmText
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                OK
            </md-td>
            <md-td>
                确定按钮的文案，中文按照2个字符统计


**最大长度**：`8`  字符
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                cancelText
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                Cancel
            </md-td>
            <md-td>
                取消按钮的文案，中文按照2个字符统计


**最大长度**：`8`  字符
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

`success`返回对象的扩展属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                confirm
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                是否点击了确定按钮
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                cancel
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                是否点击了取消按钮
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                inputValue
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                confirm为true时，用户输入的内容
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/prompt/prompt" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.showPrompt({
    "title": "这是个输入弹窗",
    "placeholder": "在这里输入内容",
    "maxLength": 50,
    "confirmText": "确定",
    "cancelText": "取消",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showPrompt fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "errMsg": "showPrompt:ok",
    "confirm": true,
    "cancel": false,
    "inputValue": "示例内容"
}
``` 
