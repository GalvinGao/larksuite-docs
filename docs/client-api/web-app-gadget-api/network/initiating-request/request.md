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

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/request/request" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| url | string | 是 |  | 请求地址<br>**示例值**：http://open.feishu.cn |
| header | object | 否 | {'content-type': 'application/json'} | 请求 Header<br>**示例值**：{'content-type': 'application/json'} |
| method | string | 否 | GET | 请求方法<br>**示例值**：GET<br>**可选值**：<br>- `GET`<br>- `POST`<br>- `PUT`<br>- `HEAD`<br>- `DELETE` |
| data | string｜object｜arraybuffer | 否 |  | 请求数据<br>**示例值**：{"noncestr":123} |
| dataType | string | 否 | json | 请求数据类型<br>**示例值**：json |
| responseType | string | 否 | text | 响应数据类型，参数值可以是 text 或 arraybuffer<br>**示例值**：text<br>**可选值**：<br>- `text`<br>- `arraybuffer` |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| statusCode | number | 返回 HTTP 状态码 |
| header | object | 返回 HTTP Header |
| data | string｜object｜arraybuffer | 返回数据 |
| trace | string | 请求ID，用于请求问题的诊断和全链路追踪<br><md-alert type="tip" icon="none"><br>Lark[V4.7.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |


`fail`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| trace | string | 请求ID，用于请求问题的诊断和全链路追踪<br><md-alert type="tip" icon="none"><br>Lark[V4.7.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |


返回值：`RequestTask`，该对象的方法列表如下：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [abort()](/document/uYjL24iN/ugDNugDNugDN/requesttask/abort) | 中断请求任务 |


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

