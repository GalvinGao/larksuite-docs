---
document_id: '6965379541104033797'
directory_id: '6907567266536308737'
title: getStorageInfoSync
full_path: /uYjL24iN/ugTOx4CO5EjL4kTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- getStorageInfoSync
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:45Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugTOx4CO5EjL4kTM
---

# getStorageInfoSync()

获取本地缓存数据的相关信息。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/storage/storage" fontSize="14">预览</md-preview-app> |
| 网页应用 | **x** | **x** | **x** | / |



## 输入
无

## 输出

返回值：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| keys | string[] | 本地数据缓存中的所有键名列表，如果没有本地数据则返回空数组 |
| currentSize | number | 占用空间大小，以 `KB` 为单位 |
| limitSize | number | 存储空间上限，以 `KB` 为单位，一般来说会返回 `10240` |


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
    let result = tt.getStorageInfoSync();
    console.log(`getStorageInfoSync success: ${JSON.stringify(result)}`);
} catch (error) {
    console.log(`getStorageInfoSync fail: ${JSON.stringify(error)}`);
}
```

返回值示例：
```json
{
    "currentSize": 0.0419921875,
    "keys": [
        "name"
    ],
    "limitSize": 10240
}
```
