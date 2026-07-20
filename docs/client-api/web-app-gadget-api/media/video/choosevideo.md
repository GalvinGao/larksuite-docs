---
document_id: '6965379543683710982'
directory_id: '6907567266536357889'
title: chooseVideo
full_path: /uYjL24iN/uEjMx4SMyEjLxITM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Video
- chooseVideo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:27Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMx4SMyEjLxITM
---

# chooseVideo(Object object)

从系统相册中选择视频，或使用相机拍摄视频。


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
      <md-td>**✓**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/video/video" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44+</md-version></md-td>
      <md-td><md-version>V3.44+</md-version></md-td>
      <md-td><md-version>V3.47+</md-version></md-td>
      <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
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
            <md-td>sourceType</md-td>
            <md-td>string[]</md-td>
            <md-td>否</md-td>
            <md-td>['album', 'camera']</md-td>
            <md-td>指定视频来源为相册或/和相机

**示例值**：['album']
              
**可选值有**：
- `['album']`
- `['camera']`
- `['album','camera']`            
<md-alert type="tip" icon="none">
PC暂不支持camera
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>maxDuration</md-td>
            <md-td>number</md-td>
            <md-td>否</md-td>
            <md-td>60</md-td>
            <md-td>选取视频最长时间，单位秒。

**示例值**：80

**最大值**：`180`
              
<md-alert type="tip" icon="none">
- PC 端：不限制最大时长
- iOS
	- 当 `compressd` 为 `true` 时，`maxDuration` 默认值为 60s，最大支持选取 180s 视频
	- 当 `compressd` 为 `false` 时，`maxDuration` 默认值为 60s，不限制最大时长
- Android
	- `maxDuration` 默认值为 60s，最大支持选取 180s 视频
</md-alert>  
        
</md-td>

</md-tr>  
        
        <md-tr>
            <md-td>compressed</md-td>
            <md-td>boolean</md-td>
            <md-td>否</md-td>
            <md-td>true</md-td>
            <md-td>
                是否对选取视频进行压缩

**示例值**：true
<md-alert type="tip" icon="none">
- iOS 端：Lark[V3.37](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持,默认为不压缩，iOS 端设置为 `false` 时，不进行压缩转码，但是相册导出可能仍然需要花费一定时间
- Android/PC 端：暂不支持（不进行压缩）
</md-alert>                
</md-td>
   </md-tr>
</md-tbody>
</md-table>
:::

## 输出

`success`返回对象的扩展属性：

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
                duration
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频时长，单位：秒/s。
<md-alert type="tip" icon="none">
PC 端：暂不支持
</md-alert>  
            </md-td>
          
        </md-tr>
        <md-tr>
            <md-td>
                tempFilePath
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                视频地址
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                size
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频大小，单位：字节/Bytes
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                width
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频宽度
<md-alert type="tip" icon="none">
PC 端：暂不支持
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                height
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                视频高度
<md-alert type="tip" icon="none">
PC 端：暂不支持
</md-alert>  
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/video/video" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseVideo({
    sourceType: [
        "album"
    ],
    maxDuration: 80,
    compressed: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`chooseVideo fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "duration": 8,
    "height": 600,
    "size": 594655,
    "tempFilePath": "ttfile://temp/1637482384392.mp4",
    "width": 600,
    "errMsg": "chooseVideo:ok"
}
```


