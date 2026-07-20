---
document_id: '6971043590353993733'
directory_id: '6907567266536505345'
title: NfcA.getMaxTransceiveLength
full_path: /uYjL24iN/ukDN4YjL5QDO24SO0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NfcA
- NfcA.getMaxTransceiveLength
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:29Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDN4YjL5QDO24SO0gjN
---

# NfcA.getMaxTransceiveLength(Object object)

获取最大传输长度


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
      <md-td><md-version>V3.38.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

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
                length
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                最大传输长度
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16" disable="true">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
let adapter = tt.getNFCAdapter();
adapter.onDiscovered(
      (res) => {
           console.log('onDiscovered res, ' + JSON.stringify(res));
           let nfcA = adapter.getNfcA();
           nfcA.getMaxTransceiveLength({
               success(res) {
               	console.log(JSON.stringify(res))
               },
               fail(err) {
               	console.log(`NfcA.getMaxTransceiveLength fail: ${JSON.stringify(res)}`)
               }
           });
      }
);
```
`success`返回对象示例：
```json
{"length":253,"errMsg":"nfcMaxTransceiveLength:ok"}
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
