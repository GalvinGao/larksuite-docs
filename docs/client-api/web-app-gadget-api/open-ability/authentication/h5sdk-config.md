---
document_id: '7346095924105543685'
directory_id: '7345691092930019333'
title: h5sdk.config
full_path: /uYjL24iN/uQjMuQjMuQjM/authentication/h5sdkconfig
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- authentication
- h5sdk.config
document_type: GuideDocumentType
updated_at: 2024-03-14T07:04:33Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQjMuQjMuQjM/authentication/h5sdkconfig
---

# h5sdk.config

用于对使用到的网页应用JSAPI进行鉴权，具体鉴权操作方法参见[步骤二：JSAPI 调用（可选）](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)。

## 支持说明

| 应用能力 | Android | iOS     | PC      | 预览效果 |
| ---- | ------- | ------- | ------- | ---- |
| 小程序  | **X**   | **X**   | **X**   | /    |
| 网页应用 | V5.1.0+ | V5.1.0+ | V5.1.0+ | /    |

## 输入

| 名称        | 数据类型   | 必填 | 描述                                                                              |
| --------- | ------ | -- | ------------------------------------------------------------------------------- |
| appId     | string | 是  | 应用的唯一标识，可在[开发者后台](https://open.larksuite.com/app?lang=zh-CN) > 应用详情页面 > 凭证与基础信息 内查看 |
| timestamp | long | 是  | 生成签名的时间戳                                                                        |
| nonceStr  | string | 是  | 随机字符串                                                                           |
| signature | string | 是  | JSAPI鉴权签名                                                                       |
| jsApiList | Array  | 是  | 需要调用的JSAPI列表                                                                    |
> 说明：appId、timestamp、nonceStr、signature 等鉴权参数来自接入方服务端，接入方服务端先获取 access_token，再获取jsapi_ticket，最后生成signature并返回鉴权参数。
## 输出

`onSuccess`返回对象的属性：

| 名称          | 数据类型   | 描述           |
| ----------- | ------ | ------------ |
| session_key | string | 会话密钥，表明鉴权通过。 | 

## 示例代码

```
window.h5sdk.config({
    appId: appId,
    timestamp: timestamp,
    nonceStr: noncestr,
    signature: signature,
    jsApiList: jsApiList,
    //成功回调
    onSuccess: (res) => {
        console.log(`config success: ${JSON.stringify(res)}`);
    },
    //失败回调
    onFail: (err) => {
        console.log(`config failed: ${JSON.stringify(err)}`);
    },
});
```

## 错误码
`onFail` 返回对象中可能包含 errCode 属性和 errno 属性，均代表错误码。

**errCode 错误码**

| 错误码      | 描述                                 |
| -------- | ---------------------------------- |
| 1012     | 参数类型错误                             |
| 10001    | 网络请求失败                             |
| 333441   | 签名错误                               |
| 333448   | 页面不在安全域名内                          |
| 333449   | 应用不可见                              |
| 1014     | 网络异常错误                             |
| 10002    | 网络请求返回数据格式错误                       |
| 333430   | userId或者appId不合法                   |
| 333440   | app不存在                             |
| 333442   | app没找到有效的jsapi_ticket              |
| 333443   | 签名重复                               |
| 333444   | 签名过期                               |
| 333445   | jsapi未授权                           |
| 333446   | jsapi不存在                           |
| 333447   | 安全域名未设置                            |
| 9999169x | invalid session 用户登录态校验失败，x=[1-4]

**errno 错误码**
  
关于 errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。
