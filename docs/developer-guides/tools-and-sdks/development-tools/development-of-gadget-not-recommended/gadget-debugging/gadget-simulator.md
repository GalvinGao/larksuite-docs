---
document_id: '7034327484552183813'
directory_id: '7027037607365754885'
title: 模拟器
full_path: /uYjL24iN/uEzMzUjLxMzM14SMzMTN/feishu-developer-tools-gadget-simulator
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Gadget Debugging
- Gadget Simulator
document_type: GuideDocumentType
updated_at: 2022-11-17T05:56:24Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzMzUjLxMzM14SMzMTN/feishu-developer-tools-gadget-simulator
---

# Lark开发者工具-小程序模拟器
程序调试主要有模拟器和调试工具

## 模拟器
在开发者工具中模拟小程序在LarkApp中的展现效果，对于大部分的 API 均能够在模拟器上呈现出准确的视觉状态。



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/172224c266eaf48c38d0e1626f4881cb_7iQFnQPe26.png)

## 顶部工具栏
从左到右，模拟器顶部的功能包括：
- 设备尺寸：可以选择预设的iOS或Android设备尺寸，或定义新的设备，用于测试适配性
- 缩放比例：控制应用的显示缩放比例
- 旋转：用于调整横屏和竖屏的效果
- 工具箱（仅小程序）：打开/关闭模拟器的工具菜单

### 工具箱中支持模拟的设备操作
- 前后台切换、[位置](/document/uYjL24iN/uUTOz4SN5MjL1kzM)、[扫码](/document/uYjL24iN/uYzNx4iN3EjL2cTM)、[权限](/document/uYjL24iN/uITMuITMuITM)、[截屏](/document/uYjL24iN/uMjNwEjLzYDMx4yM2ATM)、[罗盘](/document/uYjL24iN/uIzNx4iM3EjLycTM)、[Wi-Fi](/document/uYjL24iN/ugjNx4CO2EjL4YTM)、[加速度计](/document/uYjL24iN/ukjNx4SO2EjL5YTM)
- 模拟部分开放接口：
	- 选择用户联系人 [chooseContact](/document/uYjL24iN/uMTM04yMxQjLzEDN)
	- 选择用户会话 [chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN)
	- 二次验证安全密码 [startPasswordVerify](/document/uYjL24iN/ugTO3IjL4kzNy4CO5cjM)
	- 获取Lark启动参数 [getHostLaunchQuery](/document/uYjL24iN/ugzM4UjL4MDO14COzgTN)
	- 获取某个会话的信息 [getChatInfo](/document/uYjL24iN/uEDN2UjLxQjN14SM0YTN)
- 未读消息：模拟触发未读消息 [onChatBadgeChange](/document/uYjL24iN/uQDN2UjL0QjN14CN0YTN)


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3d1e2b42a761fc125b236a4cd061a810_SuTSwRFyKW.png)

### 底部状态栏
- 底部工具栏：可切换显示页面路径、页面参数和场景值。
- 自动刷新：勾选后可以在保存代码时自动编译小程序。
