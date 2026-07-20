---
document_id: '6967331158356115462'
directory_id: '6907567269107744770'
title: FAQ
full_path: /uYjL24iN/uITM5YjLyETO24iMxkjN
breadcrumb:
- Developer Guides
- Tools and SDKs
- Development Tools
- Development of Gadget (Not Recommended)
- Automated Testing
- FAQ
document_type: GuideDocumentType
updated_at: 2022-11-17T05:57:04Z
source_url: https://open.larksuite.com/document/uYjL24iN/uITM5YjLyETO24iMxkjN
---

# 常见问题

## **Q: case 尚未执行，出现  jest 超时**

**A**: 在超时范围内，开发者工具尚未完全启动。设置一个足够长的启动时间，如下：

```
 beforeAll(async () => {
    _miniProgram = await miniProgram.initMiniProgram();
  }, 5 * 60 * 1000);
  beforeEach(async () => {
    jest.setTimeout(5 * 60 * 1000);
  });
   
```

## **Q:  Cannot read property 'navigateTo' of underfined**

**A**: 出现这个问题大概率是配置文件不对，需要检查 connect 或者 launch 模式下 `socketPath`、`projectPath`、`socketPath` 是否正确

## **Q: 使用深度选择器定位元素时，出现找不到元素的情况**

```
<acomp class=".class1">
    <swiper-item class=".class2">
      <view class=".class3" ></view>
    </swiper-item >
</acomp>
```

```
// 测试伪代码
page.$('.class1 >>> .class2 >>> .class3')
```

**A**: 当类名出现在`swiper-item`这类 [官方组件](/document/uYjL24iN/uEjNuEjNuEjN) 时，会出现路径查找失败的情况，建议去掉 `.class2`，直接使用 `.class1 >>> .class3`

## **Q: 元素定位不到时应该怎么做**

**A**: 在开发者工具中检索元素

   1.  执行 `opdev auto ~/workspace/microapp-demo`打开auto模式调试
   2. 右键检查 - 打开模拟器检查


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fd1c14a6a2cb8088c9343b877a3c1f88_kv4BB9ors6.png)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7924da49237d46332f0f7af53fbb61e6_grtwTSTmTr.png)

3. 在右侧先打开的console中输入 

```
tt.createSelectorQuery().kernelSelect('.approval-list >>> .cascader-panel-item').fields({uid: true}, e => console.log(e)).exec()
// .approval-list >>> .cascader-panel-item 替换成你的元素定位
```

4. 观察左侧的console是否存在该组件，存在会输出一个uid

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2e6547c5fe8a88a4f970892fc80efa96_KHLJ5mC2m3.png)
