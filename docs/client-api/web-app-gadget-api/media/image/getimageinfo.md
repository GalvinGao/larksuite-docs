---
document_id: '6965379543683317766'
directory_id: '6907567266541977602'
title: getImageInfo
full_path: /uYjL24iN/ugjNwEjL4YDMx4CO2ATM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Image
- getImageInfo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:21Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugjNwEjL4YDMx4CO2ATM
---

# getImageInfo(Object object)

获取图片信息。



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
      <md-td><md-version>V2.4.0+</md-version></md-td>
      <md-td><md-version>V2.4.0+</md-version></md-td>
      <md-td><md-version>V2.4.0+</md-version></md-td>
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
                src
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                图片的路径，支持网络路径和小程序文件系统（用户目录、临时目录、包目录）下的路径，参考 [getFileSystemManager](/document/uYjL24iN/uETOuETOuETO/tt_get_file_system_manager)

**示例值**：

https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png
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
                width
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                图片原始宽度，单位 px。不考虑旋转。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                height
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                图片原始高度，单位 px。不考虑旋转。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                path
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                图片的本地路径。
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getImageInfo({
  src: "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png",
  success(res) {
    console.log(res);
  },
  fail (res) {
    console.log(`getImageInfo fail`);
  }
})
```

`success`返回对象示例：

```json
{
  "width": 1640,
  "height": 1296,
  "errMsg": "getImageInfo:ok",
  "path": "ttfile://temp/1637490835010.png"
}
``` 
