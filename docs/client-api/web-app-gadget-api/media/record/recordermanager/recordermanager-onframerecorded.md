---
document_id: '7073691561007906821'
directory_id: '7073451436033867781'
title: RecorderManager.onFrameRecorded
full_path: /uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onframerecorded
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Record
- RecorderManager
- RecorderManager.onFrameRecorded
document_type: GuideDocumentType
updated_at: 2022-04-11T02:51:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onframerecorded
---

# RecorderManager.onFrameRecorded(function callback)


监听已录制完指定帧大小的文件事件。如果设置了 frameSize，则会回调此事件。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/voice/voice" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✓** | **✓** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| frameBuffer | ArrayBuffer | 帧数据 |
| isLastFrame | boolean | 是否为最后一帧 |




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
const options = {
  duration: 100000,
  sampleRate: 44100,
  numberOfChannels: 2,
  encodeBitRate: 320000,
  frameSize: 50
};

recorderManager.onFrameRecorded((res) => {
  console.log('recorder complete framebuffer:', res.frameBuffer);
  console.log('recorder complete isLastFrame:', res.isLastFrame);
});

recorderManager.start(options);
```


