---
document_id: '7180231348975861766'
directory_id: '7174249976830492677'
title: BackgroundAudioManager.play
full_path: /uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/play
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- BackgroundAudio
- backgroundAudioManager
- BackgroundAudioManager.play
document_type: GuideDocumentType
updated_at: 2022-12-23T07:01:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/play
---

# BackgroundAudioManager.play()


播放背景音频。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.20.0+</md-version> | <md-version>V5.20.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/background-audio/backgroundAudio" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app> |


## 输入
无

## 输出
无
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
bam.src = 'https://someaudiourl';
bam.play();
```


