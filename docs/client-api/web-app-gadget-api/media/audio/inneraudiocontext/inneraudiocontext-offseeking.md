---
document_id: '7073693024735363077'
directory_id: '7073451436033982469'
title: InnerAudioContext.offSeeking
full_path: /uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeking
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Audio
- InnerAudioContext
- InnerAudioContext.offSeeking
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:30Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeking
---

# InnerAudioContext.offSeeking(function callback)

取消监听音频进行跳转操作的事件

:::html
<md-alert type="tip">
注意事项：
- 若不提供callBack参数，将对所有该事件的回调函数取消监听
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✕** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/inneraudio/inneraudio" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 否 |  | 该事件的回调函数 |


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
const cb = ()=> {console.log('正在跳转进度');}
innerAudioContext.onSeeking(cb);
innerAudioContext.offSeeking(cb);
```




