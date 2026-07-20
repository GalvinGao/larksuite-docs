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

返回值：
:::html

<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                recorderManager
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                [RecorderManager](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/recordermanager) 对象
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [start](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/start)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                开始录音
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [pause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/pause)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                暂停录音
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [resume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/resume)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                继续录音
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [stop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/stop)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                停止录音
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [onStart](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstart)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                录音开始事件回调
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [onPause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onpause)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                录音暂停事件回调
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [onResume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onresume)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                录音恢复事件回调
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [onStop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstop)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                录音停止事件回调，res对象带有一个类型为string的属性tempFilePath，表示录音文件的地址。
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [onFrameRecorded](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onframerecorded)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                监听已录制完指定帧大小的文件事件。如果设置了 frameSize，则会回调此事件。
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    [onError](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onerror)
                </md-text>
            </md-td>
            <md-td>
                function
            </md-td>
            <md-td>
                监听录音错误事件
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

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
