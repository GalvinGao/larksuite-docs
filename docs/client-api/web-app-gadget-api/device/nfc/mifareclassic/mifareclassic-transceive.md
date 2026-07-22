---
document_id: '6971043590354141189'
directory_id: '6907567266540470274'
title: MifareClassic.transceive
full_path: /uYjL24iN/ucTN4YjL3UDO24yN1gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- MifareClassic
- MifareClassic.transceive
document_type: GuideDocumentType
updated_at: 2024-03-07T08:42:04Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucTN4YjL3UDO24yN1gjN
---

# MifareClassic.transceive(Object object)

发送数据给MifareClassic类型的标签


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | **X** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| data | arraybuffer | 否 |  | 需要传递的二进制数据 |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | arraybuffer | 返回的二进制数据 |



## 示例代码

```js
let adapter = tt.getNFCAdapter();
adapter.onDiscovered(
      (res) => {
           console.log('onDiscovered res, ' + JSON.stringify(res));
           let nfc = adapter.getMifareClassic();
        	//传递数据
           nfc.transceive({
              data: new Uint8Array([0x30, 0x03]).buffer,
              success(res) {
                  console.log('MifareClassic.transceive success res=', res, Array.from(new Uint8Array(res.data)));
              },
              fail(res) {
                  console.log(`MifareClassic.transceive fail: ${JSON.stringify(res)}`)
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
