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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/upload-file/upload-file" fontSize="14">预览</md-preview-app></md-td>
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
                目标地址
<md-alert type="tip" icon="none">
仅支持http | https协议
</md-alert>
            </md-td>
        </md-tr>
      
      <md-tr>
            <md-td>
                filePath
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                需要上传的本地文件路径
            </md-td>
        </md-tr>
      
       <md-tr>
            <md-td>
                name
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                HTTP 请求的文件名
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

            </md-td>
            <md-td>
                请求 header。header内 content-type 为 multipart/form-data，不可变更
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                formData
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                请求额外参数
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
                data
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                返回数据
            </md-td>
        </md-tr>
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
                errMsg
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                错误信息
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
                data
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                返回数据
            </md-td>
        </md-tr>
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
                errMsg
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
				错误消息
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

返回值：`uploadTask`，该对象的方法列表参见下表：
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
      <md-td>[onProgressUpdate(function callback)](/document/uYjL24iN/ugDNugDNugDN/uploadtask/onprogressupdate)</md-td>
      <md-td>监听上传进度</md-td>
    </md-tr>

    <md-tr>
      <md-td>[abort()]([uploadTask.abort](/document/uYjL24iN/ugDNugDNugDN/uploadtask/abort))</md-td>
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
