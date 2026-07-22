---
document_id: '6965379543683694598'
directory_id: '7081935736593661958'
title: uploadFile
full_path: /uYjL24iN/uYDMx4iNwEjL2ATM
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- Upload
- uploadFile
document_type: GuideDocumentType
updated_at: 2022-06-15T02:33:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYDMx4iNwEjL2ATM
---

# uploadFile(Object object)

将本地文件上传到网络。

:::html
<md-alert type="tip">
注意事项：
- HTTP 请求 method 为`POST`
- tt.uploadFile的最大并发限制是5个
- 传参字段的值为""、undefined、null会被视为空， 若为非必传参数会命中接口定义中的默认值；不符合接口入参类型（如定义 string 传入 object 或 number）会中断调用链路并执行fail回调。
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/upload-file/upload-file" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| url | string | 是 |  | 目标地址<br><md-alert type="tip" icon="none"><br>仅支持http &#124; https协议<br></md-alert> |
| filePath | string | 是 |  | 需要上传的本地文件路径 |
| name | string | 是 |  | HTTP 请求的文件名 |
| header | object | 否 |  | 请求 header。header内 content-type 为 multipart/form-data，不可变更 |
| formData | object | 否 |  | 请求额外参数 |


## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | string | 返回数据 |
| statusCode | number | 返回 HTTP 状态码 |
| errMsg | string | 错误信息 |
| trace | string | 请求ID，用于请求问题的诊断和全链路追踪 |


`fail`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | string | 返回数据 |
| statusCode | number | 返回 HTTP 状态码 |
| errMsg | string | 错误消息 |


返回值：`uploadTask`，该对象的方法列表参见下表：
:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [onProgressUpdate(function callback)](/document/uYjL24iN/ugDNugDNugDN/uploadtask/onprogressupdate) | 监听上传进度 |
| [abort()]([uploadTask.abort](/document/uYjL24iN/ugDNugDNugDN/uploadtask/abort)) | 中断请求任务 |


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/upload-file/download-file" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
  const uploadTask = tt.uploadFile({
    "url": "https://cloudapi.bytedance.net/faas/services/tt594x/invoke/imgupload",
    "filePath": filePath,
    "name": "test.jpeg",
    success (res) {
	  // upload Start
      console.log(res)
    },
    fail (res) {
	  // uploadFile Failed
      console.log(res)
    }
  })
```

`success`返回对象示例：

```json
{
    "statusCode": 200,
    "data": "{\"error\":\"xxxx\"}",
    "trace": "021638203108547b7613bd1ebe5586ea6f58e17a92f1544eebbbb",
    "errMsg": "uploadFile:ok"
}
``` 

`fail`返回对象示例：

```json
{
  "errMsg": "uploadFile:fail CronetError(\"Cronet Error: code=11 xxxxx")",
  "data": "",
  "statusCode": 0
}
``` 

## 已知问题

- `header` 不支持设置 `referer`
