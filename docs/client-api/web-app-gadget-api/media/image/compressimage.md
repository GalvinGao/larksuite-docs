---
document_id: '6965379543683923974'
directory_id: '6907567266541977602'
title: compressImage
full_path: /uYjL24iN/uMjN24yM2YjLzYjN
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Image
- compressImage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:18Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMjN24yM2YjLzYjN
---

# compressImage(Object object)

压缩图片接口，可选压缩质量。



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
      <md-td><md-version>V2.0.0+</md-version></md-td>
      <md-td><md-version>V2.0.0+</md-version></md-td>
      <md-td><md-version>V2.0.0+</md-version></md-td>
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
                src
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                图片路径，可以是相对路径、临时文件路径、存储文件路径

**示例值**：

ttfile://temp/1637489223734.jpg
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                quality
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                80
            </md-td>
            <md-td>
                压缩质量，范围是 (0,100]，数值越小，质量越低，压缩率越高（仅对 jpg 有效）

**最小值**：`0`

**最大值**：`100`

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
            <md-th style="width: 20%;">
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
                tempFilePath
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                压缩后的图片路径
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
  count: 1,
  sourceType: ['album', 'camera'],
  success: (result) => {
    tt.compressImage({
      src: result.tempFilePaths[0],
      quality: 50,
      success: (res) => {
        console.log(res);
      },
      fail: (res) => {
        console.error('compressImage failed')
      }
    })
  }
});
```

`success`返回对象示例：

```json
{
  "errMsg": "compressImage:ok",
  "tempFilePath": "ttfile://temp/668fbc22-638f-42f3-8e15-cadcbe1bc8d0-e95788e6-2949-4fde-b253-14b6b4c57e15-f9c3d20d141d0e6f32eea6c240b59544.png"
}
``` 

