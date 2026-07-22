---
document_id: '6965379543683334150'
directory_id: '6907567266537652225'
title: getRecorderManager
full_path: /uYjL24iN/uQDOx4CN4EjL0gTM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Record
- getRecorderManager
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:33Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQDOx4CN4EjL0gTM
---

# 	getRecorderManager()


获取全局唯一的 `recorderManager` 。通过 `recorderManager` 进行录音操作和管理。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/voice/voice" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✓** | **✓** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 输入
无
## 输出

返回值：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| recorderManager | object | [RecorderManager](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/recordermanager) 对象 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[start](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/start)<br></md-text> | function | 开始录音 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[pause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/pause)<br></md-text> | function | 暂停录音 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[resume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/resume)<br></md-text> | function | 继续录音 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[stop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/stop)<br></md-text> | function | 停止录音 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[onStart](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstart)<br></md-text> | function | 录音开始事件回调 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[onPause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onpause)<br></md-text> | function | 录音暂停事件回调 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[onResume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onresume)<br></md-text> | function | 录音恢复事件回调 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[onStop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstop)<br></md-text> | function | 录音停止事件回调，res对象带有一个类型为string的属性tempFilePath，表示录音文件的地址。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[onFrameRecorded](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onframerecorded)<br></md-text> | function | 监听已录制完指定帧大小的文件事件。如果设置了 frameSize，则会回调此事件。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[onError](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onerror)<br></md-text> | function | 监听录音错误事件 |


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
