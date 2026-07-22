---
document_id: '6965379541104525317'
directory_id: '6907567266541535234'
title: vibrateShort
full_path: /uYjL24iN/uADOx4CM4EjLwgTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Vibrate
- vibrateShort
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:25Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADOx4CM4EjLwgTM
---

# vibrateShort(Object object)

使手机发生较短时间的振动。

:::html
<md-alert type="tip">
Android 某些机型在不支持短振动，可以使用[vibrateLong](/document/uYjL24iN/uEDOx4SM4EjLxgTM)代替。
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/vibrate/vibrate" fontSize="14">预览</md-preview-app> |
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/vibrate/vibrate" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.vibrateShort({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`vibrateShort fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "vibrateShort:ok"
}
```
