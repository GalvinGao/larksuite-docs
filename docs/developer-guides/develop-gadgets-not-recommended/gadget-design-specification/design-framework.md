---
document_id: '6967331158355787782'
directory_id: '6907567266536423425'
title: 设计框架
full_path: /uYjL24iN/ukjNzUjL5YzM14SO2MTN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Design Specification
- Design Framework
document_type: GuideDocumentType
updated_at: 2022-03-22T13:45:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukjNzUjL5YzM14SO2MTN
---

# 设计框架
## 页面框架
### 移动端



![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7ea7f34274f3aa6c941da69f58ee8630_CtPR8THbzu.png?lazyload=true&width=2900&height=2032)


### 桌面端
（后文对尺寸的描述皆为 1 倍图场景）

在会话页右侧以小窗口打开，窗口的宽度固定为 350px，最小高度为 640px

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/653933abc42aa1629cd6589cffbc2ab4_1UFoVGoK5X.png?lazyload=true&width=5800&height=1928)


在工作台中打开，窗口最小宽度为 900px，最小高度为 640px

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a9222f34c26aef08ce63fdd33f25e159_L4J5sovrI1.png?lazyload=true&width=5800&height=1862)

以独立大窗口打开。窗口最小宽度为 640px，最小高度为 480px

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ee79389b58e6642e84a9a97a1b7525ec_WPvC30gFsE.png?lazyload=true&width=5800&height=1968)

以独立小窗口打开。窗口固定尺寸为宽 350px，高700px


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6394520557113bd78d75bb2b71247669_RV7Hq1QdT8.png?lazyload=true&width=2900&height=1810)

## 页面导航
应用可以通过 Navigation Bars（应用导航栏）和 Tab Bars（应用标签栏）两种方式进行应用的页面间导航设计。
###  Navigation Bars（应用导航栏）
#### 标准应用导航
应用导航目的在于告诉用户当前在哪里，并提供小程序的全局设置和退出按钮。当处于小程序的非一级页面时，Navigation Bars 的左侧应根据情况出现“返回”按钮，帮助用户返回上一个页面。应用导航栏支持自定义标题信息。
##### 移动端
移动端应用导航栏由两部分组成，Navigation Bars（应用导航栏） 与 Status Bars（状态栏）。应用导航栏告诉用户当前在哪里，并提供小程序的全局设置和退出操作。当处于小程序的非一级页面时，应用导航栏的左侧应根据情况出现“返回”按钮，帮助用户返回上一个页面。应用导航栏支持自定义标题信息。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/419c0da87ae5806ff4ccf7b953c69494_CnEpqHsgU1.png?lazyload=true&width=2400&height=880)

移动端应用导航栏背景支持自定义颜色。开发者可在满足可用性的前提下，通过修改[配置小程序](/document/uYjL24iN/uEDNuEDNuEDN)中的navigationBarBackgroundColor 参数来设置背景颜色，navigationBarTextStyle 来设置标题颜色。标题颜色支持浅色和深色两种，在浅色背景下建议选择 “Black”，在深色背景下建议选择 “White”。示意图如下： 

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/78b1cbc4931bd29ef6bd173adfa80f10_4e4xmedsDd.png?lazyload=true&width=2400&height=488)

##### 桌面端
桌面端导航栏整体结构和功能与移动端相似，但在不同的场景下会略有差异。

1、在会话中通过小窗口打开小程序，结构与移动端小程序一致。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/722042eb9bd169bdc7b27c4ca8cfbc1a_XEhTw7paj6.png?lazyload=true&width=2400&height=780)

2、通过独立的小窗口打开小程序，与会话中的小窗口的差异在于导航栏高度。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a8546dac19e6c4f7ccaae08c122bf87c_2vaZk1JS2a.png?lazyload=true&width=2400&height=780)

3、在工作台内打开小程序，与小窗口小程序导航的差异在于：导航栏右侧无关闭小程序按钮，增加了机器人消息、分享的功能按钮


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/23dae2ce39c231a665211ac6d8676563_F8Z485jj3c.png?lazyload=true&width=2800&height=720)

4、通过独立的大窗口打开小程序，工作台内打开小程序的导航栏相比，增加了关闭按钮，减少了切换至独立窗口按钮


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e0724c0fb9ff475b830d4b6be1b0c844_3kLQ3sCcF3.png?lazyload=true&width=2800&height=816)

#### 自定义应用导航
除了标准应用导航，我们还提供了自定义应用导航的功能，可以通过修改[小程序配置中的navigationStyle 参数](/document/uYjL24iN/uEDNuEDNuEDN)，将参数设置为“custom”（自定义应用）来启用该功能。在自定义应用导航状态下，小程序窗口只会保留右上角的固定操作入口，其他区域支持开发者根据应用场景进行自定义。
导航栏可自定义的安全区域（下图绿色区域）= 导航栏总宽 - 固定操作入口区（下图红色区域），可以通过[getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM)接口中的 **NavigationBarSafeArea** （客户端 3.35.0 及以上版本支持）获取导航栏安全区域的坐标，来进行自定义元素的布局。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/65a3a992238d2b63aa8bc6471a7a590d_ZiXpl7EqUA.png?lazyload=true&width=2000&height=394)

