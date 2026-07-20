---
document_id: '6965379541104836613'
directory_id: '6907567266536456193'
title: request
full_path: /uYjL24iN/uIDMx4iMwEjLyATM
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- initiating Request
- request
document_type: GuideDocumentType
updated_at: 2024-08-05T03:36:55Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDMx4iMwEjLyATM
---

# request

发起一个 HTTP 请求。

:::html
<md-alert type="tip">
- header 的部分关键字应当遵守标准，如 Cookie，具体请参考 [HTTP Headers](https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Headers)（pc端不支持主动携带Cookie字段，平台会自动带上）。
- 以下header key作为小程序的保留header，如果使用，可能导致header内容的覆盖。
	- `x-request-id`
	- `x-request-id-op`
	- `x-tt-log-id`
- PC以及开发者工具上无法查看cookie，如需调试cookie请在移动端操作。
- tt.request的最大并发限制是10个。
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
      <md-td>**✓**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/request/request" fontSize="14">预览</md-preview-app>
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

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                url
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                请求地址

**示例值**：http://open.feishu.cn
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                header
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                {'content-type': 'application/json'}
            </md-td>
            <md-td>
                请求 Header

**示例值**：{'content-type': 'application/json'}
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                method
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                GET
            </md-td>
            <md-td>
                请求方法

**示例值**：GET

**可选值**：
- `GET`
- `POST`
- `PUT`
- `HEAD`
- `DELETE`
               
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                data
            </md-td>
            <md-td>
                string｜object｜arraybuffer
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                请求数据

**示例值**：{"noncestr":123}
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                dataType
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                json
            </md-td>
            <md-td>
                请求数据类型

**示例值**：json
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                responseType
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                text
            </md-td>
            <md-td>
                响应数据类型，参数值可以是 text 或 arraybuffer

**示例值**：text

**可选值**：
- `text`
- `arraybuffer`
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
                statusCode
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                返回 HTTP 状态码
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                header
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                返回 HTTP Header
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                data
            </md-td>
            <md-td>
                string｜object｜arraybuffer
            </md-td>
            <md-td>
                返回数据
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                trace
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                请求ID，用于请求问题的诊断和全链路追踪
<md-alert type="tip" icon="none">
Lark[V4.7.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

`fail`返回对象的扩展属性：
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
                trace
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                请求ID，用于请求问题的诊断和全链路追踪
<md-alert type="tip" icon="none">
Lark[V4.7.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

返回值：`RequestTask`，该对象的方法列表如下：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">方法</md-th>
      <md-th style="width: 50%;">介绍</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>[abort()](/document/uYjL24iN/ugDNugDNugDN/requesttask/abort)</md-td>
      <md-td>中断请求任务</md-td>
    </md-tr>
    
</md-tbody>
</md-table>
:::

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/request/request" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>

  </div>
</div> 
:::

```js
	const requestTask = tt.request({
    "url": "https://www.toutiao.com",
    "data": {
        "noncestr": Date.now()
    },
    "header": {
        "content-type": "application/json"
    },
    "method": "GET",
    "dataType": "json",
    "responseType": "text",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`request fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "statusCode": 200,
    "trace": "021637463063659f7c4bc2bf0cd6d8391a154dc31cf5e62bbbbbe",
    "header": {
        "status": "200",
        "server": "Tengine",
        "content-type": "text/html",
        "date": "Sun, 21 Nov 2021 02:51:02 GMT",
        "vary": "Accept-Encoding",
        "set-cookie": "__ac_nonce=06199b416006255ab57c1; Path=/; Max-Age=1800",
        "server-timing": "inner; dur=9,cdn-cache;desc=MISS,edge;dur=0,origin;dur=135",
        "x-tt-trace-host": "011d2aece4e92d84a85d7fcea0cc3f2c027b31de29296ecf91fdc235ecdc68b13696f60959c09c485e35660c121656222b2eae610b80d07f266a4c49a1c2611c57a24c2f641bc4e3f3534f19f29108ad6475941944ca55afeca5d20a723beb6b5c",
        "x-tt-trace-tag": "id=3;cdn-cache=miss",
        "x-tt-trace-id": "00-40677dfb0df306001cc1ab774f000489-40677dfb0df30600-01",
        "content-encoding": "identity",
        "via": "cache2.cn3641[135,0]",
        "timing-allow-origin": "*",
        "eagleid": "6f30441616374630628026648e",
        "x-net-info.remoteaddr": "111.48.68.224:443",
        "x-protocol": "CronetTcp+h2",
        "EENet-Request-Http-Channel": "rustChannel",
        "EENet-Request-Server-Ip": "",
        "EENet-Request-Dns-Cost": "0",
        "EENet-Request-Tls-Cost": "0",
        "EENet-Request-Tcp-Cost": "0",
        "EENet-XRequest-Id": "112",
        "request-id": ""
    },
    "data": "<html><head><meta charset=\"UTF-8\" /></head><body></body><script src='https://sf1-ttcdn-tos.pstatp.com/obj/rc-web-sdk/acrawler.js'></script><script>function _f1(e,t){if(\"string\"!=typeof t)return;var o,n=e+\"=\",r=t.split(/[;&]/);for(var e=0;e<r.length;e++){for(o=r[e];\" \"===o.charAt(0);)o=o.substring(1,o.length);if(0===o.indexOf(n))return o.substring(n.length,o.length)}return\"\"}function _f2(e){return _f1(e,document.cookie)}function _f3(e,t,o){try{o&&(window.sessionStorage&&window.sessionStorage.setItem(e,t),window.localStorage&&window.localStorage.setItem(e,t));var n=31536e6;document.cookie=e+\"=; expires=Mon, 20 Sep 1970 00:00:00 UTC; path=/;\",document.cookie=e+\"=\"+t+\"; expires=\"+new Date((new Date).getTime()+n).toGMTString()+\"; path=/;\"}catch(e){}}window.byted_acrawler.init({aid:99999999,dfp:!0});var __ac_nonce=_f2(\"__ac_nonce\"),__ac_signature=window.byted_acrawler.sign(\"\",__ac_nonce);_f3(\"__ac_signature\",__ac_signature),_f3(\"__ac_referer\",document.referrer||\"__ac_blank\",!0);try{sessionStorage.setItem(\"__ac_ns\",performance.timing.navigationStart)}catch(e){};window.location.reload();</script></html>",
    "errMsg": "request:ok"
}
```
`fail`返回对象示例：

```json
{
    "errMsg": "request:fail CronetError(\"Cronet Error: code=1 / internal_code=-105 / None\")",
    "trace": "xxxxx"
}
``` 

## 已知问题

- `header` 不支持设置 `referer`

