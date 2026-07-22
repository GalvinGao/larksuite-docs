---
document_id: '7444877287785054220'
directory_id: '6907567266540847106'
title: 小程序 API 总览（不推荐）
full_path: /uYjL24iN/ucjL34yN/gadget-api-list
breadcrumb:
- Client API
- Web app/Gadget API
- Gadget API list (Not Recommended)
document_type: GuideDocumentType
updated_at: 2025-02-20T06:18:33Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjL34yN/gadget-api-list
---

# 小程序 API 总览
::: warning
[小程序](/document/uYjL24iN/uUDNzUjL1QzM14SN0MTN)能力将不再迭代，推荐选择[网页应用](/document/uYjL24iN/uMTMuMTMuMTM/introduction)能力。<br>

从25年3月3日起，针对 **尚未** 开发过小程序应用的企业，将 **不允许** 创建小程序的入口。<br>

针对已开发过小程序能力的企业仍然可以创建小程序，不受影响。
:::

| Lark 扫码在线预览 | 示例代码 |
| --- | --- |
| ![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/acb2395e3e575b81f2352bd209525b10.png?height=155&lazyload=true&width=155) | - 下载示例代码：[microapp-demo.zip](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c73e19d3c910e185c0b76831e8773004_PP9jXkOARb.zip)<br>- 导入并调试示例代码：[使用说明](/document/uYjL24iN/uYDM04iNwQjL2ADN) |


## 开放接口
#### 登录

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [login](/document/uYjL24iN/uYzMuYzMuYzM) | 获取临时登录凭证 | **✓** | **✓** | **✓** |
| [checkSession](/document/uYjL24iN/ukTMx4SOxEjL5ETM) | 检查用户当前的 session 状态是否有效 | **✓** | **✓** | **✓** |

#### 用户信息

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [enterProfile](/document/uYjL24iN/ucDM04yNwQjL3ADN) | 打开个人信息主页 | **✓** | **✓** | **✓** |
| [getUserInfo](/document/uYjL24iN/ucjMx4yNyEjL3ITM) | 获取已登录用户的基本信息或特殊信息 | **✓** | **✓** | **✓** |

#### 聊天

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [enterChat](/document/uYjL24iN/ukDM04SOwQjL5ADN) | 打开指定会话 | **✓** | **✓** | **✓** |
| [toggleChat](/document/uYjL24iN/ugDM04COwQjL4ADN/toggleChat) | 侧边栏形式打开或关闭会话 | **X** | **X** | **✓** |
| [chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN) | 打开用户会话列表选择会话，调用前确保用户已经登入 | <md-version>3.1.0</md-version> | <md-version>3.1.0</md-version> | <md-version>3.1.0</md-version> |
| [getChatInfo](/document/uYjL24iN/uEDN2UjLxQjN14SM0YTN) | 获取某个会话的信息 | <md-version>3.10.0</md-version> | <md-version>3.10.0</md-version> | <md-version>3.10.0</md-version> |
| [getBlockActionSourceDetail](/document/getBlockActionSourceDetail) | 支持从block action点击进入应用后，获取block对应业务的详细信息 | **✓** | **✓** | **✓** |
| [enterBot](/document/uYjL24iN/uAjM1EjLwITNx4CMyUTM) | 打开机器人聊天页面 | <md-version>2.7.0</md-version> | <md-version>2.7.0</md-version> | <md-version>2.7.0</md-version> |
| [sendMessageCard](/document/uYjL24iN/uUjN5UjL1YTO14SN2kTN) | 发送消息卡片到指定会话 | <md-version>3.19.0</md-version> | <md-version>3.19.0</md-version> | <md-version>3.19.0</md-version> |
| [onChatBadgeChange](/document/uYjL24iN/uQDN2UjL0QjN14CN0YTN) | 监听某个群未读消息数变化，确保用户已经登录 | <md-version>3.10.0</md-version> | <md-version>3.10.0</md-version> | <md-version>3.10.0</md-version> |
| [offChatBadgeChange](/document/uYjL24iN/ugDM04COwQjL4ADN/offchatbadgechange) | 取消监听某个群未读消息数变化 | <md-version>3.10.0</md-version> | <md-version>3.10.0</md-version> | <md-version>3.10.0</md-version> |

#### 联系人

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [chooseContact](/document/uYjL24iN/uMTM04yMxQjLzEDN) | 打开用户联系人选择列表 | **✓** | **✓** | **✓** |



#### 设置

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [openSetting](/document/uYjL24iN/uUzMx4SNzEjL1MTM) | 打开设置页面，展示用户设置（包括授予和拒绝）过的权限，并返回用户设置过的授权结果 | **✓** | **✓** | **✓** |
| [getSetting](/document/uYjL24iN/uQzMx4CNzEjL0MTM) | 获取用户设置（包括授予和拒绝）过的权限 | **✓** | **✓** | **✓** |

#### 分享

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [share](/document/uYjL24iN/ugDM04COwQjL4ADN/thirdShare) | 分享内容到三方应用 | <md-version>3.47.0</md-version> | <md-version>3.47.0</md-version> | **X** |
| [showShareMenu](/document/uYjL24iN/ugjN24CO2YjL4YjN) | 显示当前页面的分享按钮 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [hideShareMenu](/document/uYjL24iN/ukjN24SO2YjL5YjN) | 隐藏当前页面的分享按钮 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |

#### 授权

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [showShareMenu](/document/uYjL24iN/ugjN24CO2YjL4YjN) | 显示当前页面的分享按钮 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [hideShareMenu](/document/uYjL24iN/ukjN24SO2YjL5YjN) | 隐藏当前页面的分享按钮 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [authorize](/document/uYjL24iN/ugzMx4COzEjL4MTM) | 向用户发出设置权限请求。如果该权限用户没有设置过，会弹窗咨询用户是否授予；如果该权限用户拒绝授予，会打开设置页面(appBadge权限除外)；如果该权限用户同意授予，会直接返回成功 | **✓** | **✓** | **✓** |

#### Lark启动参数

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getLaunchOptionsSync](/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN) | 获取小程序启动时的参数 | <md-version>3.22</md-version> | <md-version>3.22</md-version> | <md-version>3.22</md-version> |
| [getHostLaunchQuery](/document/uYjL24iN/ugzM4UjL4MDO14COzgTN) | 获取启动时传入的参数 | <md-version>3.1</md-version> | <md-version>3.1</md-version> | <md-version>3.1</md-version> |

#### 安全密码验证

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [startPasswordVerify](/document/uYjL24iN/ugTO3IjL4kzNy4CO5cjM) | 调起二次验证Lark安全密码的输入界面 | <md-version>3.1.0</md-version> | <md-version>3.1.0</md-version> | <md-version>3.1.0</md-version> |

#### 系统认证

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [startDeviceCredential](/document/uYjL24iN/uIDN14iM0UjLyQTN) | 打开系统解锁界面 | **✓** | **✓** | **X** |

#### 水印

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [checkWatermark](/document/uYjL24iN/ukTM1EjL5ETNx4SOxUTM) | 查看宿主是否显示了全局水印 | <md-version>2.7.0</md-version> | <md-version>2.7.0</md-version> | <md-version>2.7.0</md-version> |

