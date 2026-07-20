---
document_id: '7073692582770311174'
directory_id: '7073451436033867781'
title: RecorderManager.onError
full_path: /uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onerror
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Record
- RecorderManager
- RecorderManager.onError
document_type: GuideDocumentType
updated_at: 2023-05-31T02:59:33Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onerror
---

# RecorderManager.onError(function callback)


监听录音错误事件


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
      <md-td>callback</md-td>
      <md-td>function</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

   </md-tr>  
    

    
</md-tbody>
</md-table>
:::

## 输出
回调函数返回对象的属性：
```javascript 
interface OnErrorDetail {
    errno: number;
    errString: string;
}
``` 
|errno|errString|含义|
|--|--|------|
|1305001|Unable to record. A higher priority media event is being processed, ${reason}|开始录制音频抢占播放失败(有优先级更高的业务方在播放)|
|1305002|Unable to record. A higher priority media event is being processed, ${reason}|恢复录制音频抢占播放失败(有优先级更高的业务方在播放)|
|1305003|OperateRecorder ${apiType} failed, reason: ${reason}|start/pause/resume/stop失败|


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

recorderManager.onError((res) => {
  console.error('recorder complete error:', res.errno, res.errString);
});

recorderManager.start(options);
```


