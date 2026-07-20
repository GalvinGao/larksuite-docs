---
document_id: '6967331158354968582'
directory_id: '6907567266536685569'
title: 反馈
full_path: /uYjL24iN/uIDM5QjLyATO04iMwkDN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Design Specification
- Visual Specifications
- Feedback
document_type: GuideDocumentType
updated_at: 2022-03-22T13:45:29Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDM5QjLyATO04iMwkDN
---

# 反馈
## 模态弹窗
需要用户处理事务，又不希望跳转页面以致打断操作流程时，可以在当前页面正中打开一个浮层，承载相应的操作。

模态窗本身是一个容器，它的样式会有很多种变化，带标题/不带标题（若需要传达的内容很简单一句话就可以表述清楚，则可以不带标题）。模态窗上的操作按钮不应超过 3 个，如果需要传递的内容很少，可以考虑用 Toast 控件替代。
### 移动端

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/09817fc98f0c962c44bdfbe6c26f1434_9malIcPHQ4.png?lazyload=true&width=2570&height=1792)

### 桌面端


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1d11a84a1216a5460b7fb887ed72328c_nXieIttDMz.png?lazyload=true&width=1920&height=616)
## 全局提醒
全局展示操作反馈信息的控件。全局提示可提供成功、警告和错误等反馈信息。在页面顶部居中显示并自动消失，是一种不打断用户操作的轻量级提示方式。

### 移动端

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4f0fa7362902c49f962df56435c1e3a8_p8SioTHsZt.png?lazyload=true&width=2264&height=1784)

### 桌面端

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/660ca1742d81e4fc28b111f8c535aded_QVOOVJUtX5.png?lazyload=true&width=2596&height=416)

## 进度条
展示操作的当前进度，在操作需要较长时间才能完成时，为用户显示该操作的当前进度和状态。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c6b0783e563caa8d403fe5df198ed4d4_zGFwpfsdYG.png?lazyload=true&width=1920&height=640)
