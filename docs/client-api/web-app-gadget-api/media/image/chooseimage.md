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
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
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
                指定图片来源为相册或/和相机



**可选值**：
- `["album"]`：相册
- `["camera"]`：相机
- `["album", "camera"]`: 相册或相机，
<md-alert type="tip" icon="none">
- PC 端：暂不支持，默认为本地文件系统
- iOS 端：不支持同时从 album 和 camera 中选择，只能二者选其一
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
                最多可以选择的文件数量，可支持选择多个图片。**使用相机拍照时该字段失效**

**最小值**：`1`
              
**最大值**：`20`
 

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
                表示是选择原图还是对图片质量进行压缩。



**可选值**：
- `["original"]`：选择原图
- `["compressed"]`：强制对图片质量进行压缩
- `["original","compressed"]`: 默认开启压缩，但可手动选择原图
<md-alert type="tip" icon="none">
- Android/iOS/PC 端：Lark[V3.38.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC 端：不支持`["original","compressed"]`，默认为`["original"]`
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
                使用相机拍摄的默认摄像头，仅iOS支持且在sourceType为camera时生效



**可选值**：
- `front`：前置摄像头
- `back`：后置摄像头
<md-alert type="tip" icon="none">
- iOS 端：Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC/Android 端：暂不支持
</md-alert> 
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
-  Android 端 Lark[V5.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
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
                图片对象数组
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
                    path
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                图片路径
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
                图片大小，单位byte
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                tempFilePaths
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                图片路径数组
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
