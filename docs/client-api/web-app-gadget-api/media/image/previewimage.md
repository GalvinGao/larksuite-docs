---
document_id: '6965379567070543877'
directory_id: '6907567266541977602'
title: previewImage
full_path: /uYjL24iN/uMDOx4yM4EjLzgTM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Image
- previewImage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDOx4yM4EjLzgTM
---

# previewImage(Object object)


预览一组图片。


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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td>
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
                urls
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                图片地址列表，支持本地和网络url
              
**示例值**：
              
["https: //sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/05b68b58ca78f4d3de5aa4a881f3cf2b.png","https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png"]
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
            <md-td></md-td>
            <md-td>
                请求 Header，仅为网络url时有效。

**示例值**：
              
{"csrf-token": "1234"}

<md-alert type="tip" icon="none">
- PC 端：Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- Android/iOS 端：Lark[V3.3.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持

</md-alert> 
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                current
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                urls[0]的内容
            </md-td>
            <md-td>
                默认显示的图片的地址

**示例值**：

https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/05b68b58ca78f4d3de5aa4a881f3cf2b.png
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.previewImage({
    urls: [
        "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/05b68b58ca78f4d3de5aa4a881f3cf2b.png",
        "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png"
    ],
    current: "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`previewImage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "previewImage:ok"
}
```
