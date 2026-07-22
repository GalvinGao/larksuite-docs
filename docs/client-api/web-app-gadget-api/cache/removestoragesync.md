---
document_id: '6965379541104607237'
directory_id: '6907567266536308737'
title: removeStorageSync
full_path: /uYjL24iN/uQTOx4CN5EjL0kTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- removeStorageSync
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:36Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOx4CN5EjL0kTM
---

# removeStorageSync(string key)

删除本地缓存数据。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="14">预览</md-preview-app> |
| 网页应用 | **x** | **x** | **x** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| key | string | 是 |  | 键名<br>**示例值**：name<br>**最小长度**：`1`  字符 |



## 输出
无

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
try {
    tt.removeStorageSync("name");
} catch (error) {
    console.log(`removeStorageSync fail: ${JSON.stringify(error)}`);
}
```
