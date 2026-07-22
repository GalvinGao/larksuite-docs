---
document_id: '6965379543683661830'
directory_id: '6907567266541436930'
title: scanCode
full_path: /uYjL24iN/uYzNx4iN3EjL2cTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Scan Code
- scanCode
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:40Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYzNx4iN3EjL2cTM
---

# scanCode(Object object)

扫描二维码并返回扫描结果。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/scan-code/scan-code" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| scanType | string[] | 否 | ['qrCode','barCode'] | 扫码类型。传入多个类型，代表可以支持多种类型的扫码。扫码类型定义如下：<br>- `qrCode`：二维码<br>- `barCode`：条形码<br>- `datamatrix`：Data Matrix 码<br>- 仅iOS支持<br>- `pdf417`：PDF417 条码<br>- 仅iOS支持 |
| barCodeInput | boolean | 否 | false | 是否支持手动输入条形码<br><md-alert type="tip" icon="none"><br>Lark[V3.14.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| result | string | 扫描结果 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/scan-code/scan-code" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.scanCode({
    scanType: [
        "barCode",
        "qrCode"
    ],
    barCodeInput: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`scanCode fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "result": "sslocal://microapp?app_id=cli_xxx&version_type=preview&token=xxx&isdev=1&start_page=pages%2FscanCode%2Findex",
    "errMsg": "scanCode:ok"
}
```

## 已知问题

- 对于包含 `GBK ` 编码内容的二维码扫描时会有乱码。
