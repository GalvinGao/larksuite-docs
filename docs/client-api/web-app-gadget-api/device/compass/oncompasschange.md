---
document_id: '6965379541103968261'
directory_id: '6907567266541371394'
title: onCompassChange
full_path: /uYjL24iN/uQzNx4CN3EjL0cTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Compass
- onCompassChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:37Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQzNx4CN3EjL0cTM
---

# onCompassChange(function callback)

监听罗盘数据变化事件，频率：5 次/秒，接口调用后会自动开始监听，可使用 [stopCompass](/document/uYjL24iN/uMzNx4yM3EjLzcTM) 停止监听。

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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-compass-change/on-compass-change" fontSize="14">预览</md-preview-app>
      </md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
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
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

   </md-tr>  
    

    
</md-tbody>
</md-table>
:::

## 输出
回调函数返回对象的属性：

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
                direction
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                面对的方向度数
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-compass-change/on-compass-change" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.onCompassChange(function(res) {
    console.log(JSON.stringify(res));
});
```

返回值示例：
```json
{
    "direction": 86.3572986955703
}
```
