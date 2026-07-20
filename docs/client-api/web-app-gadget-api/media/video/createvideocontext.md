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
      <md-td><md-td><md-version>V6.1.0+</md-version></md-td></md-td>
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
                id
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                video 组件 id

**示例值**：myVideo
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                component
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                在自定义组件下，当前组件实例的this，以操作组件内 video 组件
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出

返回值：`VideoContext`，该对象的方法列表参见下表：

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">方法</md-th>
      <md-th style="width: 50%;">介绍</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
    <md-td>[VideoContext.play](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/play)</md-td>
    <md-td>播放视频</md-td>
  </md-tr>
<md-tr>
    <md-td>[VideoContext.pause](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/pause)</md-td>
    <md-td>暂停视频</md-td>
  </md-tr>
<md-tr>
    <md-td>[VideoContext.stop](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/stop)</md-td>
    <md-td>停止视频</md-td>
  </md-tr>
<md-tr>
    <md-td>[VideoContext.seek](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/seek)</md-td>
    <md-td>跳转到指定位置</md-td>
  </md-tr>
<md-tr>
    <md-td>[VideoContext.requestFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/requestfullscreen)</md-td>
    <md-td>进入全屏</md-td>
  </md-tr>
<md-tr>
    <md-td>[VideoContext.exitFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/exitfullscreen)</md-td>
    <md-td>退出全屏</md-td>
  </md-tr>
  <md-tr>
    <md-td>[VideoContext.playbackRate](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/playbackrate)</md-td>
    <md-td>设置视频倍速播放</md-td>
  </md-tr>
</md-tbody>
</md-table>
:::

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

