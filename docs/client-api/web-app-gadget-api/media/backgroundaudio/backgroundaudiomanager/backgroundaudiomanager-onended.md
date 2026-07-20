---
document_id: '7180231348975976454'
directory_id: '7174249976830492677'
title: BackgroundAudioManager.onEnded
full_path: /uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onended
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- BackgroundAudio
- backgroundAudioManager
- BackgroundAudioManager.onEnded
document_type: GuideDocumentType
updated_at: 2022-12-23T07:01:47Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onended
---

# BackgroundAudioManager.onEnded(function callback)


监听背景音频自然播放至结束的事件



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
      <md-td><md-version>V5.20.0+</md-version></md-td>
      <md-td><md-version>V5.20.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/background-audio/backgroundAudio" fontSize="14">预览</md-preview-app>
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14" disable=true>预览</md-preview-app></md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性
## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>
  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/background-audio/backgroundAudio" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>

</div> 
:::

```js
const bam = this.backgroundAudioManager = tt.getBackgroundAudioManager();
bam.src = 'https://someaudiourl';
const cb = ()=>{console.log(“音频已全部播放结束”)};
bam.onEnded(cb);
```