如下示例，“更多”和“关闭”为固定区的操作入口，绿色区域为自定义区域，当需增加“客服”入口时，则在“更多”左侧依序添加。同样，也可以根据实际需求在自定义区添加搜索框、大标题等内容。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6d57902f887350d453bc0f12fc125128_BWcFXUtvBu.png?lazyload=true&width=2000&height=734)

导航栏图标尺寸统一为 20*20px（一倍图），且每个图标两侧需保留 10px 固定间距，即每个图标需占 40px 的横向空间。当需要在自定义区继续增加图标操作入口时，可按照此布局规则依序添加，以保证导航布局规则统一。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a11636c9db29053b61dd670fcec3d178_SRPkQOPuIB.png?lazyload=true&width=2000&height=474)

#####  移动端
移动端导航栏除了固定操作入口“更多”和“关闭”之外，其他区域支持自定义。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/513f3f81cf839d8156c5cadceba6d212_fh9TvZEnM0.png?lazyload=true&width=2000&height=550)

##### 桌面端
由于桌面端各导航窗口和导航栏右侧 padding 不同，其导航栏安全区域的也会有所差异，以下列出四种窗口的自定义区：

1、在会话中通过小窗口打开小程序

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8cdac948c00771dbbb5ec2d5d85926b3_khiUcvAgg1.png?lazyload=true&width=2000&height=556)

2、通过独立的小窗口打开小程序

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/089516c4ac06366e0027d8566b4d8d93_aVB6Hpqabu.png?lazyload=true&width=2000&height=508)


3、在工作台内以大窗口打开小程序

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6ad1bb7b6830d2e55af829f13f5664e4_4Gycj9N0b0.png?lazyload=true&width=2000&height=844)


4、通过独立的大窗口打开小程序

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7a32aa4dc0bf0064ba6e801bef5d95bd_Cj9HNxrycC.png?lazyload=true&width=2000&height=762)

### Tab Bar（应用标签栏）
Tab Bar 用来展示多个并列的对等内容合集，帮助用户可以快速的在多个页面的内容之间跳转。可以通过小程序的全局设置中的[Tab Bar 设置](/document/uYjL24iN/uEDNuEDNuEDN) 来自定义 Tab Bar 的图标、底色、标签文案以及文案颜色。 

#### 移动端
移动端应用标签栏（又名底部导航栏）位于设备底部。推荐将图标和文字标签配合使用，尤其是在图标没有明确含义的情况下。数量最少两个 Tabs，最多支持 5 个 Tabs。Tabs 上的文字尽量简洁明了，长度控制在 2-6 个字符。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/379610762a235a473e5d23e9c45e001e_XtyTNcfJSw.png?lazyload=true&width=2000&height=880)

#### 桌面端
可通过全局设置中的[ Tab Bar 配置](/document/uYjL24iN/uEDNuEDNuEDN)应用标签栏的布局位置，目前页面框架存在小窗口和大窗口两种类型场景，建议在小窗口场景下位置配置为 “bottom”（底部），在大窗口场景下位置配置为 “top”（顶部），如下所示：

在会话页小窗口和独立小窗口应用下，与移动端一致，采用 “bottom” 状态位于页面底部。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7646b899b4274a93fedb4a06df8bcec2_ttsSETZ1yI.png?lazyload=true&width=3840&height=1940)


在大窗口（独立大窗口以及工作台窗口）场景下，采用 “top” 状态位于应用导航栏下方。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/684f10c79a6f1b2f0ff92aa7f64f47f6_j2luBQoxhW.png?lazyload=true&width=5160&height=1720)

## 页面布局
小程序通过多种空间布局组合方式，例如:增量系统、Keyline、填充、垂直高度，来实现对页面内容的组合排列。来保障内容的一致性、组织性和可访问性。

### 增量系统
Lark小程序推荐采用 4px 为单位网格系统，以实现可扩展性。在多个设备上进行设计时，使用增量系统可以使设计中的所有测量值都可预测且更易扩展。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9a031d4e9542521488513e9c700f0b69_j4dUm580g9.png?lazyload=true&width=4890&height=2738)

### Keylines
Keylines 用来指导页面内视觉元素的对齐，并定义它们到屏幕边缘的距离。由于像素密度和屏幕尺寸的不同，导致在不同设备之间的 Keylines 不同。
![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/5f90cd1f5a065986c432746cae7ede78.png?lazyload=true&width=4600&height=1796)


### 填充
填充是元素之间的水平或垂直空间。填充保持一致性并允许扩展，基于 4px 做增量系统。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cb64a4f478552144b1e8d505223bb077_QSj5a1NpXh.png?lazyload=true&width=3840&height=870)

### 垂直高度
垂直间距是指组件的高度，此高度会影响页面的整体布局，高度应满足 4px 的增量系统。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8da46dda89f95bfa1e4ba116f39edcad_pkwEmLutB9.png?lazyload=true&width=1920&height=880)
