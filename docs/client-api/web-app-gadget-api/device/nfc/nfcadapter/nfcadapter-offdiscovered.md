---
document_id: '6971043590354157573'
directory_id: '6907567266540699650'
title: NFCAdapter.offDiscovered
full_path: /uYjL24iN/uQDN4YjL0QDO24CN0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFCAdapter
- NFCAdapter.offDiscovered
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:10Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQDN4YjL0QDO24CN0gjN
---

# NFCAdapter.offDiscovered(function callback)

取消监听 NFC Tag

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.38.0+</md-version> | **X** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
无


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
var adapter = tt.getNFCAdapter();
var onDiscoveredCallback = (res) => {
   console.log(JSON.stringify(res));
};
adapter.onDiscovered(onDiscoveredCallback);
adapter.offDiscovered(onDiscoveredCallback);
```

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
