---
document_id: '7143913324618072069'
directory_id: '7124599404677513222'
title: NfcV.transceive
full_path: /uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/transceive
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NfcV
- NfcV.transceive
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:45Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/NfcV/transceive
---

# NfcV.transceive(Object object)

发送数据给NFCV类型的标签


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.14.0+</md-version> | **X** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.14.0+</md-version> | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| data | arraybuffer |  |  | 需要传递的二进制数据 |


## 输出
`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | arraybuffer | 返回的二进制数据 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
let adapter = tt.getNFCAdapter();
adapter.onDiscovered(
      (res) => {
           console.log('onDiscovered res, ' + JSON.stringify(res));
           let nfcV = adapter.getNfcV();
        	//传递数据
           nfcV.transceive({
              data: new Uint8Array([0x22, 0x20, ...this.uid, 1).buffer,
              success(res) {
                  console.log('NfcV.transceive success res=', res, Array.from(new Uint8Array(res.data)));
              },
              fail(res) {
                  console.log(`NfcV.transceive fail: ${JSON.stringify(res)}`)
              }
           });
      }
);
```
`success`返回对象示例：
```json
{"errMsg":"nfcTransceive:ok","data":{}}
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
