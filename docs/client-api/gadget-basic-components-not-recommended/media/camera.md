---
document_id: '7163183770995326981'
directory_id: '6907567266541862914'
title: camera
full_path: /uYjL24iN/uYTNuYTNuYTN/camera
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Media
- camera
document_type: GuideDocumentType
updated_at: 2022-12-23T07:01:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYTNuYTNuYTN/camera
---

# camera

相机
:::html
<md-alert type="tip">
相关API: [`tt.createCameraContext`](/document/uYjL24iN/ukDOukDOukDO/camera/createcameracontext)
 
调用前需要用户授权 `scope.camera`。了解如何授权，可查看[API 权限](/document/uYjL24iN/uITMuITMuITM)。

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
      <md-td><md-version>V5.21.0+</md-version></md-td>
      <md-td><md-version>V5.21.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/camera/camera" fontSize="14">预览</md-preview-app></md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

## 属性说明
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 15%;">属性名</md-th>
      <md-th style="width: 15%;">类型</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th style="width: 10%;">是否必填</md-th>
      <md-th style="width: 10%;">是否可动态修改</md-th>
      <md-th>描述</md-th>
      <md-th>最低版本</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>id</md-td>
      <md-td>string</md-td>
      <md-td></md-td>
      <md-td>是</md-td>
      <md-td>否</md-td>
      <md-td>
        组件标识符，当前页面内必须唯一
      </md-td>
      <md-td>5.21.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>resolution</md-td>
      <md-td>Enum&lt;string&gt;</md-td>
      <md-td>medium</md-td>
      <md-td>否</md-td>
      <md-td>否</md-td>
      <md-td>
        分辨率
      </md-td>
      <md-td>5.21.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>device-position</md-td>
      <md-td>Enum&lt;string&gt;</md-td>
      <md-td>back</md-td>
      <md-td>否</md-td>
      <md-td>是</md-td>
      <md-td>
        摄像头朝向
      </md-td>
      <md-td>5.21.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>flash</md-td>
      <md-td>Enum&lt;string&gt;</md-td>
      <md-td>auto</md-td>
      <md-td>否</md-td>
      <md-td>是</md-td>
      <md-td>
        闪光灯
      </md-td>
      <md-td>5.21.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>mode</md-td>
      <md-td>Enum&lt;string&gt;</md-td>
      <md-td>normal</md-td>
      <md-td>否</md-td>
      <md-td>否</md-td>
      <md-td>
        应用模式
      </md-td>
      <md-td>5.27.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>scan-code-type</md-td>
      <md-td>Enum&lt;string&gt;</md-td>
      <md-td>continuous</md-td>
      <md-td>否</md-td>
      <md-td>否</md-td>
      <md-td>
        扫码回调方式
      </md-td>
      <md-td>5.27.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>bindstop</md-td>
      <md-td>EventHandler</md-td>
      <md-td></md-td>
      <md-td>否</md-td>
      <md-td>是</md-td>
      <md-td>
        摄像头在非正常终止时触发，如退出后台等情况
      </md-td>
      <md-td>5.21.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>binderror</md-td>
      <md-td>EventHandler</md-td>
      <md-td></md-td>
      <md-td>否</md-td>
      <md-td>是</md-td>
      <md-td>
        用户不允许使用摄像头、相机创建失败等情况触发
      </md-td>
      <md-td>5.21.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>bindinitdone</md-td>
      <md-td>EventHandler</md-td>
      <md-td></md-td>
      <md-td>否</md-td>
      <md-td>是</md-td>
      <md-td>
- 相机初始化完成时触发，`e.detail = {maxZoom, devicePosition}`
- 从stop状态回到可用状态时重新触发回调。
- 切换摄像头时，关闭上一个摄像头并开启下一个摄像头。因此会先调用bindstop方法，再调用bindinitdone方法。
      </md-td>
      <md-td>5.21.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>bindscancode</md-td>
      <md-td>EventHandler</md-td>
      <md-td></md-td>
      <md-td>否</md-td>
      <md-td>是</md-td>
      <md-td>
        在扫码识别成功时触发，仅在`mode="scanCode"`时生效
      </md-td>
      <md-td>5.27.0</md-td>
    </md-tr>
    <md-tr>
      <md-td>bindlumadetect</md-td>
      <md-td>EventHandler</md-td>
      <md-td></md-td>
      <md-td>否</md-td>
      <md-td>是</md-td>
      <md-td>
        暗光检测回调，仅在`mode="scanCode"`时生效
      </md-td>
      <md-td>5.27.0</md-td>
    </md-tr>

  </md-tbody>
</md-table>
:::

## Bug & Tip
1. `tip`: 同一页面只能插入一个`camera`组件
2. `tip`: 请注意原生组件使用限制
3. `tip`: 受自研浏览器内核覆盖率的限制，camera 组件在层级上不保证一定为同层渲染，是否支持同层渲染可参考 `bindinserted` 回调说明。在非同层渲染场景下，开发者需对界面做相应适配，避免在 `camera` 组件上方覆盖其他组件

## ScanCode Tip
当`mode`为`scanCode`时：
1. `tip`: `flash`为 `auto`/`off`表示关闭手电筒，`on`/`torch`表示打开手电筒
2. `tip`: `device-position`不支持切换，仅支持`back`
3. `tip`: `takePhoto`, `startRecord`, `stopRecord`均不可用

  
## 参数说明

### resolution 合法值
|值|说明|最低版本
|--|----|--
|low|低|5.21.0
|medium|中|5.21.0
|high|高|5.21.0
### device-position 合法值

|值|说明|最低版本
|--|----|--
|front|前置|5.21.0
|back|后置|5.21.0
### flash 合法值

