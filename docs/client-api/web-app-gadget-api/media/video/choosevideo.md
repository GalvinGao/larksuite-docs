---
document_id: '6965379543683710982'
directory_id: '6907567266536357889'
title: chooseVideo
full_path: /uYjL24iN/uEjMx4SMyEjLxITM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- chooseVideo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:27Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMx4SMyEjLxITM
---

# chooseVideo(Object object)

从系统相册中选择视频，或使用相机拍摄视频。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/video/video" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44+</md-version> | <md-version>V3.44+</md-version> | <md-version>V3.47+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| sourceType | string[] | 否 | ['album', 'camera'] | 指定视频来源为相册或/和相机<br>**示例值**：['album']<br>**可选值有**：<br>- `['album']`<br>- `['camera']`<br>- `['album','camera']`<br><md-alert type="tip" icon="none"><br>PC暂不支持camera<br></md-alert> |
| maxDuration | number | 否 | 60 | 选取视频最长时间，单位秒。<br>**示例值**：80<br>**最大值**：`180`<br><md-alert type="tip" icon="none"><br>- PC 端：不限制最大时长<br>- iOS<br>- 当 `compressd` 为 `true` 时，`maxDuration` 默认值为 60s，最大支持选取 180s 视频<br>- 当 `compressd` 为 `false` 时，`maxDuration` 默认值为 60s，不限制最大时长<br>- Android<br>- `maxDuration` 默认值为 60s，最大支持选取 180s 视频<br></md-alert> |
| compressed | boolean | 否 | true | 是否对选取视频进行压缩<br>**示例值**：true<br><md-alert type="tip" icon="none"><br>- iOS 端：Lark[V3.37](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持,默认为不压缩，iOS 端设置为 `false` 时，不进行压缩转码，但是相册导出可能仍然需要花费一定时间<br>- Android/PC 端：暂不支持（不进行压缩）<br></md-alert> |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| duration | number | 视频时长，单位：秒/s。<br><md-alert type="tip" icon="none"><br>PC 端：暂不支持<br></md-alert> |
| tempFilePath | string | 视频地址 |
| size | number | 视频大小，单位：字节/Bytes |
| width | number | 视频宽度<br><md-alert type="tip" icon="none"><br>PC 端：暂不支持<br></md-alert> |
| height | number | 视频高度<br><md-alert type="tip" icon="none"><br>PC 端：暂不支持<br></md-alert> |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/video/video" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseVideo({
    sourceType: [
        "album"
    ],
    maxDuration: 80,
    compressed: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`chooseVideo fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "duration": 8,
    "height": 600,
    "size": 594655,
    "tempFilePath": "ttfile://temp/1637482384392.mp4",
    "width": 600,
    "errMsg": "chooseVideo:ok"
}
```


