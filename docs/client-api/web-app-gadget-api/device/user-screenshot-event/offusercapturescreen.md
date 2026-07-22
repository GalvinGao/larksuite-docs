---
document_id: '6965379541105065989'
directory_id: '6907567266536882177'
title: offUserCaptureScreen
full_path: /uYjL24iN/uQjNwEjL0YDMx4CN2ATM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- User Screenshot Event
- offUserCaptureScreen
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:10Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQjNwEjL0YDMx4CN2ATM
---

# offUserCaptureScreen(function callback)

取消监听用户主动截屏事件。

:::html
<md-alert type="tip">
建议配合[onUserCaptureScreen](/document/uYjL24iN/uMjNwEjLzYDMx4yM2ATM)接口使用。
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.4.0+</md-version> | <md-version>V2.4.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/on-user-capture-screen/on-user-capture-screen" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 用户主动截屏事件的回调函数 |


## 输出
无


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/on-user-capture-screen/on-user-capture-screen" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.offUserCaptureScreen(function(res) {
    console.log(JSON.stringify(res));
});
```
