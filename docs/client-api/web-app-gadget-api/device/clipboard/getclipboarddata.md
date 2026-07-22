---
document_id: '6965379543684169734'
directory_id: '6907567266536734721'
title: getClipboardData
full_path: /uYjL24iN/uczNx4yN3EjL3cTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Clipboard
- getClipboardData
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/uczNx4yN3EjL3cTM
---

# getClipboardData(Object object)

获取系统粘贴板数据。

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

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | string | 粘贴板数据 |


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
tt.getClipboardData({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getClipboardData fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "getClipboardData:ok",
    "data": "hello world"
}
```

