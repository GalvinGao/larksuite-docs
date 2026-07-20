---
document_id: '7073691561008398341'
directory_id: '7073451436033982469'
title: InnerAudioContext.offSeeked
full_path: /uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeked
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Audio
- InnerAudioContext
- InnerAudioContext.offSeeked
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:30Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeked
---

# InnerAudioContext.offSeeked(function callback)

取消监听音频完成跳转操作的事件

:::html
<md-alert type="tip">
注意事项：
- 若不提供callBack参数，将对所有该事件的回调函数取消监听
</md-alert>
:::

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
      <md-td>**✕**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/inneraudio/inneraudio" fontSize="14">预览</md-preview-app>
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
      <md-td>否</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/inneraudio/inneraudio" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
const innerAudioContext = tt.createInnerAudioContext();
innerAudioContext.autoplay = true;
innerAudioContext.src = 'https://someaudiourl';
const cb = ()=> {console.log('跳转进度完成');}
innerAudioContext.onSeeked(cb);
innerAudioContext.offSeeked(cb);
```