#### 邮件

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [mailto](/document/uYjL24iN/uAjNwEjLwYDMx4CM2ATM) | 调用系统发送邮件 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | **X** |

## 界面
#### 交互反馈

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [showActionSheet](/document/uYjL24iN/ukDNy4SO0IjL5QjM) | 显示操作菜单 | **✓** | **✓** | **✓** |
| [showModal](/document/uYjL24iN/ugDNy4CO0IjL4QjM) | 显示模态弹窗 | **✓** | **✓** | **✓** |
| [showPrompt](/document/uYjL24iN/uYTO4UjL2kDO14iN5gTN) | 展示可输入内容的弹窗 | <md-version>3.17.0</md-version> | <md-version>3.17.0</md-version> | <md-version>3.17.0</md-version> |
| [showLoading](/document/uYjL24iN/uMDNy4yM0IjLzQjM) | 显示灰色背景的 loading 提示框 | **✓** | **✓** | **✓** |
| [hideLoading](/document/uYjL24iN/uYDNy4iN0IjL2QjM) | 隐藏 loading 提示框 | **✓** | **✓** | **✓** |
| [showToast](/document/uYjL24iN/ugzMy4COzIjL4MjM) | 显示灰色背景的消息提示框 | **✓** | **✓** | **✓** |
| [hideToast](/document/uYjL24iN/ukzMy4SOzIjL5MjM) | 隐藏灰色背景的消息提示框 | **✓** | **✓** | **✓** |

#### Tab Bar

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [showTabBar](/document/uYjL24iN/uATN04CM1QjLwUDN) | 显示 tabBar | **✓** | **✓** | **✓** |
| [hideTabBar](/document/uYjL24iN/ukDN04SO0QjL5QDN) | 隐藏 tabBar | **✓** | **✓** | **✓** |
| [setTabBarItem](/document/uYjL24iN/uETN04SM1QjLxUDN) | 动态设置 tabBar 某一项的内容 | **✓** | **✓** | **✓** |
| [addTabBarItem](/document/uYjL24iN/uQjM04CNyQjL0IDN/addtabbaritem) | 当前小程序的tab bar数量进行增加调整 | <md-version>5.1.0</md-version> | <md-version>5.1.0</md-version> | <md-version>5.1.0</md-version> |
| [removeTabBarItem](/document/uYjL24iN/uQjM04CNyQjL0IDN/removetabbaritem) | 删除tab bar的目标item | <md-version>4.2.0</md-version> | <md-version>4.2.0</md-version> | <md-version>4.2.0</md-version> |
| [setTabBarStyle](/document/uYjL24iN/uITN04iM1QjLyUDN) | 动态设置 tabBar 的整体样式 | **✓** | **✓** | **✓** |
| [setTabBarBadge](/document/uYjL24iN/uUjM04SNyQjL1IDN) | 为 tabBar 某一项的右上角添加文本 | **✓** | **✓** | **✓** |
| [removeTabBarBadge](/document/uYjL24iN/ucjM04yNyQjL3IDN) | 移除 tabBar 某一项右上角的文本 | **✓** | **✓** | **✓** |
| [showTabBarRedDot](/document/uYjL24iN/uYjM04iNyQjL2IDN) | 显示 tabBar 某一项的右上角的红点，可以使用底部标签栏红点给予用户提示 | **✓** | **✓** | **✓** |
| [hideTabBarRedDot](/document/uYjL24iN/ugjM04COyQjL4IDN) | 隐藏 tabBar 某一项的右上角的红点 | **✓** | **✓** | **✓** |

#### 导航栏

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [setNavigationBarTitle](/document/uYjL24iN/uATNy4CM1IjLwUjM) | 设置导航栏标题 | **✓** | **✓** | **✓** |

#### 窗口

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [setWindowSize](/document/uYjL24iN/uEDO3UjLxgzN14SM4cTN/setwindowsize) | 小程序在 window 和 window-semi 模式下调整独立窗口的大小和位置 | **X** | **X** | <md-version>4.3.0</md-version> |
| [onWindowResize](/document/uYjL24iN/uADO3UjLwgzN14CM4cTN) | 监听窗口尺寸变化事件 | **✓** | **✓** | <md-version>3.13.0</md-version> |
| [offWindowResize](/document/uYjL24iN/uIDO3UjLygzN14iM4cTN) | 取消监听窗口尺寸变化事件 | **✓** | **✓** | <md-version>3.13.0</md-version> |

#### 下拉刷新

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [Page.onPullDownRefresh](/document/uYjL24iN/uQTNy4CN1IjL0UjM) | 在 Page 中注册下拉刷新的监听方法，当用户触发下拉刷新时会调用 | **✓** | **✓** | **X** |
| [startPullDownRefresh](/document/uYjL24iN/uYTNy4iN1IjL2UjM) | 下拉刷新 | **✓** | **✓** | **X** |
| [stopPullDownRefresh](/document/uYjL24iN/ugTNy4CO1IjL4UjM) | 停止当前页面下拉刷新 | **✓** | **✓** | **X** |

#### 页面位置

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [pageScrollTo](/document/uYjL24iN/uITNy4iM1IjLyUjM) | 滚动页面到目标位置 | **✓** | **✓** | **✓** |

