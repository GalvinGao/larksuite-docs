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
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                options
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                录音参数
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
                    duration
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                60000
            </md-td>
            <md-td>
                录音的时长，单位 ms，最大值 600000（10 分钟）
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
                    sampleRate
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                8000
            </md-td>
            <md-td>
                采样率

**可选值**：
- `8000`：8000 采样率
- `16000`：16000 采样率
- `44100`：44100 采样率
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
                    numberOfChannels
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                2
            </md-td>
            <md-td>
                录音通道数
**可选值**：
- `1`：一个通道
- `2`：两个通道
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
                    encodeBitRate
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                48000
            </md-td>
            <md-td>
                编码码率，有效值见下表格
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
                    frameSize
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                帧大小，单位 KB。如果设置了值，那么每当录音内容达到帧大小时会通过 onFrameRecorded 返回内容
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

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


