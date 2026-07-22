---
document_id: '6965379541104852997'
directory_id: '6907567266541371394'
title: stopCompass
full_path: /uYjL24iN/uMzNx4yM3EjLzcTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Compass
- stopCompass
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:34Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMzNx4yM3EjLzcTM
---

# stopCompass(Object object)

停止监听罗盘数据。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-compass-change/on-compass-change" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-compass-change/on-compass-change" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.stopCompass({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`stopCompass fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "stopCompass:ok"
}
```
