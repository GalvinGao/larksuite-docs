---
document_id: '6967240092650356742'
directory_id: '6907567266540847106'
title: JSAPI总览
full_path: /uYjL24iN/uMTMuMTMuMTM/
breadcrumb:
- Client API
- Web app/Gadget API
- JSAPI Overview
document_type: GuideDocumentType
updated_at: 2021-06-07T13:29:47Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTMuMTMuMTM/
---

# JSAPI总览
##  API列表
H5应用开发推荐使用如下接口。
### 设备
#### 剪贴板
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|--------|---|--------|
|[getClipboardData](/document/uYjL24iN/uczNx4yN3EjL3cTM)|获取系统剪贴板内容|支持|3.44|支持|3.44|支持|3.44|
|[setClipboardData](/document/uYjL24iN/ugzNx4CO3EjL4cTM)|设置系统剪贴板内容|支持|3.44|支持|3.44|支持|3.44|

#### 系统信息
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|---|--------|---|--------|
|[getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM)|获取系统信息|支持|3.43|支持|3.43|支持|3.47|
|[getSystemInfoSync](/document/uYjL24iN/uUjNx4SN2EjL1YTM)|获取系统信息（同步）|支持|3.43|支持|3.43|支持|3.47|

#### 网络状态
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[getNetworkType](/document/uYjL24iN/uYjNx4iN2EjL2YTM)|获取设备当前所处的网络类型|支持|3.44|支持|3.44|支持|3.47|

#### Wi-Fi
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[getConnectedWifi](/document/uYjL24iN/ugjNx4CO2EjL4YTM)|获取设备当前所连的 Wifi|支持|3.44|支持|3.44|支持|3.47|
|[getWifiStatus](/document/uYjL24iN/uYTN4QjL2UDO04iN1gDN)|请求获取 Wi-Fi 开关状态|支持|3.44|支持|3.44|不支持||
|[onGetWifiList](/document/uYjL24iN/uYDO4UjL2gDO14iN4gTN)|监听获取到 Wi-Fi 列表数据事件|支持|3.44|支持|3.44|不支持||
|[offGetWifiList](/document/uYjL24iN/ucDO4UjL3gDO14yN4gTN)|取消监听获取到 Wi-Fi 列表数据事件|支持|3.44|支持|3.44|不支持||

#### 加速度计
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[startAccelerometer](/document/uYjL24iN/ukjNx4SO2EjL5YTM)|通知客户端开始监听加速度计数据 Wifi|支持|3.44|支持|3.44|不支持||
|[stopAccelerometer](/document/uYjL24iN/uAzNx4CM3EjLwcTM)|停止监听加速度计数据 Wifi|支持|3.44|支持|3.44|不支持||
|[onAccelerometerChange](/document/uYjL24iN/uEzNx4SM3EjLxcTM)|监听加速度计数据 Wifi|支持|3.44|支持|3.44|不支持||

#### 屏幕亮度
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[getScreenBrightness](/document/uYjL24iN/uIjNx4iM2EjLyYTM/get-screen-brightness)|获取屏幕亮度|支持|3.44|支持|3.44|不支持||
|[setScreenBrightness](/document/uYjL24iN/uIjNx4iM2EjLyYTM/set-screen-brightness)|设置屏幕亮度|支持|3.44|支持|3.44|不支持||
|[setKeepScreenOn](/document/uYjL24iN/ukzNx4SO3EjL5cTM)|设置是否保持常亮状态|支持|3.44|支持|3.44|不支持||

#### 截屏监听
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[onUserCaptureScreen](/document/uYjL24iN/uMjNwEjLzYDMx4yM2ATM)|监听用户主动截屏事件|支持|3.44|支持|3.44|不支持||
|[offUserCaptureScreen](/document/uYjL24iN/uQjNwEjL0YDMx4CN2ATM)|取消监听用户主动截屏事件|支持|3.44|支持|3.44|不支持||


#### 罗盘
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[startCompass](/document/uYjL24iN/uIzNx4iM3EjLycTM)|开始监听罗盘数据|支持|3.44|支持|3.44|不支持||
|[stopCompass](/document/uYjL24iN/uMzNx4yM3EjLzcTM)|停止监听罗盘数据|支持|3.44|支持|3.44|不支持||


#### 拨打电话
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[makePhoneCall](/document/uYjL24iN/uUzNx4SN3EjL1cTM)|拨打电话|支持|3.44|支持|3.44|不支持||

#### 震动
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[vibrateLong](/document/uYjL24iN/uEDOx4SM4EjLxgTM)|使手机发生较长时间的振动|支持|3.44|支持|3.44|不支持||
|[vibrateShort](/document/uYjL24iN/uADOx4CM4EjLwgTM)|使手机发生较短时间的振动|支持|3.44|支持|3.44|不支持||

#### 扫码
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[scanCode](/document/uYjL24iN/uYzNx4iN3EjL2cTM)|扫描二维码并返回扫描结果|支持|3.44|支持|3.44|不支持|




