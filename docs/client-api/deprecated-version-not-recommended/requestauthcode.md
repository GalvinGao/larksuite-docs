---
document_id: '7073692582769524742'
directory_id: '7345445193132867590'
title: requestAuthCode
full_path: /uYjL24iN/uUzMuUzMuUzM/20220308
breadcrumb:
- Client API
- Deprecated Version (Not Recommended)
- requestAuthCode
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:01Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUzMuUzMuUzM/20220308
---

# requestAuthCode(Object object)

requestAuthCode(Object object) 用于获取网页应用免登授权码，从而实现网页应用的用户免登流程。

:::note
- 关于网页应用免登流程的操作方法，可参见[步骤三：免登流程（可选）](/document/uYjL24iN/uMTMuMTMuMTM/development-guide/step-3)。
- 你无需进行[网页应用鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)即可调用该接口，但需要保证在 `window.h5sdk.ready` 的回调函数触发后调用该接口。
<br>
`window.h5sdk.ready` 由 H5 JSSDK 提供，用于 JSSDK 初始化完成后的回调。关于 JSSDK 的相关说明，可参见[开放接口](/document/uYjL24iN/uMTMuMTMuMTM/introduction#818efa2e)。
:::

## 支持说明

该接口目前仅支持在网页应用中调用，具体支持的客户端版本说明如下表所示。

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
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.1.0+</md-version></md-td>
      <md-td><md-version>V5.1.0+</md-version></md-td>
      <md-td><md-version>V5.1.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览				 </md-preview-app>
	  </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性如下所示。

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">
                名称
            </md-th>
            <md-th style="width: 15%;">
                数据类型
            </md-th>
            <md-th style="width: 15%;">
                是否必填
            </md-th>
            <md-th style="width: 15%;">
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
                appId
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>\-</md-td>
            <md-td>
应用的唯一标识（App ID），可在[开发者后台](https://open.larksuite.com/app)的指定应用详情页内，进入 **凭证与基础信息** 页面获取 **App ID**。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。
:::html

<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 20%;">
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
                code
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                免登授权码，有效期为 5 分钟，该授权码用于换取登录用户身份。
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码

调用示例：

```js
window.h5sdk.ready(() => { // ready 方法不需要每次都调用。
	tt.requestAuthCode({
	  appId: appId,
	  success: (info) => {
	    console.info(info.code)
	  },
	  fail: (error) => {
	    console.error(error)
	  }
	});
});
```

`success` 返回对象示例：
```json
{
    "code": "xxxxxxxx"
}
```

## 错误码

`fail` 返回对象中可能包含 errCode 属性和 errno 属性，均代表错误码。

**errCode 错误码**

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 35%;">错误码</md-th>
      <md-th style="width: 65%;">描述</md-th>

    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>10200</md-td>
      <md-td>appid 不合法。</md-td>
   </md-tr>
        <md-tr>
      <md-td>10207</md-td>
      <md-td>sessionid 为空。</md-td>
   </md-tr>    <md-tr>
      <md-td>10210</md-td>
      <md-td>sessionid 不合法。</md-td>
   </md-tr>    <md-tr>
      <md-td>10233</md-td>
      <md-td>获取 larksession 失败。</md-td>
   </md-tr>    <md-tr>
      <md-td>10227</md-td>
      <md-td>可见性检查失败。</md-td>
   </md-tr>    <md-tr>
      <md-td>10228</md-td>
      <md-td>应用对该用户不可见。</md-td>
   </md-tr>    <md-tr>
      <md-td>10232</md-td>
      <md-td>获取授权码失败。</md-td>
   </md-tr>    <md-tr>
      <md-td>10234</md-td>
      <md-td>获取开发者后台配置的重定向 URL 列表失败。</md-td>
   </md-tr>    <md-tr>
      <md-td>10235</md-td>
      <md-td>未在开发者后台配置重定向 URL。你可以在[开发者后台](https://open.larksuite.com/app)的指定应用详情页内，进入 **安全设置** 页面配置 **重定向 URL**。</md-td>
   </md-tr>    <md-tr>
      <md-td>10236</md-td>
      <md-td>URL 不合法（URL 为空，或不在重定向 URL 列表中）。</md-td>
    </md-tbody>
</md-table>
:::

接口**errCode**错误码相应的排查建议如下表所示。

:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width:15%">错误码</md-th>
<md-th style="width:25%">描述</md-th>
<md-th style="width:60%">排查建议</md-th>
</md-tr>
</md-thead>
<md-tbody>

<md-tr>
<md-td>10236</md-td>
<md-td>URL 不合法。

URL 为空，或没有在[开发者后台](https://open.larksuite.com/app)的应用详情页 **安全设置 > 重定向 URL** 列表中配置当前页面 URL。</md-td>
<md-td>
检查当前页面 URL 是否已经配置在[开发者后台](https://open.larksuite.com/app)的应用详情页 **安全设置 > 重定向 URL** 中。如果没配置，则需要配置重定向 URL 后再试。
如果配置 URL 后，依然出现此错误码，则是因为你调用 **requestAuthCode** 接口的页面，不在重定向 URL 列表中。建议检查当前页面 URL 的完整路径（不包括`?`和`#`后面的部分），是否已经配置在开发者后台的应用详情页 **安全设置 > 重定向 URL** 中，如果没配置，则需要配置重定向 URL 后再试。

- （建议）当前页面 URL 获取方式：在当前页面调用`window.location.href.split('?')[0].split('#')[0]`获取 URL。
	
	应用主页对应的 URL 完整路径是域名加斜杠，该路径可调用`window.location.href.split('?')[0].split('#')[0]`验证。例如，域名`https://open.larksuite.com`或者域名`http://127.0.0.1:3000`（IP 地址 + 端口号），对应的主页如果需要获取 Authorization Code，应该填入的 重定向 URL 分别为`https://open.larksuite.com/`、`http://127.0.0.1:3000/`。
	
- 重定向 URL 填写时，常见情况举例与说明如下图所示。


	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1f1d229f7fdad46c49383b6fb643cd7a_YqiOcuMWqa.png?height=448&lazyload=true&width=1588)

	
</md-td>
</md-tr>

<md-tr>
<md-td>10235</md-td>
<md-td>未在开发者后台配置重定向 URL。</md-td>
<md-td>检查当前页面 URL 是否已经配置在开发者后台中应用详情页 **安全设置 > 重定向 URL**。如果没有，请配置之后再试。</md-td>
</md-tr>

<md-tr>
<md-td>10200</md-td>
<md-td>appid 不合法。</md-td>
<md-td>请检查前端调用 requestAuthCode 时传入的 app_id，与你在开发者后台中应用详情页 **凭证与基础信息** 获得的 App ID 是否相同。如果不同，请保持相同之后再试。</md-td>
</md-tr>

<md-tr>
<md-td>10228</md-td>
<md-td>应用对该用户不可见。</md-td>
<md-td>在开发者后台中应用详情页 **版本管理与发布** 中找到状态为 **已发布** 的版本，查看版本详情，检查 **可用范围** 是否包括当前用户。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/61497d0a539dc2666eb9f721a2c7157c_Nhe1Rg90RM.png?height=1252&lazyload=true&width=1790)
  
</md-td>
</md-tr>

<md-tr>
<md-td>10232</md-td>
<md-td>获取免登临时授权码 Code 失败。</md-td>
<md-td>建议再次打开页面尝试调用。</md-td>
</md-tr>

<md-tr>
<md-td>10234</md-td>
<md-td>获取开发者后台的应用详情页 **安全设置 > 重定向 URL** 列表失败。</md-td>
<md-td>建议再次打开页面尝试调用。</md-td>
</md-tr>

<md-tr>
<md-td>10210</md-td>
<md-td>sessionid 不合法。</md-td>
<md-td>建议退出当前账号重新登录后尝试调用。</md-td>
</md-tr>

</md-tbody>
</md-table>
:::
  
**errno 错误码**
  
关于 Errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。