#### Canvas绘图

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [createCanvasContext](/document/uYjL24iN/uMTNy4yM1IjLzUjM) | 创建并返回对应 canvasId 的绘图上下文 | **✓** | **✓** | **✓** |
| [canvasToTempFilePath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvastotempfilepath) | 导出当前画布指定区域，生成图片并返回文件路径 | **✓** | **✓** | **✓** |
| [canvasPutImageData](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvasputimagedata) | 更新画布像素数据 | <md-version>3.45.0</md-version> | <md-version>3.45.0</md-version> | <md-version>3.45.0</md-version> |
| [canvasGetImageData](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvasgetimagedata) | 获取画布像素数据 | <md-version>3.45.0</md-version> | <md-version>3.45.0</md-version> | <md-version>3.45.0</md-version> |
| [CanvasContext.createPattern](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createPattern) | 创建径向渐变管理对象 | <md-version>3.45.0</md-version> | <md-version>3.45.0</md-version> | <md-version>3.45.0</md-version> |
| [CanvasContext.createCircularGradient](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createCircularGradient) | 创建圆形渐变管理对象 | **✓** | **✓** | **✓** |
| [CanvasContext.translate](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-translate) | 平移坐标矩阵 | **✓** | **✓** | **✓** |
| [CanvasContext.transform](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-transform) | 坐标转换矩阵叠加，每一次调用会在乘以前一次的变换矩阵 | **✓** | **✓** | **✓** |
| [CanvasContext.strokeText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeText) | 绘制文字路径 | **✓** | **✓** | **✓** |
| [CanvasContext.strokeRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeRect) | 绘制矩形路径，不添加到当前路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke) | 绘制当前路径 | **✓** | **✓** | **✓** |
| [CanvasContext.setTransform](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTransform) | 设置坐标转换矩阵 | **✓** | **✓** | **✓** |
| [CanvasContext.setTextBaseline](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextBaseline) | 设置字体的对齐基线 | **✓** | **✓** | **✓** |
| [CanvasContext.setTextAlign](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextAlign) | 设置字体对齐方式 | **✓** | **✓** | **✓** |
| [CanvasContext.setStrokeStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setStrokeStyle) | 设置绘制线样式 | **✓** | **✓** | **✓** |
| [CanvasContext.setShadow](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setShadow) | 设置阴影 | **✓** | **✓** | **✓** |
| [CanvasContext.setMiterLimit](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setMiterLimit) | 设置线连接点渲染的斜面倾斜程度 | **✓** | **✓** | **✓** |
| [CanvasContext.setLineWidth](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineWidth) | 设置线宽 | **✓** | **✓** | **✓** |
| [CanvasContext.setLineJoin](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineJoin) | 设置线连接点样式 | **✓** | **✓** | **✓** |
| [CanvasContext.setLineDash](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineDash) | 设置间断线 | **✓** | **✓** | **✓** |
| [CanvasContext.setLineCap](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineCap) | 设置线端点样式 | **✓** | **✓** | **✓** |
| [CanvasContext.setGlobalAlpha](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setGlobalAlpha) | 设置全局不透明度 | **✓** | **✓** | **✓** |
| [CanvasContext.setFillStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setFillStyle) | 设置填充样式 | **✓** | **✓** | **✓** |
| [CanvasContext.scale](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-scale) | 缩放坐标点 | **✓** | **✓** | **✓** |
| [CanvasContext.save](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-save) | 创建新的绘图上下文，并将之前的上下文保存在栈中 | **✓** | **✓** | **✓** |
| [CanvasContext.rotate](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rotate) | 旋转坐标点 | **✓** | **✓** | **✓** |
| [CanvasContext.restore](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-restore) | 恢复栈中存储的上下文 | **✓** | **✓** | **✓** |
| [CanvasContext.rect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rect) | 添加矩形到当前路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.quadraticCurveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-quadraticCurveTo) | 添加二次贝塞尔曲线到路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.moveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-moveTo) | 移动绘制点 | **✓** | **✓** | **✓** |
| [CanvasContext.measureText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-measureText) | 测量文字宽度 | <md-version>3.9.0</md-version> | <md-version>3.9.0</md-version> | <md-version>3.9.0</md-version> |
| [CanvasContext.lineTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-lineTo) | 移动并添加线段到路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.fillText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillText) | 填充文字 | **✓** | **✓** | **✓** |
| [CanvasContext.fillRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillRect) | 填充矩形，不添加到当前路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill) | 填充当前路径 | **✓** | **✓** | **✓** |
| [CanvasContext.drawImage](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-drawImage) | 绘制 Image | **✓** | **✓** | **✓** |
| [CanvasContext.createLinearGradient](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createLinearGradient) | 创建线性渐变对象 | **✓** | **✓** | **✓** |
| [CanvasContext.closePath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-closePath) | 闭合当前路径 | **✓** | **✓** | **✓** |
| [CanvasContext.clip](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clip) | 剪切当前路径，限制后续的渲染范围 | **✓** | **✓** | **✓** |
| [CanvasContext.clearRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clearRect) | 清空画布矩形区域 | **✓** | **✓** | **✓** |
| [CanvasContext.bezierCurveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-bezierCurveTo) | 添加三次贝塞尔曲线到路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.beginPath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-beginPath) | 创建新的子路径 | **✓** | **✓** | **✓** |
| [CanvasContext.arcTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arcTo) | 移动并添加弧线到当前路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.arc](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arc) | 添加圆弧到当前路径中 | **✓** | **✓** | **✓** |
| [CanvasContext.draw](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-draw) | 将所有的操作绘制到 Canvas 中 | **✓** | **✓** | **✓** |

#### 动画

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [createAnimation](/document/uYjL24iN/uETNy4SM1IjLxUjM) | 创建一个动画实例 animation。调用实例的方法来描述动画。最后通过动画实例的 export 方法导出动画数据传递给组件的 animation 属性 | **✓** | **✓** | **✓** |
| [Animation.backgroundColor](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_backgroundcolor) | 设置背景色 | **✓** | **✓** | **✓** |
| [Animation.bottom](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_bottom) | 设置 bottom 值 | **✓** | **✓** | **✓** |
| [Animation.export](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_export) | 导出动画队列。export 方法每次调用后会清掉之前的动画操作 | **✓** | **✓** | **✓** |
| [Animation.height](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_height) | 设置高度 | **✓** | **✓** | **✓** |
| [Animation.left](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_left) | 设置 left 值 | **✓** | **✓** | **✓** |
| [Animation.matrix](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_matrix) | 同 transform-function matrix | **✓** | **✓** | **✓** |
| [Animation.matrix3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_matrix3d) | 同 transform-function matrix3d | **✓** | **✓** | **✓** |
| [Animation.opacity](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_opacity) | 设置透明度 | **✓** | **✓** | **✓** |
| [Animation.right](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_right) | 设置 right 值 | **✓** | **✓** | **✓** |
| [Animation.rotate](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotate) | 从原点顺时针旋转一个角度 | **✓** | **✓** | **✓** |
| [Animation.rotate3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotate3d) | 从 固定 轴顺时针旋转一个角度 | **✓** | **✓** | **✓** |
| [Animation.rotateX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotatex) | 从 X 轴顺时针旋转一个角度 | **✓** | **✓** | **✓** |
| [Animation.rotateY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotateY) | 从 Y 轴顺时针旋转一个角度 | **✓** | **✓** | **✓** |
| [Animation.rotateZ](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotatez) | 从 Z 轴顺时针旋转一个角度 | **✓** | **✓** | **✓** |
| [Animation.scale](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scale) | 缩放 | **✓** | **✓** | **✓** |
| [Animation.scale3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scale3d) | 缩放 | **✓** | **✓** | **✓** |
| [Animation.scaleX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scalex) | 缩放 X 轴 | **✓** | **✓** | **✓** |
| [Animation.scaleY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scaley) | 缩放 Y 轴 | **✓** | **✓** | **✓** |
| [Animation.scaleZ](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scalez) | 缩放 Z 轴 | **✓** | **✓** | **✓** |
| [Animation.skew](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skew) | 对 X、Y 轴坐标进行倾斜 | **✓** | **✓** | **✓** |
| [Animation.skewX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skewx) | 对 X 轴坐标进行倾斜 | **✓** | **✓** | **✓** |
| [Animation.skewY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skewy) | 对 Y 轴坐标进行倾斜 | **✓** | **✓** | **✓** |
| [Animation.step](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_step) | 表示一组动画完成。可以在一组动画中调用任意多个动画方法，一组动画中的所有动画会同时开始，一组动画完成后才会进行下一组动画 | **✓** | **✓** | **✓** |
| [Animation.top](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_top) | 设置 top 值 | **✓** | **✓** | **✓** |
| [Animation.translate](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translate) | 平移变换 | **✓** | **✓** | **✓** |
| [Animation.translate3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translate3d) | 对 xyz 坐标进行平移变换 | **✓** | **✓** | **✓** |
| [Animation.translateX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translatex) | 对 X 轴平移 | **✓** | **✓** | **✓** |
| [Animation.translateY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translatey) | 对 Y 轴平移 | **✓** | **✓** | **✓** |
| [Animation.translateZ](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translatez) | 对 Z 轴平移 | **✓** | **✓** | **✓** |
| [Animation.width](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_width) | 设置宽度 | **✓** | **✓** | **✓** |

