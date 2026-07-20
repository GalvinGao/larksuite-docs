---
document_id: '7073692582769098758'
directory_id: '7073451436033916933'
title: VideoContext.seek
full_path: /uYjL24iN/uITMx4iMxEjLyETM/videocontext/seek
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- VideoContext
- VideoContext.seek
document_type: GuideDocumentType
updated_at: 2023-05-22T07:36:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/seek
---

# VideoContext.seek(number position)

跳转到指定位置

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
      <md-td><md-version>V4.3.0+</md-version></md-td>
      <md-td><md-version>V4.3.0+</md-version></md-td>
      <md-td><md-version>V6.1.0+</md-version></md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/video/video" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::



## 输入
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
                position
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                视频位置，单位秒(s)

**示例值**：20
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
无


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/video/video" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>

  </div>
</div> 
:::

```html
<video id="myVideo"
  src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4bc8f1f99761a8ab242a337592ff7f9f.mp4"
  loop="{{true}}"
  initial-time=5
  auto-fullscreen="{{true}}"
  bindtimeupdate="bindupdatetime"
  muted="{{myMuted}}">
</video>
```

```js
Page({
  onReady (res) {
    this.videoContext = tt.createVideoContext('myVideo');
    this.videoContext.play();
    this.VideoContext.seek(20);
  }
})
```




