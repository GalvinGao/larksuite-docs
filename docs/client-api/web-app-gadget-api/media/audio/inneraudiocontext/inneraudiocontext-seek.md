---
document_id: '7073692582769557510'
directory_id: '7073451436033982469'
title: InnerAudioContext.seek
full_path: /uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/seek
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Audio
- InnerAudioContext
- InnerAudioContext.seek
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:30Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/seek
---

# InnerAudioContext.seek(number position)

跳转到指定位置


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✕** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/inneraudio/inneraudio" fontSize="14">预览</md-preview-app> |
| 网页应用 | **✕** | **✕** | **✕** | / |



## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| position | number | 是 |  | 跳转到指定的位置播放，单位为 s |




## 输出
无



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/inneraudio/inneraudio" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
const innerAudioContext = tt.createInnerAudioContext();
innerAudioContext.autoplay = true;
innerAudioContext.src = 'https://someaudiourl';
innerAudioContext.seek(1);
```