#### Customized Input

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getCustomizedInput](/document/uYjL24iN/uEDN1EjLxQTNx4SM0UTM) | 获取全局唯一的customizedInput实例。通过customizedInput显示一个 可定制化的富文本输入框，支持@联系人、插入图片、插入表情、显示用户头像、切换用户头像状态 | **✓** | **✓** | **X** |
| [CustomizedInput.show](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show) | 显示输入框 | **✓** | **✓** | **X** |
| [CustomizedInput.update](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/update) | 更新输入框中显示的内容 | **✓** | **✓** | **X** |
| [CustomizedInput.hide](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/hide) | 隐藏输入框 | **✓** | **✓** | **X** |
| [CustomizedInput.onPicSelect](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpicselect) | 监听连接成功的事件回调 | **✓** | **✓** | **X** |
| [CustomizedInput.onModelSelect](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onmodelselect) | 选择pickerView之后触发的事件 | **✓** | **✓** | **X** |
| [CustomizedInput.onPublish](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpublish) | 点击发送按钮触发的事件 | **✓** | **✓** | **X** |
| [CustomizedInput.onHide](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onhide) | 隐藏输入框之后触发的事件 | **✓** | **✓** | **X** |

#### Pad

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [togglePadFullScreen](/document/uYjL24iN/uUTOuUTOuUTO/pad/togglepadfullscreen) | 在当前Pad小程序窗口可以全屏缩放的前提下， 进行全屏缩放状态的切换 | <md-version>4.10.0</md-version> | <md-version>4.10.0</md-version> | **X** |
| [getPadDisplayScaleMode](/document/uYjL24iN/uUTOuUTOuUTO/pad/getpaddisplayscalemode) | 获取当前Pad的小程序窗口缩放状态，当前显示状态 能否进行全屏缩放的切换 | <md-version>4.10.0</md-version> | <md-version>4.10.0</md-version> | **X** |

## 设备
#### 系统信息

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM) | 获取系统信息 | **✓** | **✓** | **✓** |
| [getSystemInfoSync](/document/uYjL24iN/uUjNx4SN2EjL1YTM) | 获取系统信息 | **✓** | **✓** | **✓** |

#### NFC

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getNFCAdapter](/document/uYjL24iN/ukzM4YjL5MDO24SOzgjN) | 获取 NFC 实例 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NFCAdapter.getNfcA](/document/uYjL24iN/ugzM4YjL4MDO24COzgjN) | 获取NfcA实例，实例支持NFC-A (ISO 14443-3A)标准的读写 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NFCAdapter.getMifareClassic](/document/uYjL24iN/uEDN4YjLxQDO24SM0gjN) | 获取MifareClassic实例，实例支持MIFARE Classic标签的读写 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NFCAdapter.startDiscovery](/document/uYjL24iN/uIDN4YjLyQDO24iM0gjN) | 开始扫描NFC标签 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NFCAdapter.stopDiscovery](/document/uYjL24iN/uMDN4YjLzQDO24yM0gjN) | 关闭NFC标签扫描 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NFCAdapter.onDiscovered](/document/uYjL24iN/uUDN4YjL1QDO24SN0gjN) | 监听 NFC Tag | <md-version>3.38.0</md-version> | **X** | **X** |
| [NFCAdapter.offDiscovered](/document/uYjL24iN/uQDN4YjL0QDO24CN0gjN) | 取消监听 NFC Tag | <md-version>3.38.0</md-version> | **X** | **X** |
| [NfcA.connect](/document/uYjL24iN/ucDN4YjL3QDO24yN0gjN) | 连接NfcA类型的标签 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NfcA.transceive](/document/uYjL24iN/uITN4YjLyUDO24iM1gjN) | 发送数据给NFCA类型的标签 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NfcA.close](/document/uYjL24iN/uYDN4YjL2QDO24iN0gjN) | 断开与NFCA标签之间的连接 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NfcA.getAtqa](/document/uYjL24iN/ugDN4YjL4QDO24CO0gjN) | 获取ATQA信息 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NfcA.getMaxTransceiveLength](/document/uYjL24iN/ukDN4YjL5QDO24SO0gjN) | 获取最大传输长度 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NfcA.getSak](/document/uYjL24iN/uATN4YjLwUDO24CM1gjN) | 获取SAK信息 | <md-version>3.38.0</md-version> | **X** | **X** |
| [NfcA.setTimeout](/document/uYjL24iN/uETN4YjLxUDO24SM1gjN) | 设置超时时间 | <md-version>3.38.0</md-version> | **X** | **X** |
| [MifareClassic.connect](/document/uYjL24iN/uQTN4YjL0UDO24CN1gjN) | 连接MifareClassic类型的标签 | <md-version>3.38.0</md-version> | **X** | **X** |
| [MifareClassic.transceive](/document/uYjL24iN/ucTN4YjL3UDO24yN1gjN) | 发送数据给MifareClassic类型的标签 | <md-version>3.38.0</md-version> | **X** | **X** |
| [MifareClassic.close](/document/uYjL24iN/uMTN4YjLzUDO24yM1gjN) | 断开与MifareClassic标签之间的连接 | <md-version>3.38.0</md-version> | **X** | **X** |
| [MifareClassic.getMaxTransceiveLength](/document/uYjL24iN/uUTN4YjL1UDO24SN1gjN) | 获取最大传输长度 | <md-version>3.38.0</md-version> | **X** | **X** |
| [MifareClassic.setTimeout](/document/uYjL24iN/uYTN4YjL2UDO24iN1gjN) | 设置超时时间 | <md-version>3.38.0</md-version> | **X** | **X** |

#### 蓝牙

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [openBluetoothAdapter](/document/uYjL24iN/ugzNxYjL4cTM24CO3EjN) | 初始化蓝牙模块 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [closeBluetoothAdapter](/document/uYjL24iN/uYDOxYjL2gTM24iN4EjN) | 关闭蓝牙模块。调用该方法将断开所有已建立的连接并释放系统资源。建议在使用蓝牙流程后，与 tt.openBluetoothAdapter 成对调用 | <md-version>3.25.0</md-version> | <md-version>3.25.0</md-version> | **X** |
| [getBluetoothAdapterState](/document/uYjL24iN/uUDOxYjL1gTM24SN4EjN) | 获取本机蓝牙适配器状态 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [startBluetoothDevicesDiscovery](/document/uYjL24iN/uUzNxYjL1cTM24SN3EjN) | 开始搜寻附近的蓝牙外围设备 | <md-version>3.44</md-version> | <md-version>3.44</md-version> | **X** |
| [stopBluetoothDevicesDiscovery](/document/uYjL24iN/uczNxYjL3cTM24yN3EjN) | 停止搜寻附近的蓝牙外围设备。若已经找到需要的蓝牙设备并不需要继续搜索时，建议调用该接口停止蓝牙搜索 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [getConnectedBluetoothDevices](/document/uYjL24iN/uMDOxYjLzgTM24yM4EjN) | 根据 uuid 获取处于已连接状态的设备 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [getBluetoothDevices](/document/uYjL24iN/uQDOxYjL0gTM24CN4EjN) | 获取在蓝牙模块生效期间所有已发现的蓝牙设备。包括已经和本机处于连接状态的设备 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [onBluetoothDeviceFound](/document/uYjL24iN/ukzNxYjL5cTM24SO3EjN) | 监听寻找到新设备的事件 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [offBluetoothDeviceFound](/document/uYjL24iN/uEDOxYjLxgTM24SM4EjN) | 取消监听寻找到新设备的事件 | <md-version>3.25.0</md-version> | <md-version>3.25.0</md-version> | **X** |
| [onBluetoothAdapterStateChange](/document/uYjL24iN/uADOxYjLwgTM24CM4EjN) | 监听蓝牙适配器状态变化事件 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [offBluetoothAdapterStateChange](/document/uYjL24iN/uIDOxYjLygTM24iM4EjN) | 取消监听蓝牙适配器状态变化事件 | <md-version>3.25.0</md-version> | <md-version>3.25.0</md-version> | **X** |

