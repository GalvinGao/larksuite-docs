---
document_id: '7073692582769786886'
directory_id: '7073451436033867781'
title: RecorderManager.resume
full_path: /uYjL24iN/uATMx4CMxEjLwETM/recordermanager/resume
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Record
- RecorderManager
- RecorderManager.resume
document_type: GuideDocumentType
updated_at: 2022-04-11T02:51:35Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/resume
---

# RecorderManager.resume()


继续录音


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
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/voice/voice" fontSize="14">预览</md-preview-app>
        </md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
        <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14" disable="true">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


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

recorderManager.resume();
```


