---
document_id: '6965379541104377861'
directory_id: '6907567266540503042'
title: getSystemInfo
full_path: /uYjL24iN/uQjNx4CN2EjL0YTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- System Information
- getSystemInfo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:44Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQjNx4CN2EjL0YTM
---

# getSystemInfo(Object object)
获取系统信息。



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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-system-info/get-system-info" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.43.0+</md-version></md-td>
      <md-td><md-version>V3.43.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


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
                system
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                操作系统版本
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                platform
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                操作系统类型。(darwin代表Mac，windows_nt代表windows）
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                brand
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                手机品牌。PC端值为`PC`
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                model
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                手机型号。PC端均为`PC`
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                version
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                LarkApp版本号
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                SDKVersion
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                客户端基础库版本
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                screenWidth
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                屏幕宽度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                screenHeight
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                屏幕高度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                windowWidth
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                可使用窗口的宽度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                windowHeight
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                可使用窗口高度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                pixelRatio
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                设备像素比
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                statusBarHeight
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                状态栏高度
<md-alert type="tip" icon="none">
PC端：网页应用不支持
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                language
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                Lark设置的语言
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                fontSizeSetting
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                用户字体大小
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                appName
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                宿主app名字
<md-alert type="tip" icon="none">
Lark[V3.5.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                safeArea
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                在竖屏正方向下的安全区域
<md-alert type="tip" icon="none">
PC端：网页应用不支持
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    left
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                安全区域左上角的横坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    right
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                安全区域右上角的横坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    top
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                安全区域左上角的纵坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    bottom
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                安全区域左下角的纵坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    width
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                安全区域的宽度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    height
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                安全区域的高度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                navigationBarSafeArea
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                导航栏的安全区域坐标，当页面配置了自定义导航时返回，方便应用在该区域内进行自定义元素的布局
<md-alert type="tip" icon="none">
- 需要在页面渲染完成后再调用, 否则可能无法获取正确的值
- Android/iOS 端：Lark[V3.33.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC端
  - 小程序：Lark[V3.35.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
  - 网页应用：不支持
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    left
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                导航栏的安全区域左上角横坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    right
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                导航栏的安全区域右上角的横坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    top
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                导航栏的安全区域左上角的纵坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    bottom
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                导航栏的安全区域右下角的纵坐标
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    width
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                导航栏的安全区域宽度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    height
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                导航栏的安全区域高度
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                theme
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                当前系统主题，在[小程序支持 DarkMode](/document/uYjL24iN/uQTM5UjL0ETO14CNxkTN/darkmode)的时候才会返回

**示例值**：light

**可选值**：
- `light`：浅色主题
- `dark`：深色主题
<md-alert type="tip" icon="none">
- Lark [V5.3.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持
- 网页应用：不支持
</md-alert>
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-system-info/get-system-info" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getSystemInfo({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getSystemInfo fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
  "errMsg": "getSystemInfo:ok",
  "system": "11.4.0",
  "platform": "darwin",
  "appName": "Lark",
  "version": "5.1.0",
  "language": "zh_CN",
  "SDKVersion": "1.9.56",
  "screenWidth": 1322,
  "screenHeight": 913,
  "windowWidth": 1322,
  "windowHeight": 913,
  "pixelRatio": 2,
  "statusBarHeight": 0,
  "safeArea": {
    "left": 0,
    "right": 1322,
    "top": 0,
    "bottom": 913,
    "width": 1322,
    "height": 913
  },
  "navigationBarSafeArea": {
    "left": 0,
    "right": 1268,
    "top": 0,
    "bottom": 36,
    "width": 1268,
    "height": 36
  },
  "brand": "PC",
  "model": "PC",
  "fontSizeSetting": 12,
  "theme": "light"
}
``` 

## 已知问题
- 有少量`iOS`设备的`model`字段返回如`iPod7,1`等值。
- 如需使用 `safeArea` 和 `navigationBarSafeArea` 字段，需要在页面渲染完成后使用该 API 进行获取


## 注释
### model
iOS 设备在 iPhone12（包含）后续更新机型，model字段会返回系统机型标识，iPhone12 之前的系列机型会显示系统机型标识的映射名字，具体设备及返回结果参考下表：
```json
{
    "iPhone 2G":"iPhone 2G",
    "iPhone 3G":"iPhone 3G",
    "iPhone 3GS":"iPhone 3GS",
    "iPhone 4":"iPhone 4",
    "iPhone 4S":"iPhone 4S",
    "iPhone 5":"iPhone 5",
    "iPhone 5c":"iPhone 5c",
    "iPhone 5s":"iPhone 5s",
    "iPhone 6 Plus":"iPhone 6 Plus",
    "iPhone 6":"iPhone 6",
    "iPhone 6s":"iPhone 6s",
    "iPhone 6s Plus":"iPhone 6s Plus",
    "iPhone SE":"iPhone SE",
    "iPhone 7":"iPhone 7",
    "iPhone 7 Plus":"iPhone 7 Plus",
    "iPhone 8":"iPhone 8",
    "iPhone 8 Plus":"iPhone 8 Plus",
    "iPhone X":"iPhone X",
    "iPhone XS":"iPhone XS",
    "iPhone XS Max":"iPhone XS Max",
    "iPhone XR":"iPhone XR",
    "iPhone 11":"iPhone 11",
    "iPhone 11 Pro":"iPhone 11 Pro",
    "iPhone 11 Pro Max":"iPhone 11 Pro Max",
    "iPhone SE2" : "iPhone12,8",
    "iPhone 12 mini":"iPhone13,1",
    "iPhone 12": "iPhone13,2",
    "iPhone 12 Pro":"iPhone13,3",
    "iPhone 12 Pro Max":"iPhone13,4"
}
``` 
