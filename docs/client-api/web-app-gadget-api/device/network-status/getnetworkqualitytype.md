---
document_id: '7073693024735854597'
directory_id: '6907567266540748802'
title: getNetworkQualityType
full_path: /uYjL24iN/uUTNx4SN1EjL1UTM/getnetworkqualitytype
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Network Status
- getNetworkQualityType
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:55Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTNx4SN1EjL1UTM/getnetworkqualitytype
---

# getNetworkQualityType(Object object)


网络评级接口，获取当前设备所处的网络状态




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
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


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
                networkQualityType
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                网络分级类型
              
**可选值**：
- `unavailable`：无网络
- `weak`：弱网络
- `moderate`：中等网络
- `excellent`：良好网络
- `unknown`：如果设备无法确定网络分级，则会返回该值
                
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::



## 示例代码


```js
tt.getNetworkQualityType({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getNetworkQualityType fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "networkQualityType": "excellent",
    "errMsg": "getNetworkQualityType:ok"
}
```

## 已知问题

- 本接口在 iOS 5.6 版本存在异常，会返回固定值 `excellent`

