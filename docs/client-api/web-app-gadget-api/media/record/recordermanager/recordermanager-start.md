---
document_id: '7073693024736051205'
directory_id: '7073451436033867781'
title: RecorderManager.start
full_path: /uYjL24iN/uATMx4CMxEjLwETM/recordermanager/start
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Record
- RecorderManager
- RecorderManager.start
document_type: GuideDocumentType
updated_at: 2022-04-11T02:51:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/start
---

# RecorderManager.start(object options)

开始录音

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/voice/voice" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✓** | **✓** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| options | object | 否 |  | 录音参数 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>duration<br></md-text> | number | 否 | 60000 | 录音的时长，单位 ms，最大值 600000（10 分钟） |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>sampleRate<br></md-text> | number | 否 | 8000 | 采样率<br>**可选值**：<br>- `8000`：8000 采样率<br>- `16000`：16000 采样率<br>- `44100`：44100 采样率 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>numberOfChannels<br></md-text> | number | 否 | 2 | 录音通道数<br>**可选值**：<br>- `1`：一个通道<br>- `2`：两个通道 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>encodeBitRate<br></md-text> | number | 否 | 48000 | 编码码率，有效值见下表格 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>frameSize<br></md-text> | number | 否 |  | 帧大小，单位 KB。如果设置了值，那么每当录音内容达到帧大小时会通过 onFrameRecorded 返回内容 |


`sampleRate`和`encodeBitRate`的对应关系如下：

采样率 | 编码码率
--|--
8000 | 16000 ~ 48000
11025 | 16000 ~ 48000
12000 | 24000 ~ 64000
16000 | 24000 ~ 96000
22050 | 32000 ~ 128000
24000 | 32000 ~ 128000
32000 | 48000 ~ 192000
44100 | 64000 ~ 320000
48000 | 64000 ~ 320000
Android端录制格式为.wav。iOS端录制格式为.aac。
## 输出
无


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

recorderManager.start(options);
```


