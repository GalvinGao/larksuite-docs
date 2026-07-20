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
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td><md-version>V4.7.0+</md-version></md-td>
      <md-td><md-version>V4.7.0+</md-version></md-td>
      <md-td><md-version>V4.7.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/choose-media/choose-media" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.7.0+</md-version></md-td>
      <md-td><md-version>V4.7.0+</md-version></md-td>
      <md-td><md-version>V4.7.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                mediaType
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                ["image", "video"]
            </md-td>
            <md-td>
                文件类型，图片或/和视频


**可选值**：
- `["image"]`：图片
- `["video"]`：视频
- `["video", "image"]`：视频或图片，不分顺序
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                sourceType
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                ["album", "camera"]
            </md-td>
            <md-td>
                指定视频来源为相册或/和相机


**可选值**：
- `["album"]`：相册
- `["camera"]`：相机
- `["album", "camera"]`：相册或相机，不分顺序
<md-alert type="tip" icon="none">
- PC 端：暂不支持，默认为本地文件系统
</md-alert> 
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                count
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                9
            </md-td>
            <md-td>
             最多可以选择的文件数量，可支持选择多个图片或多个视频。Lark V5.30前，最多可支持9个文件，V5.30及以后最多可支持20个文件，使用相机拍照拍视频时该字段失效

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                sizeType
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                ["original","compressed"]
            </md-td>
            <md-td>
                表示是选择原图或原视频，还是对图片或视频质量进行压缩

**可选值**：
- `["original"]`：选择原图
- `["compressed"]`：强制对图片或视频质量进行压缩
- `["original","compressed"]`：默认开启压缩，但可手动选择原图或原视频
<md-alert type="tip" icon="none">
- Android/PC 端：不支持视频压缩
- PC 端：不支持 `["original","compressed"]`，默认为`["original"]`
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                maxDuration
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                60
            </md-td>
            <md-td>
                拍摄视频最长拍摄时间，单位秒。时间范围为 3s 至 60s 之间。不限制相册。

**示例值**：30
<md-alert type="tip" icon="none">
- PC 端：暂不支持此参数，不限制最大时长
</md-alert> 
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                cameraDevice
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                back
            </md-td>
            <md-td>
                使用相机拍摄的默认摄像头
<md-alert type="tip" icon="none">
- PC 端：暂不支持
</md-alert>  

**可选值**：
- `front`：前置摄像头
- `back`：后置摄像头
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                isSaveToAlbum
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                0
            </md-td>
            <md-td>
                使用相机拍摄后图片是否保存到相册，仅iOS和Android支持且在sourceType为camera时生效



**可选值**：
- `"0"`：不保存
- `"1"`：保存
<md-alert type="tip" icon="none">
- iOS 端：Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
-  Android 端：Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC 端：暂不支持

</md-alert> 
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出


`success`返回对象的扩展属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                tempFiles
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                临时文件数组
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    tempFilePath
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                文件地址
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    type
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                文件类型，有效值有image、video，分别对应图片和视频
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    size
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频大小，单位：字节/Bytes
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    duration
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频时长，单位：秒/s。选取文件类型为video时返回该字段
<md-alert type="tip" icon="none">
- PC 端：暂不支持
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    width
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频宽度。选取文件类型为video时时返回该字段
<md-alert type="tip" icon="none">
- PC 端：暂不支持
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    height
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频高度。选取文件类型为video时返回该字段
<md-alert type="tip" icon="none">
- PC 端：暂不支持
</md-alert>  
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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
