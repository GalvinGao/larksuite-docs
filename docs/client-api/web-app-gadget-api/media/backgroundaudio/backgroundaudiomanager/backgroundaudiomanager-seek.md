---
document_id: '7180231348975812614'
directory_id: '7174249976830492677'
title: BackgroundAudioManager.seek
full_path: /uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/seek
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- BackgroundAudio
- backgroundAudioManager
- BackgroundAudioManager.seek
document_type: GuideDocumentType
updated_at: 2022-12-23T07:01:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/seek
---

# BackgroundAudioManager.seek(number position)


背景音频跳转到指定位置播放

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

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
      <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>position</md-td>
      <md-td>number</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        跳转到指定的位置播放，单位为 s


      </md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::


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
bam.seek(1);
```