#### 低功耗蓝牙

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [setBLEMTU](/document/uYjL24iN/uMTMyYjLzEjM24yMxIjN) | 设置蓝牙最大传输单元 | <md-version>3.26</md-version> | **X** | **X** |
| [readBLECharacteristicValue](/document/uYjL24iN/uYTOxYjL2kTM24iN5EjN) | 读取数据 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [writeBLECharacteristicValue](/document/uYjL24iN/ucTOxYjL3kTM24yN5EjN) | 写入数据 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [getBLEDeviceCharacteristics](/document/uYjL24iN/ukDOxYjL5gTM24SO4EjN) | 获取读写特征 | <md-version>3.25</md-version> | <md-version>32.5</md-version> | **X** |
| [connectBLEDevice](/document/uYjL24iN/ucDOxYjL3gTM24yN4EjN) | 链接外围设备 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [disconnectBLEDevice](/document/uYjL24iN/ugDOxYjL4gTM24CO4EjN) | 断开设备连接 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [getBLEDeviceServices](/document/uYjL24iN/uATOxYjLwkTM24CM5EjN) | 获取设备服务 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [onBLEConnectionStateChange](/document/uYjL24iN/uUTOxYjL1kTM24SN5EjN) | 蓝牙连接状态变化 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [offBLEConnectionStateChange](/document/uYjL24iN/uMTOxYjLzkTM24yM5EjN) | 取消监听蓝牙低功耗连接状态的改变事件 | <md-version>3.25.0</md-version> | <md-version>3.25.0</md-version> | **X** |
| [notifyBLECharacteristicValueChange](/document/uYjL24iN/uETOxYjLxkTM24SM5EjN) | 监听特征值数据变化 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [onBLECharacteristicValueChange](/document/uYjL24iN/uQTOxYjL0kTM24CN5EjN) | 监听特征值数据变化 | <md-version>3.25</md-version> | <md-version>3.25</md-version> | **X** |
| [offBLECharacteristicValueChange](/document/uYjL24iN/uITOxYjLykTM24iM5EjN) | 取消监听蓝牙低功耗设备的特征值变化事件 | <md-version>3.25.0</md-version> | <md-version>3.25.0</md-version> | **X** |

#### Beacon

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [startBeaconDiscovery](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/startbeacondiscovery) | 开始搜索附近的 iBeacon 设备 | <md-version>4.6.0</md-version> | <md-version>4.6.0</md-version> | **X** |
| [stopBeaconDiscovery](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/stopbeacondiscovery) | 停止搜索附近的 iBeacon 设备 | <md-version>4.6.0</md-version> | <md-version>4.6.0</md-version> | **X** |
| [getBeacons](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/getbeacons) | 获取所有已搜索到的 iBeacon 设备 | <md-version>4.6.0</md-version> | <md-version>4.6.0</md-version> | **X** |
| [onBeaconUpdate](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/onbeaconupdate) | 监听 iBeacon 设备更新事件，仅能注册一个监听 | <md-version>4.6.0</md-version> | <md-version>4.6.0</md-version> | **X** |
| [offBeaconUpdate](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconupdate) | 取消监听 iBeacon 设备更新事件 | <md-version>4.6.0</md-version> | <md-version>4.6.0</md-version> | **X** |
| [onBeaconServiceChange](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/onbeaconservicechange) | 监听蓝牙适配器状态变化事件 | <md-version>4.6.0</md-version> | <md-version>4.6.0</md-version> | **X** |
| [offBeaconServiceChange](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconservicechange) | 取消监听 iBeacon 服务状态变化事件 | <md-version>4.6.0</md-version> | <md-version>4.6.0</md-version> | **X** |

#### 扫码

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [scanCode](/document/uYjL24iN/uYzNx4iN3EjL2cTM) | 扫描二维码并返回扫描结果 | **✓** | **✓** | **X** |

#### Wi-Fi

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getConnectedWifi](/document/uYjL24iN/ugjNx4CO2EjL4YTM) | 获取设备当前所连的 Wifi | **✓** | **✓** | **X** |
| [getWifiStatus](/document/uYjL24iN/uYTN4QjL2UDO04iN1gDN) | 请求获取 Wi-Fi 开关状态 | **✓** | **✓** | **X** |
| [getWifiList](/document/uYjL24iN/uUDO4UjL1gDO14SN4gTN) | 请求获取Wifi 列表 | **✓** | **X** | **X** |
| [onGetWifiList](/document/uYjL24iN/uYDO4UjL2gDO14iN4gTN) | 监听获取到 Wi-Fi 列表数据事件 | **✓** | **X** | **X** |
| [offGetWifiList](/document/uYjL24iN/ucDO4UjL3gDO14yN4gTN) | 取消监听获取到 Wi-Fi 列表数据事件 | **✓** | **X** | **X** |

#### 剪贴板

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [setClipboardData](/document/uYjL24iN/ugzNx4CO3EjL4cTM) | 设置系统剪贴板内容 | **✓** | **✓** | **✓** |
| [getClipboardData](/document/uYjL24iN/uczNx4yN3EjL3cTM) | 获取系统粘贴板数据 | **✓** | **✓** | **✓** |

#### 网络状态

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getNetworkType](/document/uYjL24iN/uYjNx4iN2EjL2YTM) | 获取设备当前所处的网络类型 | **✓** | **✓** | **✓** |
| [onNetworkStatusChange](/document/uYjL24iN/ucjNx4yN2EjL3YTM) | 监听网络状态变化 | **✓** | **✓** | **✓** |
| [getNetworkQualityType](/document/uYjL24iN/uUTNx4SN1EjL1UTM/getnetworkqualitytype) | 网络评级接口，获取当前设备所处的网络状态 | <md-version>4.9.0</md-version> | <md-version>4.9.0</md-version> | <md-version>5.1.0</md-version> |
| [onNetworkQualityChange](/document/uYjL24iN/uUTNx4SN1EjL1UTM/onnetworkqualitychange) | 监听网络质量变化 | <md-version>4.9.0</md-version> | <md-version>4.9.0</md-version> | <md-version>5.1.0</md-version> |

