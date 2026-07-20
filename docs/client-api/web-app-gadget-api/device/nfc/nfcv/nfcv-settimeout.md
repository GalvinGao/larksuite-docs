---
document_id: '7143913324618022917'
directory_id: '7124599404677513222'
title: NfcV.setTimeout
full_path: /uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/settimeout
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NfcV
- NfcV.setTimeout
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:56Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/settimeout
---

# NfcV.setTimeout(Object object)

设置超时时间


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
      <md-td><md-version>V5.14.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" disable="true" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.14.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app></md-td>
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
                timeout
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                超时时长（ms）
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::
## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
let adapter = tt.getNFCAdapter();
adapter.onDiscovered(
      (res) => {
           console.log('onDiscovered res, ' + JSON.stringify(res));
           let nfcV = adapter.getNfcV();
           nfcV.setTimeout({
              'timeout': 1000,
               success(res) {
               	console.log(JSON.stringify(res))
               },
               fail(err) {
               	console.log(`NfcV.setTimeout fail: ${JSON.stringify(res)}`)
               }
           });
      }
);
```
`success`返回对象示例：
```json
{"errMsg":"nfcSetTimeout:ok"}
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
