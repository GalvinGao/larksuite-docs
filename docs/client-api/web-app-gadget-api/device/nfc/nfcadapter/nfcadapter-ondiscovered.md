---
document_id: '6971043590353960965'
directory_id: '6907567266540699650'
title: NFCAdapter.onDiscovered
full_path: /uYjL24iN/uUDN4YjL1QDO24SN0gjN
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- NFC
- NFCAdapter
- NFCAdapter.onDiscovered
document_type: GuideDocumentType
updated_at: 2024-03-07T08:41:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDN4YjL1QDO24SN0gjN
---

# NFCAdapter.onDiscovered(function callback)

监听 NFC Tag

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
      <md-td><md-version>V3.38.0+</md-version></md-td>
      <md-td><md-version>V5.25.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V5.25.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14"disable=true>预览</md-preview-app></md-td>
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
                techs
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                tech 数组，用于匹配NFC卡片具体可以使用什么标准（NfcA等实例）处理
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                message
            </md-td>
            <md-td>
                object[]
            </md-td>
            <md-td>
                NdefMessage 数组
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                uid
            </md-td>
            <md-td>
                arraybuffer
            </md-td>
            <md-td>
                NFC标签的UID
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/nfc/nfc" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16" disable=true>预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
const nfcAdapter = tt.getNFCAdapter();
nfcAdapter.onDiscovered(function(res) {
    console.log(JSON.stringify(res));
});
```

回调函数返回对象示例：

```json
{
    "techs":[
        "ISO-DEP",
        "NFC-A",
        "NFC-A",
        "MIFARE-Classic"
    ],
    "__nativeBuffers__":[
        {
            "key":"uid",
            "base64":"efvPNA=="
        }
    ],
    "messages":[

    ]
}
``` 

## 错误码
`fail`返回对象中会包含[errno属性](/document/uYjL24iN/uAjMuAjMuAjM/errno)，代表错误码。

通用错误码可参见 [NFC API 错误码](/document/uYjL24iN/uQzM4YjL0MDO24CNzgjN/nfc-error-codes)
