---
document_id: '6965379543683678214'
directory_id: '7081934717348937733'
title: downloadFile
full_path: /uYjL24iN/ucDMx4yNwEjL3ATM
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- Download
- downloadFile
document_type: GuideDocumentType
updated_at: 2022-12-23T06:32:07Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucDMx4yNwEjL3ATM
---

# downloadFile(Object object)


下载网络文件到本地临时目录。

:::html
<md-alert type="tip">
注意事项：
- HTTP 请求 method 为GET
- tt.downloadFile的最大并发限制是 5 个
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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/download-file/download-file" fontSize="14">预览</md-preview-app></md-td>
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
                文件地址

**示例值**：https://open.lark.com/a.txt
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
                是
            </md-td>
            <md-td>
                {}
            </md-td>
            <md-td>
                请求 Header

**示例值**：{\"Content-Type\": \"application/json\"}
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
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                指定文件下载后存储的路径 (本地路径)。格式为 ttfile://temp/lark.png 或 ttfile://user/lark.png，其中 ttfile://temp/ 和 ttfile://user/ 为下载目录，lark.png 为文件名。详情可参考[文件系统](/document/uYjL24iN/uETOuETOuETO/file-system)。
              默认下载目录为 ttfile://temp/

**示例值**：ttfile://temp/lark.png
<md-alert type="tip" icon="none">
Lark[V3.43.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
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
                tempFilePath
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                文件本地路径
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
                tempFilePath
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                文件本地路径
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

返回值：`downloadTask`，该对象的方法列表参见下表：
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
      <md-td>[onProgressUpdate](/document/uYjL24iN/ugDNugDNugDN/downloadfile/onprogressupdate)</md-td>
      <md-td>监听下载进度</md-td>
    </md-tr>

    <md-tr>
      <md-td>[abort](/document/uYjL24iN/ugDNugDNugDN/downloadfile/abort)</md-td>
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/download-file/download-file" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.downloadFile({
    "url": "https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fimg.jj20.com%2Fup%2Fallimg%2Ftp05%2F19100120461512E-0-lp.jpg&refer=http%3A%2F%2Fimg.jj20.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=jpeg?sec=1640079653&t=22aafb14cb145c11fc833022d61507c5",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`downloadFile fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "trace": "021637487932728d06e52699ed7c796bac157dc5fca6dde1vbbbb",
    "tempFilePath": "ttfile://temp/src=http___img.jj20.com_up_allimg_tp05_19100120461512E-0-lp.jpg&refer=http___img.jj20.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=jpeg",
    "statusCode": 200,
    "errMsg": "downloadFile:ok"
}
``` 

`fail`返回对象示例：
```json
{
    "errMsg": "downloadFile:fail 不支持的URL",
    "tempFilePath": "",
    "statusCode": "0"
}
``` 