#### 屏幕亮度

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [setScreenBrightness](/document/uYjL24iN/uIjNx4iM2EjLyYTM/set-screen-brightness) | 设置屏幕亮度 | <md-version>3.42.0</md-version> | <md-version>3.42.0</md-version> | **X** |
| [getScreenBrightness](/document/uYjL24iN/uIjNx4iM2EjLyYTM/get-screen-brightness) | 获取屏幕亮度 | <md-version>3.42.0</md-version> | <md-version>3.42.0</md-version> | **X** |
| [setKeepScreenOn](/document/uYjL24iN/ukzNx4SO3EjL5cTM) | 设置是否保持常亮状态 | **✓** | **✓** | **X** |

#### 截屏监听

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [onUserCaptureScreen](/document/uYjL24iN/uMjNwEjLzYDMx4yM2ATM) | 监听用户主动截屏事件。用户使用系统截屏按键截屏时触发 | <md-version>2.4.0</md-version> | <md-version>2.4.0</md-version> | **X** |
| [offUserCaptureScreen](/document/uYjL24iN/uQjNwEjL0YDMx4CN2ATM) | 取消监听用户主动截屏事件 | <md-version>2.4.0</md-version> | <md-version>2.4.0</md-version> | **X** |

#### 加速度计

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [startAccelerometer](/document/uYjL24iN/ukjNx4SO2EjL5YTM) | 通知客户端开始监听加速度计数据。具体的数据返回通过注册onAccelerometerChange接口回调方法获取 | **✓** | **✓** | **X** |
| [stopAccelerometer](/document/uYjL24iN/uAzNx4CM3EjLwcTM) | 停止监听加速度计数据 | **✓** | **✓** | **X** |
| [onAccelerometerChange](/document/uYjL24iN/uEzNx4SM3EjLxcTM) | 监听加速度计数据 | **✓** | **✓** | **X** |

#### 拨打电话

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [makePhoneCall](/document/uYjL24iN/uUzNx4SN3EjL1cTM) | 拨打电话 | **✓** | **✓** | **X** |

#### 震动

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [vibrateShort](/document/uYjL24iN/uADOx4CM4EjLwgTM) | 使手机发生较短时间的振动 | **✓** | **✓** | **X** |
| [vibrateLong](/document/uYjL24iN/uEDOx4SM4EjLxgTM) | 使手机发生较长时间的振动 | **✓** | **✓** | **X** |

#### 罗盘

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [startCompass](/document/uYjL24iN/uIzNx4iM3EjLycTM) | 开始监听罗盘数据。 | **✓** | **✓** | **X** |
| [stopCompass](/document/uYjL24iN/uMzNx4yM3EjLzcTM) | 停止监听罗盘数据 | **✓** | **✓** | **X** |
| [onCompassChange](/document/uYjL24iN/uQzNx4CN3EjL0cTM) | 监听罗盘数据变化事件，频率：5 次/秒，接口调用后会自动开始监听，可使用 tt.stopCompass 停止监听 | **✓** | **✓** | **X** |


## 文件

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [saveFile](/document/uYjL24iN/ugDOz4CO4MjL4gzM) | 保存临时文件到本地永久目录 | **✓** | **✓** | **✓** |
| [saveFileAs](/document/uYjL24iN/uQjN3UjL0YzN14CN2cTN) | 保存文件到本地指定目录 | **X** | **X** | <md-version>3.9.0</md-version> |
| [filePicker](/document/uYjL24iN/uETM04SMxQjLxEDN) | 打开附件选择列表 | **✓** | **✓** | **✓** |
| [docsPicker](/document/uYjL24iN/ukTN3UjL5UzN14SO1cTN) | 打开云文档选择列表 | <md-version>3.12.0</md-version> | <md-version>3.12.0</md-version> | <md-version>3.13.0</md-version> |
| [openDocument](/document/uYjL24iN/ukTN24SO1YjL5UjN) | 在新页面打开文档 | <md-version>2.6.0</md-version> | <md-version>2.6.0</md-version> | <md-version>2.6.0</md-version> |
| [getFileSystemManager](/document/uYjL24iN/uETOuETOuETO/tt_get_file_system_manager) | 获取全局唯一的文件管理器 | <md-version>4.11</md-version> | <md-version>4.11</md-version> | **X** |
| [FileSystemManager.readFile](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file) | 读取本地文件内容 | <md-version>4.11</md-version> | <md-version>4.11</md-version> | **X** |
| [FileSystemManager.stat](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat) | 获取本地文件 Stats 对象 | <md-version>4.11</md-version> | <md-version>4.11</md-version> | **X** |
| [Stats.isDirectory](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_directory) | 判断当前文件是否一个目录 | <md-version>4.11.0</md-version> | <md-version>4.11.0</md-version> | **X** |
| [Stats.isFile](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_file) | 判断当前文件是否一个普通文件 | <md-version>4.11</md-version> | <md-version>4.11</md-version> | **X** |

## 媒体
#### 图片

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [saveImageToPhotosAlbum](/document/uYjL24iN/uUTMx4SNxEjL1ETM) | 保存图片到系统相册 | **✓** | **✓** | **✓** |
| [chooseImage](/document/uYjL24iN/uMTMx4yMxEjLzETM) | 从系统相册中选择图片，或使用相机拍摄图片 | **✓** | **✓** | **✓** |
| [previewImage](/document/uYjL24iN/uMDOx4yM4EjLzgTM) | 预览一组图片 | **✓** | **✓** | **✓** |
| [compressImage](/document/uYjL24iN/uMjN24yM2YjLzYjN) | 压缩图片接口，可选压缩质量 | <md-version>2.0.0</md-version> | <md-version>2.0.0</md-version> | <md-version>2.0.0</md-version> |
| [getImageInfo](/document/uYjL24iN/ugjNwEjL4YDMx4CO2ATM) | 获取图片信息 | <md-version>2.4.0</md-version> | <md-version>2.4.0</md-version> | <md-version>2.4.0</md-version> |

#### 视频

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [saveVideoToPhotosAlbum](/document/uYjL24iN/ucDOx4yN4EjL3gTM) | 保存视频到系统相册 | **✓** | **✓** | **✓** |
| [chooseVideo](/document/uYjL24iN/uEjMx4SMyEjLxITM) | 从系统相册中选择视频，或使用相机拍摄视频 | **✓** | **✓** | **✓** |
| [chooseMedia](/document/uYjL24iN/uITMx4iMxEjLyETM/choosemedia) | 拍摄或从系统相册中选择图片或视频 | <md-version>4.7.0</md-version> | <md-version>4.7.0</md-version> | <md-version>4.7.0</md-version> |
| [createVideoContext](/document/uYjL24iN/uITMx4iMxEjLyETM/createvideocontext) | 创建 VideoContext 实例，通过 id 跟一个 video 组件绑定，操作对应的 video 组件 | <md-version>4.3.0</md-version> | <md-version>4.3.0</md-version> | **X** |
| [VideoContext.play](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/play) | 播放视频 | <md-version>4.3.0</md-version> | <md-version>4.3.0</md-version> | **X** |
| [VideoContext.pause](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/pause) | 暂停视频 | <md-version>4.3.0</md-version> | <md-version>4.3.0</md-version> | **X** |
| [VideoContext.stop](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/stop) | 停止视频 | <md-version>4.3.0</md-version> | <md-version>4.3.0</md-version> | **X** |
| [VideoContext.seek](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/seek) | 跳转到指定位置 | <md-version>4.3.0</md-version> | <md-version>4.3.0</md-version> | **X** |
| [VideoContext.requestFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/requestfullscreen) | 进入全屏 | <md-version>4.3.0</md-version> | <md-version>4.3.0</md-version> | **X** |
| [VideoContext.exitFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/exitfullscreen) | 退出全屏 | <md-version>4.3.0</md-version> | <md-version>4.3.0</md-version> | **X** |

