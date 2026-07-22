---
document_id: '6965379543683989510'
directory_id: '6907567266536734721'
title: setClipboardData
full_path: /uYjL24iN/ugzNx4CO3EjL4cTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Clipboard
- setClipboardData
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:43Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugzNx4CO3EjL4cTM
---

# setClipboardData(Object object)

设置系统粘贴板数据
:::html
<md-alert type="tip">
从**3.36版本**开始后, 调用前需要用户授权 `scope.clipboard`。了解如何授权，可查看[API 权限](/document/uYjL24iN/uITMuITMuITM)。
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-clipboard-data/get-clipboard-data" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| data | string | 是 |  | 设置系统粘贴板数据<br>**示例值**："hello world" |


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-clipboard-data/get-clipboard-data" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.setClipboardData({
    data: "hello world",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setClipboardData fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "setClipboardData:ok"
}
```
