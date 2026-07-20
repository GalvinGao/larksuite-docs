---
document_id: '7180231348975845382'
directory_id: '7174249976830509061'
title: getBackgroundAudioManager
full_path: /uYjL24iN/ukDOukDOukDO/backgroundaudio/getbackgroundaudiomanager
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- BackgroundAudio
- getBackgroundAudioManager
document_type: GuideDocumentType
updated_at: 2022-12-23T07:01:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/getbackgroundaudiomanager
---

# getBackgroundAudioManager()

创建`backgroundAudioManager`实例，通过它能够操作背景音频播放。

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
      <md-td>**✕**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/background-audio/backgroundAudio" fontSize="14">预览</md-preview-app>
      </md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**✕**</md-td>
      <md-td>**✕**</md-td>
      <md-td>**✕**</md-td>
      <md-td>/</md-td>
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
                src
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                要播放背景音频的资源地址，目前仅支持网络url路径
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                startTime
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
开始播放的位置（单位：s）,默认为0
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                title	
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
背景音频标题, 用于原生音频播放器背景音频标题。默认为小程序名称。
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                coverImgUrl
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
背景音频封面图 URL，用于做原生音频播放器背景图。浮窗控件展示图片url。默认是小程序图标
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                audioPage	
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
设置点击悬浮控件跳转当前小程序指定页面，数据格式为 {path:"(音乐播放路径)",query:{name:''}}。默认为启动到小程序离开的页面。
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                duration	
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
当前背景音频总时长(单位 s)，只读
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                currentTime	
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
当前背景音频进度(单位 s)，只读
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                paused
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
当前背景音频是否处于暂停状态，只读
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                buffered	
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
当前背景音频已缓冲部分(单位 s)，只读
            </md-td>
        </md-tr>
      
                    <md-tr>
            <md-td>
                playbackRate
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
播放速度。范围 0.5 ～ 2.0，默认为 1.0
            </md-td>
        </md-tr>
      
      
    </md-tbody>
</md-table>
:::

## 输出
返回值：`backgroundAudioManager`，该对象的方法列表参见下表：


:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">方法</md-th>
      <md-th style="width: 50%;">介绍</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
    <md-td>[play](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/play)</md-td>
    <md-td>播放</md-td>
  </md-tr>
<md-tr>
    <md-td>[pause](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/pause)</md-td>
    <md-td>暂停播放</md-td>
  </md-tr>
<md-tr>
    <md-td>[stop](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/stop)</md-td>
    <md-td>停止播放</md-td>
  </md-tr>
<md-tr>
    <md-td>[seek](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/seek)</md-td>
    <md-td>跳转到指定位置播放，数据格式为number，单位为s</md-td>
  </md-tr>

<md-tr>
    <md-td>[onCanplay](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/oncanplay)</md-td>
    <md-td>监听背景音频 Canplay 状态，但不保证后面可以流畅播放</md-td>
  </md-tr>

<md-tr>
    <md-td>[onPlay](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onplay)</md-td>
    <md-td>监听背景音频 Play 事件</md-td>
  </md-tr>

<md-tr>
    <md-td>[onPause](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onpause)</md-td>
    <md-td>监听背景音频 Pause 事件</md-td>
  </md-tr>

<md-tr>
    <md-td>[onStop](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onstop)</md-td>
    <md-td>监听背景音频 Stop 事件</md-td>
  </md-tr>

<md-tr>
    <md-td>[onEnded](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onended)</md-td>
    <md-td>监听背景音频 Ended 事件</md-td>
  </md-tr>

<md-tr>
    <md-td>[onTimeUpdate](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/ontimeupdate)</md-td>
    <md-td>监听背景音频 TimeUpdate 事件</md-td>
  </md-tr>

<md-tr>
    <md-td>[onError](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onerror)</md-td>
    <md-td>监听背景音频 Error 事件</md-td>
  </md-tr>

<md-tr>
    <md-td>[onWaiting](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onwaiting)</md-td>
    <md-td>监听背景音频 Waiting 事件，当背景音频因为数据不足，需要停下来加载时会触发</md-td>
  </md-tr>

<md-tr>
    <md-td>[onSeeking](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onseeking)</md-td>
    <md-td>背景音频进行 seek 操作事件</md-td>
  </md-tr>

<md-tr>
    <md-td>[onSeeked](/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/backgroundaudiomanager/onseeked)</md-td>
    <md-td>背景音频完成 seek 操作事件</md-td>
  </md-tr>


</md-tbody>
</md-table>
:::

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
bam.src = 'https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/fb66cd3a25fe6077f20c8d2f81e4db83.mp3';
bam.title = '背景音频';
bam.playbackRate = 1.0
bam.onPlay(() => {
    console.log('开始播放');
});
bam.onError((error) => {
    console.log(error)
});
bam.onTimeUpdate((res) => {
    this.setData({
        progress: bam.currentTime / bam.duration
    });
})
```
