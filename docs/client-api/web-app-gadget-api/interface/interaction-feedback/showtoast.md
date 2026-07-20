---
document_id: '6965379541104394245'
directory_id: '6907567266541240322'
title: showToast
full_path: /uYjL24iN/ugzMy4COzIjL4MjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Interaction Feedback
- showToast
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:17Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugzMy4COzIjL4MjM
---

# showToast(Object object)


显示灰色背景的消息提示。

:::html
<md-alert type="tip">
注意事项：
- 多次弹出 toast/loading 时，后一个会立刻覆盖前一个。
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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/toast/toast" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
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
                title
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                消息内容。



**示例值**：添加购物车成功
<md-alert type="tip" icon="none">
从3.39版本开始，在平台为移动端且配置显示图标的情况下title将只能显示至多7个字符，其他情况下可显示最多2行
- 当显示图标时，title 最多能够展示7个字符
- 当不显示图标时，title 最多可显示两行
</md-alert>
            </md-td>
        </md-tr>
              <md-tr>
            <md-td>
                duration
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                1500
            </md-td>
            <md-td>
                提示框停留的时间，单位ms
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                icon
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                success
            </md-td>
            <md-td>
                图标的类型


**可选值**：
- `success`：成功
- `loading`：加载中
- `none`：不显示图标（PC暂不支持）
- `error`：错误（仅PC端有效）
- `info`：提示（仅PC端有效）
- `warning`：警告（仅PC端有效）
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                mask
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                false                
            </md-td>
            <md-td>

是否显示透明蒙层，防止触摸穿透
<md-alert type="tip" icon="none">
- Android/iOS 端：Lark[V2.5.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC 端：暂不支持
</md-alert>

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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/toast/toast" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.showToast({
    "title": "添加购物车成功",
    "duration": 3000,
    "icon": "success",
    "mask": false,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showToast fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
  errMsg: "showToast:ok"
}
``` 

## 已知问题
- PC端暂不支持`mask`参数
- PC端暂不支持`icon`为`none`
