---
document_id: '7073692582770409478'
directory_id: '6907567266536357889'
title: createVideoContext
full_path: /uYjL24iN/uITMx4iMxEjLyETM/createvideocontext
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- createVideoContext
document_type: GuideDocumentType
updated_at: 2023-05-22T07:36:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITMx4iMxEjLyETM/createvideocontext
---

# 	createVideoContext(string id, object component)

 
创建 `VideoContext` 实例，通过 id 跟一个 `video` 组件绑定，操作对应的 `video` 组件。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V4.3.0+</md-version> | <md-version>V4.3.0+</md-version> | <md-version>V6.1.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/video/video" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| id | string | 是 |  | video 组件 id<br>**示例值**：myVideo |
| component | object | 否 |  | 在自定义组件下，当前组件实例的this，以操作组件内 video 组件 |



## 输出

返回值：`VideoContext`，该对象的方法列表参见下表：

| 方法 | 介绍 |
| --- | --- |
| [VideoContext.play](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/play) | 播放视频 |
| [VideoContext.pause](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/pause) | 暂停视频 |
| [VideoContext.stop](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/stop) | 停止视频 |
| [VideoContext.seek](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/seek) | 跳转到指定位置 |
| [VideoContext.requestFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/requestfullscreen) | 进入全屏 |
| [VideoContext.exitFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/exitfullscreen) | 退出全屏 |
| [VideoContext.playbackRate](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/playbackrate) | 设置视频倍速播放 |


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
  },
  pause: function () {
      this.videoContext.pause();
  }
})
```

