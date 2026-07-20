---
document_id: '7073692582769917958'
directory_id: '6907567266540748802'
title: onNetworkQualityChange
full_path: /uYjL24iN/uUTNx4SN1EjL1UTM/onnetworkqualitychange
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Network Status
- onNetworkQualityChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:55Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTNx4SN1EjL1UTM/onnetworkqualitychange
---

# onNetworkQualityChange(function callback)

监听网络质量变化


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
      <md-td><md-version>V4.9.0+</md-version></md-td>
      <md-td><md-version>V4.9.0+</md-version></md-td>
      <md-td><md-version>V5.1.0+</md-version></md-td>

<md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.9.0+</md-version></md-td>
      <md-td><md-version>V4.9.0+</md-version></md-td>
      <md-td><md-version>V5.1.0+</md-version></md-td>
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
                networkQualityType
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                网络分级类型，同 [getNetworkQualityType](/document/uYjL24iN/uUTNx4SN1EjL1UTM/getnetworkqualitytype) 中描述

            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码



```js
tt.onNetworkQualityChange(function(res) {
    console.log(JSON.stringify(res));
});
```

回调函数返回对象示例：
```json
{
    "networkQualityType": "excellent"
}
```
 