|值|说明|最低版本
|--|--|--
|auto|自动（仅 iOS 支持）|5.21.0
|on|打开|5.21.0
|off|关闭|5.21.0
|torch|常亮|5.21.0
### mode 合法值

|值|说明|最低版本
|--|----|--
|normal|相机模式|5.21.0
|scanCode|扫码模式|5.27.0
### scan-code-type 合法值

|值|说明|最低版本
|--|--------|--
|continuous|连续回调，只要画面内有可扫的码，bindscancode就会持续回调|5.27.0
|single|触发式回调，只有当前后扫的码不同时，bindscancode才会回调|5.27.0
### bindinitdone
- 相机初次初始化时调用
- 相机每次由stop恢复可用时调用
``` JavaScript
interface BindInitDoneDetail {
  maxZoom: number; // 指代最大变焦范围，相关接口见CameraContext.setZoom
  devicePosition: string; // 与 device-position 属性对应
}
```
### bindinserted
- 相机组件被插入到页面时调用
- 此回调当前用于告知是否开启同层渲染
```JavaScript
interface BindInsertedDetail {
  isRenderInSameLayer: boolean; // 当前是否开启同层渲染
}
```
### bindscancode
```JavaScript
enum ScanCodeType {
    QR_CODE = 'QR_CODE',
    VORTEX_CODE = 'VORTEX_CODE',
    UPC_A_CODE = 'UPC_A_CODE',
    UPC_E_CODE = 'UPC_E_CODE',
    EAN_8_CODE = 'EAN_8_CODE',
    EAN_13_CODE = 'EAN_13_CODE',
    CODE39_CODE = 'CODE39_CODE',
    CODE128_CODE = 'CODE128_CODE',
    DATA_MATRIX = 'DATA_MATRIX',
    PDF_417 = 'PDF_417'
}

interface BindScanCodeDetail {
    type?: ScanCodeType;
    result?: string;
}
```
### bindlumadetect
- 暗光检测回调，只在mode="scanCode"时生效
- 回调频率: 500ms每次
```JavaScript
interface BindLumaDetectDetail {
    scene: number; // 暗光返回0, 亮光返回1
}
```

## 代码示例
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/component/pages/camera/camera" fontSize="16" style="margin-right: 24px">扫码预览效果</md-preview-app>
  </div>
</div> 
:::



TTML文件
```html 
<scroll-view class="page-body">
  <view class="page-body-wrapper">
    <camera
    id="myCamera"
    binderror="binderror"
    bindstop="bindstop"
    bindinitdone="bindinitdone"
    bindinserted="bindinserted"
    style="width: 100%; height: 300px"/>
    
    <view class="btn-area">
      <button type="primary" bindtap="takePhoto">{{take_photo}}</button>
    </view>

    <view class="btn-area">
      <button type="primary" bindtap="startRecord">{{start_record}}</button>
    </view>
    <view class="btn-area">
      <button type="primary" bindtap="stopRecord">{{stop_record}}</button>
    </view>
    <view class="preview-tips" tt:if="{{src}}">{{preview_tips}}</view>
    <view class="btn-area" tt:if="{{src}}">
      <button type="primary" bindtap="previewImage">{{preview_image}}</button>
    </view>
    <video tt:if="{{videoSrc}}" class="video" src="{{videoSrc}}"/>

  </view>
</scroll-view>
``` 
JS文件
```javascript 
const camera = i18n.cameraComponent

Page({
  data: {
    ...camera,
    src: undefined,
    videoSrc: undefined,
    maxZoom: undefined,
    currentDevicePosition: undefined,
  },

  onReady: function () {
    this.ctx = tt.createCameraContext('myCamera');
  },
  takePhoto: function () {
    this.ctx.takePhoto({
      success: res => {
        console.log('takePhoto success', res);
        this.setData({
          src: res.tempImagePath
        });
      },
      fail: res => {
        console.log('takePhoto fail', res);
      }
    });
  },

  startRecord: function () {
    this.ctx.startRecord({
      timeoutCallback: res => {
        console.log('startRecord timeout', res);
        this.setData({
          src: res.tempThumbPath,
          videoSrc: res.tempVideoPath
        });
      },
      success: res => {
        console.log('startRecord success', res);
      },
      fail: res => {
        console.log('startRecord fail', res);
      }
    });
  },

  stopRecord: function () {
    this.ctx.stopRecord({
      compressed: this.data.stopRecordCompressed,
      success: res => {
        console.log('stopRecord success', res);
        this.setData({
          src: res.tempThumbPath,
          videoSrc: res.tempVideoPath
        });
      },
      fail: res => {
        console.log('stopRecord fail', res);
      }
    });
  },

  binderror: function (e) {
    console.log('binderror', e);
  },

  bindstop: function (e) {
    console.log('bindstop', e);
  },

  bindinitdone: function (e) {
    console.log('bindinitdone', e.detail);
    this.setData({
      maxZoom: e.detail.maxZoom,
      currentDevicePosition: e.detail.devicePosition,
    })
  },

  previewImage: function () {
    let image = this.data.src;
    tt.previewImage({
      urls: [image]
    })
  },
})

 
```
TTSS文件
```css
.preview-tips {
  margin: 20rpx 0;  
}

.video {
  margin: 50px auto;
  width: 100%;
  height: 300px;
}

.picker{
  padding: 19rpx 32rpx;
  background-color: #FFFFFF;
}
.section__title{
  padding: 48rpx 32rpx 8rpx 32rpx;
  font-size: medium;
  color: #999999;
}

.view-above {
  position: absolute;
  width: 100%;
  height: 40px;
  top: 40px;
}
.preview-tips {
  margin: 20rpx 0;  
}

.video {
  margin: 50px auto;
  width: 100%;
  height: 300px;
}
```
