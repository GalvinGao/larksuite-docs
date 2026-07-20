---
document_id: '6965379541104115717'
directory_id: '6907567266541830146'
title: createInnerAudioContext
full_path: /uYjL24iN/uUDOx4SN4EjL1gTM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Audio
- createInnerAudioContext
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:30Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDOx4SN4EjL1gTM
---

# `createInnerAudioContext`

创建`innerAudioContext`实例，通过它能够操作音频播放。

::: note
**PC端暂不支持该接口**
:::

## `innerAudioContext`的属性列表

名称 | 数据类型 | 属性 | 默认值| 描述
--|--|--|--|--
`src` | `string` |  | N/A | 音频源地址
`startTime` | `number` |  | `0` | 开始播放的位置，单位 `s`
`autoplay` | `boolean` |  | `false` | 是否自动播放
`loop` | `boolean` |  | `false` | 是否自动循环
`obeyMuteSwitch` | `boolean` |  | `true` | 是否遵循系统静音开关
`duration` | `number` |  |  | 当前音频总时长，单位 `s`
`currentTime` | `number` ||  | 当前音频进度，单位 `s`
`paused` | `boolean` |  |  | 当前音频是否处于暂停状态
`buffered` | `number` |  |  | 当前音频已缓冲部分百分比
`volume` | `number` |  |  | 当前音量

## `innerAudioContext`的方法列表

### `play()`
播放
### `pause()`
暂停播放
### `stop()`
停止播放
### `seek(position)`
跳转到`position`指定的位置播放，数据格式为`number`，单位为`s`
### `destroy()`
销毁当前`innerAudioContext`实例
### `onCanplay(() => {})`
音频进入可以播放状态，但不保证后面可以流畅播放
### `offCanplay(() => {})`
取消监听 `Canplay` 事件
### `onPlay(() => {})`
音频播放事件
### `offPlay(() => {})`
取消监听 `Play` 事件
### `onPause(() => {})`
音频暂停事件
### `offPause(() => {})`
取消监听 `Pause` 事件
### `onStop(() => {})`
音频停止事件
### `offStop(() => {})`
取消监听 `Stop` 事件
### `onEnded(() => {})`
音频自然播放结束事件
### `offEnded(() => {})`
取消监听 `Ended` 事件
### `onTimeUpdate(() => {})`
音频播放进度更新事件
### `offTimeUpdate(() => {})`
取消监听 `TimeUpdate` 事件
### `onError((error) => {})`
音频播放错误事件
### `offError(() => {})`
取消监听 `Error` 事件
### `onWaiting(() => {})`
音频加载中事件，当音频因为数据不足，需要停下来加载时会触发	
### `offWaiting(() => {})`
取消监听 `Waiting` 事件
### `onSeeking(() => {})`
音频进行 seek 操作事件
### `offSeeking(() => {})`
取消监听 `Seeking` 事件
### `onSeeked(() => {})`
音频完成 seek 操作事件
### `offSeeked(() => {})`
取消监听 `Seeked` 事件

## 代码示例

```js
const innerAudioContext = tt.createInnerAudioContext();
innerAudioContext.autoplay = true;
innerAudioContext.src = 'https://someaudiourl';
innerAudioContext.onPlay(() => {
    console.log('开始播放');
});
innerAudioContext.onError((error) => {
    console.log(error)
});
innerAudioContext.onTimeUpdate((res) => {
    this.setData({
        progress: innerAudioContext.currentTime / innerAudioContext.duration
    });
})
```
