---
document_id: '7180231348975779846'
directory_id: '7174249976830509061'
title: 背景音频概述
full_path: /uYjL24iN/ukDOukDOukDO/backgroundaudio/background-audio
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- BackgroundAudio
- BackgroundAudio Overview
document_type: GuideDocumentType
updated_at: 2022-12-23T07:01:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukDOukDOukDO/backgroundaudio/background-audio
---

# 背景音频概述
## 功能介绍
小程序中正在播放的音频随着当前小程序退到后台可以不被打断，支持后台播放。并可以在小程序界面，浮窗界面，状态栏界面分别控制音频播控状态

## 场景介绍
- **小程序播控界面展示** - 小程序内通过接入标准api所定制的播控界面

- **浮窗播控界面展示** - Lark内展示背景音频小浮窗，点击小浮窗可以展开播控界面进行播放控制，注：Android端添加浮窗需要先申请系统添加悬浮窗的权限

- **系统状态栏播控界面展示** - Android系统状态栏，iOS锁屏界面上展示背景音频播控界面进行播放控制

[Android端demo视频展示](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/27c95862d503643236fa4ae3ce0005c0_4i1cpzanO0.mp4)

[iOS端demo视频展示](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e1951f1442831855ff8e02fa30ab195_jDHvplyX9i.mp4)


## 已知问题，及Android，iOS双端差异

Android端 | iOS端
--|--|--|--|--
只支持网络url音频播放，暂不支持本地音频文件的播放 | 只支持网络url音频播放，暂不支持本地音频文件的播放
不支持同一个小程序内多条背景音频音轨同时来回切换播放 | 不支持同一个小程序内多条背景音频音轨同时来回切换播放
背景音频的播放会被其他播放音频打断并暂停 | 会存在背景音频和其他业务音频播放混音的现象。
背景音频的浮窗可以展示在手机系统桌面上 | 背景音频的浮窗只展示在Lark内的各级页面中，退出Lark则不展示浮窗
背景音频的浮窗不会和其他需要抢占浮窗的业务一起展示在浮窗中 | 背景音频的浮窗会和其他需要抢占浮窗的业务一起展示在浮窗中。
