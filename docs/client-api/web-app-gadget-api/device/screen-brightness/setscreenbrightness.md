---
document_id: '6965379543683579910'
directory_id: '6907567266540535810'
title: setScreenBrightness
full_path: /uYjL24iN/uIjNx4iM2EjLyYTM/set-screen-brightness
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Screen Brightness
- setScreenBrightness
document_type: GuideDocumentType
updated_at: 2023-09-11T08:57:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIjNx4iM2EjLyYTM/set-screen-brightness
---

# setScreenBrightness(Object object)

设置屏幕亮度。

::: note
iOS离开小程序或网页应用后设置依然生效。Android仅在当前小程序或网页应用页面生效，离开小程序或网页应用后设置失效。
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.42.0+</md-version> | <md-version>V3.42.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/screen-brightness/screen-brightness" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" disable="true" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| value | number | 是 |  | 屏幕亮度值，0 最暗，1 最亮<br>**示例值**：`0.8`<br>**最小值**：`0`<br>**最大值**：`1` |

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/screen-brightness/screen-brightness" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.setScreenBrightness({
    "value": 0.8,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setScreenBrightness fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "setScreenBrightness:ok"
}
```
