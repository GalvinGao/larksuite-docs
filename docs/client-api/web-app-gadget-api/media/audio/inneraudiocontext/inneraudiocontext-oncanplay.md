---
document_id: '7073692582769197062'
directory_id: '7073451436033982469'
title: InnerAudioContext.onCanplay
full_path: /uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/oncanplay
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Audio
- InnerAudioContext
- InnerAudioContext.onCanplay
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:30Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/oncanplay
---

# InnerAudioContext.onCanplay(function callback)

在音频进入可播放状态后触发回调函数

:::html
<md-alert type="tip">
注意事项：
- 开始播放后仍有可能再次进入等待播放状态
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✕** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="这里一定要修改成具体的路径" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
无

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/inneraudio/inneraudio" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
const innerAudioContext = tt.createInnerAudioContext();
innerAudioContext.autoplay = true;
innerAudioContext.src = 'https://someaudiourl';
innerAudioContext.onCanPlay(() => {
    console.log('可以开始播放');
});
```




