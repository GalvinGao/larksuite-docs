---
document_id: '6965379543684120582'
directory_id: '6907567266536357889'
title: saveVideoToPhotosAlbum
full_path: /uYjL24iN/ucDOx4yN4EjL3gTM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- saveVideoToPhotosAlbum
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:24Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucDOx4yN4EjL3gTM
---

# saveVideoToPhotosAlbum(Object object)

保存视频到系统相册。


:::html
<md-alert type="tip">
注意事项：
- **iOS和Android**调用前需要用户授权 `scope.writePhotosAlbum`。了解如何授权，可查看[API 权限](/document/uYjL24iN/uITMuITMuITM)。
</md-alert>
:::


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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/video/video" fontSize="14">预览</md-preview-app>
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
                filePath
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                视频文件路径。不支持网络地址。
              
**示例值**：ttfile://temp/1637482384392.mp4
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::
## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/video/video" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
    <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseVideo({
  sourceType: ['album'],
  compressed: true,
  success(res) {
    tt.saveVideoToPhotosAlbum({
      filePath: res.tempFilePath,
      success(res) {
        console.log(res);
      },
      fail(res) {
        console.log(`saveVideoToPhotosAlbum failed`);
      }
    });
  },
  fail(res) {
    console.log(`chooseVideo failed`);
  }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "saveVideoToPhotosAlbum:ok"
}
```