#### 音频

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [createInnerAudioContext](/document/uYjL24iN/uUDOx4SN4EjL1gTM) | 创建innerAudioContext实例，通过它能够操作音频播放 | **✓** | **✓** | **X** |
| [InnerAudioContext.play](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/play) | 播放音频 | **✓** | **✓** | **X** |
| [InnerAudioContext.pause](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/pause) | 暂停。暂停后的音频再播放会从暂停处开始播放 | **✓** | **✓** | **X** |
| [InnerAudioContext.stop](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/stop) | 停止。停止后的音频再播放会从头开始播放 | **✓** | **✓** | **X** |
| [InnerAudioContext.seek](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/seek) | 跳转到指定位置 | **✓** | **✓** | **X** |
| [InnerAudioContext.destroy](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/destroy) | 销毁当前实例 | **✓** | **✓** | **X** |
| [InnerAudioContext.onCanplay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/oncanplay) | 音频进入可以播放状态时触发回调函数 | **✓** | **✓** | **X** |
| [InnerAudioContext.offCanplay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offcanplay) | 取消监听 Canplay 事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onPlay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onPlay) | 监听音频播放事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offPlay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offplay) | 取消监听音频播放事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onPause](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onPause) | 监听音频暂停事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offPause](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offpause) | 取消监听音频暂停事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onStop](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onstop) | 监听音频停止事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offStop](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offstop) | 取消监听音频停止事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onEnded](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onended) | 监听音频自然播放至结束的事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offEnded](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offEnded) | 取消监听音频自然播放至结束的事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onTimeUpdate](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onTimeUpdate) | 监听音频播放进度更新事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offTimeUpdate](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offtimeupdate) | 取消监听音频播放进度更新事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onError](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onerror) | 监听音频播放错误事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offError](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offerror) | 取消监听音频播放错误事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onWaiting](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onwaiting) | 监听音频加载中事件。当音频因为数据不足，需要停下来加载时会触发 | **✓** | **✓** | **X** |
| [InnerAudioContext.offWaiting](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offwaiting) | 取消监听音频加载中事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onSeeking](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onseeking) | 监听音频进行跳转操作的事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offSeeking](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeking) | 取消监听音频进行跳转操作的事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.onSeeked](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onSeeked) | 监听音频完成跳转操作的事件 | **✓** | **✓** | **X** |
| [InnerAudioContext.offSeeked](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeked) | 取消监听音频完成跳转操作的事件 | **✓** | **✓** | **X** |

#### 录音

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getRecorderManager](/document/uYjL24iN/uQDOx4CN4EjL0gTM) | 获取全局唯一的recorderManager。通过recorderManager进行录音操作和管理 | **✓** | **✓** | **X** |
| [RecorderManager.start](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/start) | 开始录音 | **✓** | **✓** | **X** |
| [RecorderManager.pause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/pause) | 暂停录音 | **✓** | **✓** | **X** |
| [RecorderManager.resume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/resume) | 继续录音 | **✓** | **✓** | **X** |
| [RecorderManager.stop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/stop) | 停止录音 | **✓** | **✓** | **X** |
| [RecorderManager.onStart](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstart) | 监听录音开始事件 | **✓** | **✓** | **X** |
| [RecorderManager.onPause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onpause) | 监听录音暂停事件 | **✓** | **✓** | **X** |
| [RecorderManager.onResume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onresume) | 监听录音继续事件 | **✓** | **✓** | **X** |
| [RecorderManager.onStop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstop) | 监听录音结束事件 | **✓** | **✓** | **X** |
| [RecorderManager.onFrameRecorded](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onframerecorded) | 监听已录制完指定帧大小的文件事件。如果设置了 frameSize，则会回调此事件 | **✓** | **✓** | **X** |
| [RecorderManager.onError](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onerror) | 监听录音错误事件 | **✓** | **✓** | **X** |

## 导航

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [navigateTo](/document/uYjL24iN/uYTOz4iN5MjL2kzM) | 跳转到指定页面。跳转后原页面保留 | **✓** | **✓** | **✓** |
| [navigateBack](/document/uYjL24iN/uADM04CMwQjLwADN) | 返回上一级页面（或上N级页面）。可通过 getCurrentPages 获取当前的页面栈，决定需要返回几层 | **✓** | **✓** | **✓** |
| [redirectTo](/document/uYjL24iN/ucTOz4yN5MjL3kzM) | 关闭当前页面，跳转到指定页面 | **✓** | **✓** | **✓** |
| [switchTab](/document/uYjL24iN/ukTOz4SO5MjL5kzM) | 跳转到指定 TabBar 页面，并关闭其他所有非 TabBar 页面 | **✓** | **✓** | **✓** |
| [reLaunch](/document/uYjL24iN/uEDM04SMwQjLxADN) | 关闭所有当前页面，打开指定页面 | **✓** | **✓** | **✓** |
| [exitMiniProgram](/document/uYjL24iN/uATN4IjLwUDOy4CM1gjM) | 退出当前小程序 | **✓** | **✓** | **✓** |
| [openSchema](/document/uYjL24iN/ukzN4IjL5cDOy4SO3gjM) | 跳转到小程序以外的应用（如云文档、网页等） | <md-version>3.1.0</md-version> | <md-version>3.1.0</md-version> | <md-version>3.1.0</md-version> |

## 网络
#### WebSocket

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [connectSocket](/document/uYjL24iN/ugDMx4COwEjL4ATM) | 创建一个 WebSocket 连接实例，并通过返回的 socketTask 操作该连接 | **✓** | **✓** | **✓** |
| [SocketTask.send](/document/uYjL24iN/ugDOugDOugDO/sockettask/send) | 通过 WebSocket 连接发送数据 | **✓** | **✓** | **✓** |
| [SocketTask.close](/document/uYjL24iN/ugDOugDOugDO/sockettask/close) | 关闭 WebSocket 连接 | **✓** | **✓** | **✓** |
| [SocketTask.onOpen](/document/uYjL24iN/ugDOugDOugDO/sockettask/onopen) | 监听 WebSocket 连接打开事件 | **✓** | **✓** | **✓** |
| [SocketTask.onClose](/document/uYjL24iN/ugDOugDOugDO/sockettask/onclose) | 监听 WebSocket 连接关闭事件 | **✓** | **✓** | **✓** |
| [SocketTask.onMessage](/document/uYjL24iN/ugDOugDOugDO/sockettask/onmessage) | 监听 WebSocket 接受到服务器的消息事件 | **✓** | **✓** | **✓** |

