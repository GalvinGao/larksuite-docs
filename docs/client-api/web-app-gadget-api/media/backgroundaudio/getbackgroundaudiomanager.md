---
document_id: '7180231348975845382'
directory_id: '7174249976830509061'
title: getBackgroundAudioManager
full_path: /uYjL24iN/ukDOukDOukDO/backgroundaudio/getbackgroundaudiomanager
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- BackgroundAudio
- getBackgroundAudioManager
document_type: GuideDocumentType
updated_at: 2022-12-23T07:01:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/getbackgroundaudiomanager
---

# getBackgroundAudioManager()

创建`backgroundAudioManager`实例，通过它能够操作背景音频播放。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.20.0+</md-version> | <md-version>V5.20.0+</md-version> | **✕** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/background-audio/backgroundAudio" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |


## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| src | string | 是 |  | 要播放背景音频的资源地址，目前仅支持网络url路径 |
| startTime | number | 否 |  | 开始播放的位置（单位：s）,默认为0 |
| title | string | 否 |  | 背景音频标题, 用于原生音频播放器背景音频标题。默认为小程序名称。 |
| coverImgUrl | string | 否 |  | 背景音频封面图 URL，用于做原生音频播放器背景图。浮窗控件展示图片url。默认是小程序图标 |
| audioPage | object | 否 |  | 设置点击悬浮控件跳转当前小程序指定页面，数据格式为 {path:"(音乐播放路径)",query:{name:''}}。默认为启动到小程序离开的页面。 |
| duration | number | 否 |  | 当前背景音频总时长(单位 s)，只读 |
| currentTime | number | 否 |  | 当前背景音频进度(单位 s)，只读 |
| paused | boolean | 否 |  | 当前背景音频是否处于暂停状态，只读 |
| buffered | number | 否 |  | 当前背景音频已缓冲部分(单位 s)，只读 |
| playbackRate | number | 否 |  | 播放速度。范围 0.5 ～ 2.0，默认为 1.0 |


## 输出
返回值：`backgroundAudioManager`，该对象的方法列表参见下表：


:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [play](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/play) | 播放 |
| [pause](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/pause) | 暂停播放 |
| [stop](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/stop) | 停止播放 |
| [seek](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/seek) | 跳转到指定位置播放，数据格式为number，单位为s |
| [onCanplay](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/oncanplay) | 监听背景音频 Canplay 状态，但不保证后面可以流畅播放 |
| [onPlay](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onplay) | 监听背景音频 Play 事件 |
| [onPause](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onpause) | 监听背景音频 Pause 事件 |
| [onStop](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onstop) | 监听背景音频 Stop 事件 |
| [onEnded](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onended) | 监听背景音频 Ended 事件 |
| [onTimeUpdate](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/ontimeupdate) | 监听背景音频 TimeUpdate 事件 |
| [onError](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onerror) | 监听背景音频 Error 事件 |
| [onWaiting](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onwaiting) | 监听背景音频 Waiting 事件，当背景音频因为数据不足，需要停下来加载时会触发 |
| [onSeeking](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onseeking) | 背景音频进行 seek 操作事件 |
| [onSeeked](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onseeked) | 背景音频完成 seek 操作事件 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/background-audio/backgroundAudio" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
const bam = this.backgroundAudioManager = tt.getBackgroundAudioManager();
bam.src = 'https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/fb66cd3a25fe6077f20c8d2f81e4db83.mp3';
bam.title = '背景音频';
bam.playbackRate = 1.0
bam.onPlay(() => {
    console.log('开始播放');
});
bam.onError((error) => {
    console.log(error)
});
bam.onTimeUpdate((res) => {
    this.setData({
        progress: bam.currentTime / bam.duration
    });
})
```
