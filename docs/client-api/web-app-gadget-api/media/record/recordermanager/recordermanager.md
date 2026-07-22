---
document_id: '7073691561007661061'
directory_id: '7073451436033867781'
title: RecorderManager
full_path: /uYjL24iN/uATMx4CMxEjLwETM/recordermanager/recordermanager
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Record
- RecorderManager
- RecorderManager
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:36Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/recordermanager
---

# RecorderManager


全局唯一的录音管理器，可通过
[getRecorderManager](/document/uYjL24iN/uQDOx4CN4EjL0gTM)获取。

::: note
调用前需要用户授权 `scope.record`，**请开发者兼容用户拒绝授权的场景**。
该页面假设你已经阅读过了[API 权限](/document/uYjL24iN/uITMuITMuITM)。如果你对小程序 API 权限还不太了解，推荐你先阅读它。
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/voice/voice" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✓** | **✓** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 方法

| 方法 | 介绍 |
| --- | --- |
| [RecorderManager.start](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/start) | 开始录音 |
| [RecorderManager.pause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/pause) | 暂停录音 |
| [RecorderManager.resume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/resume) | 继续录音 |
| [RecorderManager.stop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/stop) | 停止录音 |
| [RecorderManager.onStart](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstart) | 录音开始事件回调 |
| [RecorderManager.onPause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onpause) | 录音暂停事件回调 |
| [RecorderManager.onResume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onresume) | 录音恢复事件回调 |
| [RecorderManager.onStop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstop) | 录音停止事件回调，res对象带有一个类型为string的属性tempFilePath，表示录音文件的地址。 |
| [RecorderManager.onFrameRecorded](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onframerecorded) | 监听已录制完指定帧大小的文件事件。如果设置了 frameSize，则会回调此事件。 |
| [RecorderManager.onError](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onerror) | 监听录音错误事件 |



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/voice/voice" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
        <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
const recorderManager = tt.getRecorderManager();

recorderManager.onStart(() => {
    console.log('recorder start')
});
recorderManager.onPause(() => {
    console.log('recorder pause')
});
recorderManager.onResume(() => {
    console.log('recorder onResume')
});
recorderManager.onStop((res) => {
    console.log('recorder stop', res)
    const { tempFilePath } = res
});
recorderManager.onFrameRecorded((res) => {
    const { frameBuffer } = res
    console.log('frameBuffer.byteLength', frameBuffer.byteLength)
});

const options = {
    duration: 10000,
    sampleRate: 44100,
    numberOfChannels: 1,
    encodeBitRate: 192000,
    format: 'aac',
    frameSize: 50
};

recorderManager.start(options);
```

## 已知问题

- `start` 方法暂不支持 `audioSource` 参数
-  android 暂不支持 `mp3` 和 `aac`，只支持 `wav` 格式
-  iOS  暂不支持 `mp3` 和 `wav`，只支持 `aac` 格式（文件名后缀通常为 `.m4a`, `.aac`, `.mp4`）
