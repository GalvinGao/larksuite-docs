---
document_id: '7073691561007890437'
directory_id: '7073451436033916933'
title: VideoContext.requestFullScreen
full_path: /uYjL24iN/uITMx4iMxEjLyETM/videocontext/requestfullscreen
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- VideoContext
- VideoContext.requestFullScreen
document_type: GuideDocumentType
updated_at: 2023-05-22T07:36:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/requestfullscreen
---

# VideoContext.requestFullScreen()

进入全屏

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.3.0+</md-version> | <md-version>V4.3.0+</md-version> | <md-version>V6.1.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/video/video" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入
无


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
    this.VideoContext.requestFullScreen();
  }
})
```