#### NFC
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[nfcConnect](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|连接NfcA类型的标签|支持|3.44|不支持||不支持||
|[nfcClose](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|断开与NfcA标签之间的连接|支持|3.44|不支持||不支持||
|[nfcGetSak](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|获取SAK信息|支持|3.44|不支持||不支持||
|[nfcGetAtqa](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|获取ATQA信息|支持|3.44|不支持||不支持||
|[nfcMaxTransceiveLength](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|获取最大传输长度|支持|3.44|不支持||不支持||
|[nfcTransceive](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|发送数据给NFCA类型的标签|支持|3.44|不支持||不支持||
|[nfcSetTimeout](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|设置超时时间|支持|3.44|不支持||不支持||
|[nfcStartDiscovery](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|开始扫描NFC标签|支持|3.44|不支持||不支持||
|[nfcStopDiscovery](/document/uYjL24iN/ugTN4YjL4UDO24CO1gjN)|关闭NFC标签扫描|支持|3.44|不支持||不支持||


### 媒体

#### 图片
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[chooseImage](/document/uYjL24iN/uMTMx4yMxEjLzETM)|从系统相册中选择图片，或使用相机拍摄图片|支持|3.44|支持|3.44|支持|3.47|
|[getImageInfo](/document/uYjL24iN/ugjNwEjL4YDMx4CO2ATM)|获取图片信息|支持|3.44|支持|3.44|支持|3.47|
|[compressImage](/document/uYjL24iN/uMjN24yM2YjLzYjN)|压缩图片接口|支持|3.44|支持|3.44|支持|3.47|
|[saveImageToPhotosAlbum](/document/uYjL24iN/uUTMx4SNxEjL1ETM)|保存图片到系统相册|支持|3.44|支持|3.44|支持|3.47|

#### 视频
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[chooseVideo](/document/uYjL24iN/uEjMx4SMyEjLxITM)|从系统相册中选择视频，或使用相机拍摄视频|支持|3.44|支持|3.44|支持|3.47|
|[saveVideoToPhotosAlbum](/document/uYjL24iN/ucDOx4yN4EjL3gTM)|保存视频到系统相册|支持|3.44|支持|3.44|支持|3.47|

### 开放接口

#### 用户信息
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[getUserInfo](/document/uYjL24iN/ucjMx4yNyEjL3ITM)|获取已登录用户的基本信息或特殊信息|支持|3.44|支持|3.44|支持|3.44|

#### 授权
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[authorize](/document/uYjL24iN/ugzMx4COzEjL4MTM)|向用户发出授权请求|支持|3.44|支持|3.44|支持|3.44|

#### 邮件
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|---------|---|--------|
|[mailto](/document/uYjL24iN/uAjNwEjLwYDMx4CM2ATM)|调用系统发送邮件|支持|3.44|支持|3.44|不支持||

#### 系统认证
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|---------|---|--------|
|[startDeviceCredential](/document/uYjL24iN/uIDN14iM0UjLyQTN)|打开系统解锁界面|支持|3.44|支持|3.44|不支持||

#### 水印
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[checkWatermark](/document/uYjL24iN/ukTM1EjL5ETNx4SOxUTM)|查看宿主是否显示了全局水印|支持|3.44|支持|3.44|支持|3.47|

#### 聊天
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[getChatInfo](/document/uYjL24iN/uEDN2UjLxQjN14SM0YTN)|获取某个会话的信息|支持|3.44|支持|3.44|支持|3.44|
|[chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN)|打开用户会话列表选择会话|支持|3.44|支持|3.44|支持|3.44|

#### 设置
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|---------|---|--------|
|[getSetting](/document/uYjL24iN/uQzMx4CNzEjL0MTM)|获取用户已经授权过的配置|支持|3.44|支持|3.44|支持|3.44|
|[openSetting](/document/uYjL24iN/uUzMx4SNzEjL1MTM)|打开设置页面|支持|3.44|支持|3.44|支持|3.44|

#### 导航
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[openSchema](/document/uYjL24iN/ukzN4IjL5cDOy4SO3gjM)|跳转到小程序以外的应用|支持|3.44|支持|3.44|支持|3.47|

#### 文件
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[docsPicker](/document/uYjL24iN/ukTN3UjL5UzN14SO1cTN)|打开云文档选择列表|支持|3.44|支持|3.44|支持|3.47|
|[filePicker](/document/uYjL24iN/uETM04SMxQjLxEDN)|打开附件选择列表|支持|3.44|支持|3.44|支持|3.47|
|[saveFile](/document/uYjL24iN/ugDOz4CO4MjL4gzM)|保存临时文件到本地永久目录|支持|3.44|支持|3.44|支持|3.47|
|[openDocument](/document/uYjL24iN/ukTN24SO1YjL5UjN)|在新页面打开文档|支持|3.44|支持|3.44|支持|3.44|

### 界面
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
-----|--------|----------|--------|--------|--------|---|--------|
|[hideToast](/document/uYjL24iN/ukzMy4SOzIjL5MjM)|隐藏灰色背景的消息提示框|支持|3.44|支持|3.44|支持|3.47|
|[showToast](/document/uYjL24iN/ugzMy4COzIjL4MjM)|显示灰色背景的消息提示框|支持|3.44|支持|3.44|支持|3.47|
|[showActionSheet](/document/uYjL24iN/ukDNy4SO0IjL5QjM)|显示操作菜单|支持|3.44|支持|3.44|不支持|
|[showModal](/document/uYjL24iN/ugDNy4CO0IjL4QjM)|显示模态弹窗|支持|3.44|支持|3.44|支持|3.47|
|[showPrompt](/document/uYjL24iN/uYTO4UjL2kDO14iN5gTN)|展示可输入内容的弹窗|支持|3.44|支持|3.44|支持|3.47|


### 地理位置
|JSAPI|接口说明|Android|支持版本|iOS|支持版本|PC|支持版本|
|--------|--------|--------|----------|--------|--------|--------|---|--------|
|[getLocation](/document/uYjL24iN/uUTOz4SN5MjL1kzM)|获取设备当前的地理位置|支持|3.44|支持|3.44|不支持|
|[chooseLocation](/document/uYjL24iN/uUDN1EjL1QTNx4SN0UTM)|打开地图选择位置|支持|3.44|支持|3.44|不支持|
|[openLocation](/document/uYjL24iN/uQTOz4CN5MjL0kzM)|使用客户端内置地图查看位置|支持|3.44|支持|3.44|不支持|

##  bizAPI列表（旧版）
声明：biz版接口后续将不再维护。

| JSAPI         | 接口说明           |
| --------- | --------------- |
|[biz.navigation.close](/document/uYjL24iN/ugjN1EjL4YTNx4CO2UTM) | 设置关闭页面的回调 | 
|[biz.util.previewImage](/document/uYjL24iN/ukDN4IjL5QDOy4SO0gjM) | 调起预览图片组件 | 
|[biz.util.openLink](/document/uYjL24iN/uczN1EjL3cTNx4yN3UTM) | 打开链接 | 
|[biz.util.copyText](/document/uYjL24iN/uMDN4IjLzQDOy4yM0gjM) | 复制文本 | 
|[biz.util.share](/document/uYjL24iN/uUzN1EjL1cTNx4SN3UTM) | 分享 | 
|[biz.util.scan](/document/uYjL24iN/uYzN1EjL2cTNx4iN3UTM) | 扫码 |  
|[device.geolocation.get](/document/uYjL24iN/ukzM4IjL5MDOy4SOzgjM) | 获取定位信息 | 
|[device.geolocation.start](/document/uYjL24iN/uADN4IjLwQDOy4CM0gjM) | 开始持续定位 | 
|[device.geolocation.stop](/document/uYjL24iN/uEDN4IjLxQDOy4SM0gjM) | 停止持续定位 | 
|[device.notification.showPreloader](/document/uYjL24iN/uQDO1EjL0gTNx4CN4UTM) | 展示 ProgressHUD | 
|[device.notification.hidePreloader](/document/uYjL24iN/uUDO1EjL1gTNx4SN4UTM) | 隐藏 ProgressHUD | 
|[device.notification.confirm](/document/uYjL24iN/uADO1EjLwgTNx4CM4UTM) | confirm 窗口 | 
|[device.notification.alert](/document/uYjL24iN/ukzN1EjL5cTNx4SO3UTM) | alert 弹窗 | 
|[device.notification.toast](/document/uYjL24iN/uMDO1EjLzgTNx4yM4UTM) | toast 弹窗 | 
|[device.notification.prompt](/document/uYjL24iN/uEDO1EjLxgTNx4SM4UTM) | prompt 窗口 | 
|[device.notification.vibrate](/document/uYjL24iN/uIDO1EjLygTNx4iM4UTM) | 震动 | 
|[device.base.onUserCaptureScreen](/document/uYjL24iN/uQDN4IjL0QDOy4CN0gjM) | 注册截屏事件 |
|[device.base.offUserCaptureScreen](/document/uYjL24iN/uUDN4IjL1QDOy4SN0gjM) | 注销监听截屏事件 | 
|[device.screen.lockViewOrientation](/document/uYjL24iN/uYDN4IjL2QDOy4iN0gjM) | 锁定当前屏幕视图到指定状态（横屏/竖屏） |
|[device.screen.unlockViewOrientation](/document/uYjL24iN/ucDN4IjL3QDOy4yN0gjM) | 解除调用接口 lockViewOrientation 时对屏幕视图产生的锁定行为 | 
| [device.geolocation.get](/document/uYjL24iN/ukzM4IjL5MDOy4SOzgjM) |获取用户定位信息
| [device.geolocation.start](/document/uYjL24iN/uADN4IjLwQDOy4CM0gjM) | 开启连续定位
| [device.geolocation.stop](/document/uYjL24iN/uEDN4IjLxQDOy4SM0gjM) | 关闭连续定位

