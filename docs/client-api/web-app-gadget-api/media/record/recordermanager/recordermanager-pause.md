---
document_id: '7073692582768869382'
directory_id: '7073451436033867781'
title: RecorderManager.pause
full_path: /uYjL24iN/uATMx4CMxEjLwETM/recordermanager/pause
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Record
- RecorderManager
- RecorderManager.pause
document_type: GuideDocumentType
updated_at: 2022-04-11T02:51:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/pause
---

# RecorderManager.pause()


暂停录音


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/voice/voice" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✓** | **✓** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app> |



## 输入

无

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
recorderManager.pause();
```