#### HTTP

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [request](/document/uYjL24iN/uIDMx4iMwEjLyATM) | 发起一个 HTTP 请求 | **✓** | **✓** | **✓** |
| [RequestTask.abort](/document/uYjL24iN/ugDNugDNugDN/requesttask/abort) | 中断请求任务 | **✓** | **✓** | **✓** |
| [downloadFile](/document/uYjL24iN/ucDMx4yNwEjL3ATM) | 下载网络文件到本地临时目录 | **✓** | **✓** | **✓** |
| [DownloadTask.onProgressUpdate](/document/uYjL24iN/ugDNugDNugDN/downloadfile/onprogressupdate) | downloadFile的调用结果在通过回调传递的同时会返回一个downloadTask对象，通过onProgressUpdate方法监听下载进度 | **✓** | **✓** | **✓** |
| [DownloadTask.abort](/document/uYjL24iN/ugDNugDNugDN/downloadfile/abort) | 调用downloadFile时，会返回一个downloadTask对象，可以通过该对象的abort方法中断请求任务 | **✓** | **✓** | **✓** |
| [uploadFile](/document/uYjL24iN/uYDMx4iNwEjL2ATM) | 将本地文件上传到网络 | **✓** | **✓** | **✓** |
| [UploadTask.onProgressUpdate](/document/uYjL24iN/ugDNugDNugDN/uploadtask/onprogressupdate) | 调用uploadFile时，会返回一个uploadTask对象，通过onProgressUpdate方法监听下载进度 | **✓** | **✓** | **✓** |
| [UploadTask.abort](/document/uYjL24iN/ugDNugDNugDN/uploadtask/abort) | 调用uploadFile时，会返回一个uploadTask对象，可以通过该对象的abort方法中断请求任务 | **✓** | **✓** | **✓** |

## 地理位置

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [openLocation](/document/uYjL24iN/uQTOz4CN5MjL0kzM) | 使用客户端内置地图查看位置 | **✓** | **✓** | **X** |
| [chooseLocation](/document/uYjL24iN/uUDN1EjL1QTNx4SN0UTM) | 打开地图选择位置 | **✓** | **✓** | **X** |
| [getLocation](/document/uYjL24iN/uUTOz4SN5MjL1kzM) | 获取设备当前的地理位置 | **✓** | **✓** | **X** |

## 数据缓存

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [setStorage](/document/uYjL24iN/uETOx4SM5EjLxkTM) | 以「键值对」的形式设置本地缓存数据 | **✓** | **✓** | **✓** |
| [setStorageSync](/document/uYjL24iN/uITOx4iM5EjLykTM) | 以「键值对」的形式设置本地缓存数据 | **✓** | **✓** | **✓** |
| [getStorage](/document/uYjL24iN/ukDOx4SO4EjL5gTM) | 获取本地缓存数据 | **✓** | **✓** | **✓** |
| [getStorageSync](/document/uYjL24iN/uATOx4CM5EjLwkTM) | 获取本地缓存数据 | **✓** | **✓** | **✓** |
| [removeStorage](/document/uYjL24iN/uMTOx4yM5EjLzkTM) | 删除本地缓存数据 | **✓** | **✓** | **✓** |
| [removeStorageSync](/document/uYjL24iN/uQTOx4CN5EjL0kTM) | 删除本地缓存数据 | **✓** | **✓** | **✓** |
| [clearStorage](/document/uYjL24iN/uUTOx4SN5EjL1kTM) | 清理全部本地缓存数据 | **✓** | **✓** | **✓** |
| [clearStorageSync](/document/uYjL24iN/uYTOx4iN5EjL2kTM) | 清理全部本地缓存数据 | **✓** | **✓** | **✓** |
| [getStorageInfo](/document/uYjL24iN/ucTOx4yN5EjL3kTM) | 获取本地缓存数据的相关信息 | **✓** | **✓** | **✓** |
| [getStorageInfoSync](/document/uYjL24iN/ugTOx4CO5EjL4kTM) | 获取本地缓存数据的相关信息 | **✓** | **✓** | **✓** |

## TTML

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [createSelectorQuery](/document/uYjL24iN/uYjN24iN2YjL2YjN) | 获取一个 SelectorQuery 对象实例 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [SelectorQuery.in](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/in) | 将选择器的选取范围更改为自定义组件 component 内（初始时，选择器仅选取页面范围的节点，不会选取任何自定义组件中的节点） | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [SelectorQuery.select](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/select) | 在当前页面下选择第一个匹配选择器 selector 的节点，返回一个 NodesRef 对象实例，可以用于获取节点信息。  selector 类似于 CSS 的选择器，其中移动端只支持 ID 选择器 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [SelectorQuery.selectAll](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectall) | 在当前页面下选择匹配选择器 selector 的所有节点，返回一个 NodesRef 对象实例，可以用于获取节点信息。  selector 类似于 CSS 的选择器，同 select | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [SelectorQuery.selectViewport](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectviewport) | 选择显示区域。可用于获取显示区域的尺寸、滚动位置等信息 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | **X** |
| [SelectorQuery.exec](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/exec) | 执行所有的请求。请求结果按请求次序构成数组，在callback的第一个参数中返回 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [NodesRef.boundingClientRect](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/boundingclientrect) | 添加节点的布局位置的查询请求 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [NodesRef.scrollOffset](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/scrolloffset) | 添加节点的滚动位置查询请求 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [NodesRef.fields](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/fields) | 获取节点的相关信息 | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> | <md-version>2.2.0</md-version> |
| [createIntersectionObserver](/document/uYjL24iN/ucTNwEjL3UDMx4yN1ATM) | 创建并返回一个 IntersectionObserver 对象实例 | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> |
| [IntersectionObserver.observe](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/observe) | 指定目标节点并开始监听相交状态变化情况 | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> |
| [IntersectionObserver.relativeTo](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativeto) | 使用选择器指定一个节点，作为参照区域之一 | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> |
| [IntersectionObserver.relativeToViewport](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativetoviewport) | 指定页面显示区域作为参照区域之一 | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> |
| [IntersectionObserver.disconnect](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/disconnect) | 停止监听，回调函数将不再触发 | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> | <md-version>2.3.0</md-version> |

## 更新

| API名称 | 说明 | Android | iOS | PC |
| --- | --- | --- | --- | --- |
| [getUpdateManager](/document/uYjL24iN/uEzM04SMzQjLxMDN) | 获取全局唯一的版本更新管理器，返回 updateManager 对象，用于管理小程序更新 | **✓** | **✓** | **✓** |
| [UpdateManager.applyUpdate](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/applyupdate) | 强制小程序重启并使用新版本。在小程序新版本下载完成后（即收到 onUpdateReady 回调）调用 | **✓** | **✓** | **✓** |
| [UpdateManager.onCheckForUpdate](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/oncheckforupdate) | 监听向后台请求检查更新结果事件。客户端在小程序冷启动时自动检查更新，不需由开发者主动触发。线上环境在有更新内容时会触发callback回调 | **✓** | **✓** | **✓** |
| [UpdateManager.onUpdateFailed](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdatefailed) | 监听小程序更新失败事件 | **✓** | **✓** | **✓** |
| [UpdateManager.onUpdateReady](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdateready) | 监听小程序有版本更新事件 | **✓** | **✓** | **✓** |
| [UpdateManager.triggerCheckUpdate](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/triggercheckupdate) | 主动触发更新小程序 | **✓** | **✓** | **✓** |


