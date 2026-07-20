---
document_id: '6965379543684595718'
directory_id: '6907567266541158402'
title: onAccelerometerChange
full_path: /uYjL24iN/uEzNx4SM3EjLxcTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Accelerometer
- onAccelerometerChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzNx4SM3EjLxcTM
---

# onAccelerometerChange(function callback)

监听加速度计数据。注册回调后一旦数据变化会收到结果。

::: note
调用该方法时若未打开加速度计，会调用一次 [startAccelerometer](/document/uYjL24iN/ukjNx4SO2EjL5YTM) 方法。
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
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-accelerometer-change/on-accelerometer-change" fontSize="14">预览</md-preview-app>
      </md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td>**X**</md-td>
      	  <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
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
                x
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                x 轴数据
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                y
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                y 轴数据
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                z
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                z 轴数据
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/on-accelerometer-change/on-accelerometer-change" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true"  appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.startAccelerometer();
tt.onAccelerometerChange(function(res) {
    console.log(JSON.stringify(res));
});
```

回调函数返回对象示例：
```json
{
    "y": -0.5976561903953552,
    "z": -0.808624267578125,
    "x": 0.1167755201458931
}
```
