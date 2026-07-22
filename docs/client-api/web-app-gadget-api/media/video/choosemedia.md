---
document_id: '7073693024735428613'
directory_id: '6907567266536357889'
title: chooseMedia
full_path: /uYjL24iN/uITMx4iMxEjLyETM/choosemedia
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- chooseMedia
document_type: GuideDocumentType
updated_at: 2023-03-03T03:15:27Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITMx4iMxEjLyETM/choosemedia
---

# chooseMedia(Object object)

拍摄或从系统相册中选择图片或视频。




## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.7.0+</md-version> | <md-version>V4.7.0+</md-version> | <md-version>V4.7.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/choose-media/choose-media" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V4.7.0+</md-version> | <md-version>V4.7.0+</md-version> | <md-version>V4.7.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| mediaType | string[] | 否 | ["image", "video"] | 文件类型，图片或/和视频<br>**可选值**：<br>- `["image"]`：图片<br>- `["video"]`：视频<br>- `["video", "image"]`：视频或图片，不分顺序 |
| sourceType | string[] | 否 | ["album", "camera"] | 指定视频来源为相册或/和相机<br>**可选值**：<br>- `["album"]`：相册<br>- `["camera"]`：相机<br>- `["album", "camera"]`：相册或相机，不分顺序<br><md-alert type="tip" icon="none"><br>- PC 端：暂不支持，默认为本地文件系统<br></md-alert> |
| count | number | 否 | 9 | 最多可以选择的文件数量，可支持选择多个图片或多个视频。Lark V5.30前，最多可支持9个文件，V5.30及以后最多可支持20个文件，使用相机拍照拍视频时该字段失效 |
| sizeType | string[] | 否 | ["original","compressed"] | 表示是选择原图或原视频，还是对图片或视频质量进行压缩<br>**可选值**：<br>- `["original"]`：选择原图<br>- `["compressed"]`：强制对图片或视频质量进行压缩<br>- `["original","compressed"]`：默认开启压缩，但可手动选择原图或原视频<br><md-alert type="tip" icon="none"><br>- Android/PC 端：不支持视频压缩<br>- PC 端：不支持 `["original","compressed"]`，默认为`["original"]`<br></md-alert> |
| maxDuration | number | 否 | 60 | 拍摄视频最长拍摄时间，单位秒。时间范围为 3s 至 60s 之间。不限制相册。<br>**示例值**：30<br><md-alert type="tip" icon="none"><br>- PC 端：暂不支持此参数，不限制最大时长<br></md-alert> |
| cameraDevice | string | 否 | back | 使用相机拍摄的默认摄像头<br><md-alert type="tip" icon="none"><br>- PC 端：暂不支持<br></md-alert><br>**可选值**：<br>- `front`：前置摄像头<br>- `back`：后置摄像头 |
| isSaveToAlbum | string | 否 | 0 | 使用相机拍摄后图片是否保存到相册，仅iOS和Android支持且在sourceType为camera时生效<br>**可选值**：<br>- `"0"`：不保存<br>- `"1"`：保存<br><md-alert type="tip" icon="none"><br>- iOS 端：Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>-  Android 端：Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：暂不支持<br></md-alert> |


## 输出


`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| tempFiles | object[] | 临时文件数组 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>tempFilePath<br></md-text> | string | 文件地址 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>type<br></md-text> | string | 文件类型，有效值有image、video，分别对应图片和视频 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>size<br></md-text> | number | 视频大小，单位：字节/Bytes |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>duration<br></md-text> | number | 视频时长，单位：秒/s。选取文件类型为video时返回该字段<br><md-alert type="tip" icon="none"><br>- PC 端：暂不支持<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>width<br></md-text> | number | 视频宽度。选取文件类型为video时时返回该字段<br><md-alert type="tip" icon="none"><br>- PC 端：暂不支持<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>height<br></md-text> | number | 视频高度。选取文件类型为video时返回该字段<br><md-alert type="tip" icon="none"><br>- PC 端：暂不支持<br></md-alert> |



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/choose-media/choose-media" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
    <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseMedia({
    mediaType: [
        "image",
        "video"
    ],
    sourceType: [
        "album"
    ],
    count: 1,
    sizeType: [
        "compressed",
        "original"
    ],
    maxDuration: 60,
    cameraDevice: "back",
    isSaveToAlbum: '0',
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`chooseMedia fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "chooseMedia:ok",
    "tempFiles": [
        {
            "tempFilePath": "ttfile://temp/6abe1c2d-2b78-476f-ad53-f7d4db3b51d3-11bb40a45f41c6cc4ff343c3ddf97c93.jpg",
            "size": 51014,
            "type": "image"
        }
    ]
}
```
