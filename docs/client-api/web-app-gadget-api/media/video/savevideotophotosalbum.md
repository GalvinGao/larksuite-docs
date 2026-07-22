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

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/video/video" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.47.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| filePath | string | 是 |  | 视频文件路径。不支持网络地址。<br>**示例值**：ttfile://temp/1637482384392.mp4 |

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
