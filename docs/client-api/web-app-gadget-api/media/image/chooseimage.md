---
document_id: '6965379543684235270'
directory_id: '6907567266541977602'
title: chooseImage
full_path: /uYjL24iN/uMTMx4yMxEjLzETM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Image
- chooseImage
document_type: GuideDocumentType
updated_at: 2022-05-31T03:08:21Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMx4yMxEjLzETM
---

# chooseImage(Object object)


从系统相册中选择图片，或使用相机拍摄图片。




## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.47.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入


继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| sourceType | string[] | 否 | ["album", "camera"] | 指定图片来源为相册或/和相机<br>**可选值**：<br>- `["album"]`：相册<br>- `["camera"]`：相机<br>- `["album", "camera"]`: 相册或相机，<br><md-alert type="tip" icon="none"><br>- PC 端：暂不支持，默认为本地文件系统<br>- iOS 端：不支持同时从 album 和 camera 中选择，只能二者选其一<br></md-alert> |
| count | number | 否 | 9 | 最多可以选择的文件数量，可支持选择多个图片。**使用相机拍照时该字段失效**<br>**最小值**：`1`<br>**最大值**：`20` |
| sizeType | string[] | 否 | ["original","compressed"] | 表示是选择原图还是对图片质量进行压缩。<br>**可选值**：<br>- `["original"]`：选择原图<br>- `["compressed"]`：强制对图片质量进行压缩<br>- `["original","compressed"]`: 默认开启压缩，但可手动选择原图<br><md-alert type="tip" icon="none"><br>- Android/iOS/PC 端：Lark[V3.38.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：不支持`["original","compressed"]`，默认为`["original"]`<br></md-alert> |
| cameraDevice | string | 否 | back | 使用相机拍摄的默认摄像头，仅iOS支持且在sourceType为camera时生效<br>**可选值**：<br>- `front`：前置摄像头<br>- `back`：后置摄像头<br><md-alert type="tip" icon="none"><br>- iOS 端：Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC/Android 端：暂不支持<br></md-alert> |
| isSaveToAlbum | string | 否 | 0 | 使用相机拍摄后图片是否保存到相册，仅iOS和Android支持且在sourceType为camera时生效<br>**可选值**：<br>- `"0"`：不保存<br>- `"1"`：保存<br><md-alert type="tip" icon="none"><br>- iOS 端：Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>-  Android 端 Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：暂不支持<br></md-alert> |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| tempFiles | object[] | 图片对象数组 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>path<br></md-text> | string | 图片路径 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>size<br></md-text> | number | 图片大小，单位byte |
| tempFilePaths | string[] | 图片路径数组 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseImage({
  sourceType: ['album', 'camera'], // PC端无效
  count: 10,
  sizeType:['compressed'],
  cameraDevice: 'front',
  isSaveToAlbum: '0',
  success (res) {
    console.log(res.tempFilePaths, res.tempFiles);
  },
  fail (res) {
    console.log(`chooseImage 调用失败`);
  }
});
```

`success`返回对象示例：

```json
{
  "tempFilePaths": ["ttfile://temp/1637489223734.jpg"],
  "tempFiles": [
    {
      "path": "ttfile://temp/1637489223734.jpg",
      "size": 14247
    }
  ],
  "errMsg": "chooseImage:ok"
}

``` 
