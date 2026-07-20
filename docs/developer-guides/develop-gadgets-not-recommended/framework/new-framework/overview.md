---
document_id: '7175034130278580230'
directory_id: '7174665909235073030'
title: 概述
full_path: /uYjL24iN/uEjMuEjMuEjM/new-framework/overview
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- new-framework
- overview
document_type: GuideDocumentType
updated_at: 2022-12-12T05:27:04Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMuEjMuEjM/new-framework/overview
---

# 概述
在 >= 5.12 的Lark客户端的基础库上，除了已存的、线上稳定的小程序渲染框架实现，还额外提供了一套新的渲染框架实现。这套框架实现就是 **新框架**。


![流程图.jpg](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8560e0180bf3023f15a787b6665c098e_twqOvHs1Tw.jpg?lazyload=true&width=874&height=474)

新框架是针对 Component、setData、selectComponent、triggerEvent 等框架接口的一整套优化实现，在组件和 API 方面没有改动。

新框架性能更好，对于使用自定义组件较多的小程序（如基于 uni-app 开发的），能够大幅减少渲染通信次数和渲染耗时，有较为明显的性能提升。  

新框架在接口语法、参数和返回值层面没有变化，但在接口的内部实现方面存在一些 **不兼容改动（breaking changes）**。这些 **不兼容改动** 在大部分场景下都不会有影响，在部分场景下可能会由于业务不兼容而产生 bug。出于对业务稳定性的考虑，这套框架逻辑不会主动开启，而是交由业务侧在 app.json 中自行配置开启。 

![流程图.jpg](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/10c70778861ad77456bb4b6c9e66ea07_ZBsyrEDNQs.jpg?lazyload=true&width=1375&height=684)

建议业务侧在开启后进行一轮回归测试，依照 [不兼容改动](/document/uYjL24iN/uEjMuEjMuEjM/new-framework/breaking-changes)，在解决完兼容问题后当前业务代码对新框架的兼容性。

## 开启新框架
安装最新的[开发者工具](/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN)，并在 app.json 中增加 newFramework 配置项。

```json 
{
    "newFramework": {
        // 是否开启
        "enabled": true,
        // 开启新框架的最低Lark客户端版本，默认且最低为 5.12
        // < 这个版本的Lark客户端下就只会运行旧渲染逻辑
        // >= 这个版本的Lark客户端下才会运行新渲染逻辑
        "minimumSupportedVersion": "5.12"
    }
}
``` 
在上传的情况下，开启新框架的配置规则如上述注释。  

以如上配置为例，增加该配置并上传发布版本后：
- 当用户用 >= 5.12 的Lark客户端打开新发布的小程序时，就会走入新框架，用新框架的框架逻辑渲染。
- 当用户用 < 5.12 的Lark客户端打开新发布的小程序时，就会走入旧框架，用旧框架的框架逻辑渲染。

但是在预览、实时预览和真机调试模式下，只要配置了 newFramework 配置项且 newFramework.enabled 为 true，小程序框架逻辑就会走入新框架，因此请务必使用 >= 5.12 的Lark客户端进行调试，否则小程序会报错，显示为白屏。

使用命令行工具 opdev 依然可以上传和预览配置了 newFramework 的小程序项目。但是 opdev 不再维护，因此在实时预览和真机调试模式下，newFramework 配置项不会生效，在这两种调试模式下小程序的框架逻辑都只会走入旧框架。

在 IDE 中点击编译并模拟器中预览时，只要配置了 newFramework 配置项且 newFramework.enabled 为 true，小程序框架逻辑就会走入新框架，反之走入旧框架，只需要保证使用高版本的调试基础库即可，因为低版本的基础库中不支持新框架。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5b5866477c6bc7f06464cf8c2ad71f90_CZsJzentR5.png?lazyload=true&width=800&height=480)

### minimumSupportedVersion 的作用
假设某小程序开启新框架并进行回归测试时发现了 bug，且自行排查无果后通过技术支持联系开放平台侧，平台侧排查后确定该 bug 需要 JSSDK 修复。  

假设在 5.21（假设的未来的某个Lark客户端版本）的小程序基础库上修复，那么就需要配置 minimumSupportedVersion 为 5.21，只在 >= 5.21 的Lark客户端上开启新框架。
```json 
{
    "newFramework": {
        "enabled": true,
        "minimumSupportedVersion": "5.21"
    }
}
``` 
### 如何判断当前是否为新框架
在 >= 5.12 的Lark客户端的小程序基础库上提供了接口 tt.isNewFramework()，返回 true 即为新框架，返回 false 则为旧框架。
```javascript 
Page({
	onLoad() {
    	console.log('是否为新框架', tt.isNewFramework && tt.isNewFramework())
    }
})
``` 

## 关闭新框架
如果开启新框架并上传发布版本后，发现异常需要回滚，那么回滚至上一个没有开启新框架、即 app.json 中无 newFramework 配置的版本即可。   
或者发布一个 app.json 不配置 newFramework 的版本。
