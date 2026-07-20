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

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">Lark 扫码在线预览</md-th>
      <md-th style="width: 50%;">示例代码</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
     <md-tr>
      <md-td>![图片名称](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/acb2395e3e575b81f2352bd209525b10.png?height=155&lazyload=true&width=155)</md-td>
      <md-td>
- 下载示例代码：[microapp-demo.zip](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c73e19d3c910e185c0b76831e8773004_PP9jXkOARb.zip) 
- 导入并调试示例代码：[使用说明](/document/uYjL24iN/uYDM04iNwQjL2ADN)</md-td>


    </md-tr>
</md-tbody>
</md-table>
::: 

## 开放接口
#### 登录

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[login](/document/uYjL24iN/uYzMuYzMuYzM)</md-td>
      <md-td>获取临时登录凭证</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[checkSession](/document/uYjL24iN/ukTMx4SOxEjL5ETM)</md-td>
      <md-td>检查用户当前的 session 状态是否有效</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 用户信息

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[enterProfile](/document/uYjL24iN/ucDM04yNwQjL3ADN)</md-td>
      <md-td>打开个人信息主页</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getUserInfo](/document/uYjL24iN/ucjMx4yNyEjL3ITM)</md-td>
      <md-td>获取已登录用户的基本信息或特殊信息</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 聊天

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[enterChat](/document/uYjL24iN/ukDM04SOwQjL5ADN)</md-td>
      <md-td>打开指定会话</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[toggleChat](/document/uYjL24iN/ugDM04COwQjL4ADN/toggleChat)</md-td>
      <md-td>侧边栏形式打开或关闭会话</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[chooseChat](/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN)</md-td>
      <md-td>打开用户会话列表选择会话，调用前确保用户已经登入</md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[getChatInfo](/document/uYjL24iN/uEDN2UjLxQjN14SM0YTN)</md-td>
      <md-td>获取某个会话的信息</md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[getBlockActionSourceDetail](/document/getBlockActionSourceDetail)</md-td>
      <md-td>支持从block action点击进入应用后，获取block对应业务的详细信息</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[enterBot](/document/uYjL24iN/uAjM1EjLwITNx4CMyUTM)</md-td>
      <md-td>打开机器人聊天页面</md-td>
      <md-td><md-version>2.7.0</md-version></md-td>
      <md-td><md-version>2.7.0</md-version></md-td>
      <md-td><md-version>2.7.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[sendMessageCard](/document/uYjL24iN/uUjN5UjL1YTO14SN2kTN)</md-td>
      <md-td>发送消息卡片到指定会话</md-td>
      <md-td><md-version>3.19.0</md-version></md-td>
      <md-td><md-version>3.19.0</md-version></md-td>
      <md-td><md-version>3.19.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[onChatBadgeChange](/document/uYjL24iN/uQDN2UjL0QjN14CN0YTN)</md-td>
      <md-td>监听某个群未读消息数变化，确保用户已经登录</md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[offChatBadgeChange](/document/uYjL24iN/ugDM04COwQjL4ADN/offchatbadgechange)</md-td>
      <md-td>取消监听某个群未读消息数变化</md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
      <md-td><md-version>3.10.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 联系人

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[chooseContact](/document/uYjL24iN/uMTM04yMxQjLzEDN)</md-td>
      <md-td>打开用户联系人选择列表</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::


#### 设置

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[openSetting](/document/uYjL24iN/uUzMx4SNzEjL1MTM)</md-td>
      <md-td>打开设置页面，展示用户设置（包括授予和拒绝）过的权限，并返回用户设置过的授权结果</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getSetting](/document/uYjL24iN/uQzMx4CNzEjL0MTM)</md-td>
      <md-td>获取用户设置（包括授予和拒绝）过的权限</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 分享

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[share](/document/uYjL24iN/ugDM04COwQjL4ADN/thirdShare)</md-td>
      <md-td>分享内容到三方应用</md-td>
      <md-td><md-version>3.47.0</md-version></md-td>
      <md-td><md-version>3.47.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>
    <md-tr>
      <md-td>[showShareMenu](/document/uYjL24iN/ugjN24CO2YjL4YjN)</md-td>
      <md-td>显示当前页面的分享按钮</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[hideShareMenu](/document/uYjL24iN/ukjN24SO2YjL5YjN)</md-td>
      <md-td>隐藏当前页面的分享按钮</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>

</md-tbody>
</md-table>
:::
#### 授权

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>    <md-tr>
      <md-td>[showShareMenu](/document/uYjL24iN/ugjN24CO2YjL4YjN)</md-td>
      <md-td>显示当前页面的分享按钮</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[hideShareMenu](/document/uYjL24iN/ukjN24SO2YjL5YjN)</md-td>
      <md-td>隐藏当前页面的分享按钮</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>
  <md-tbody>
		

    <md-tr>
      <md-td>[authorize](/document/uYjL24iN/ugzMx4COzEjL4MTM)</md-td>
      <md-td>向用户发出设置权限请求。如果该权限用户没有设置过，会弹窗咨询用户是否授予；如果该权限用户拒绝授予，会打开设置页面(appBadge权限除外)；如果该权限用户同意授予，会直接返回成功</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### Lark启动参数

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getLaunchOptionsSync](/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN)</md-td>
      <md-td>获取小程序启动时的参数</md-td>
      <md-td><md-version>3.22</md-version></md-td>
      <md-td><md-version>3.22</md-version></md-td>
      <md-td><md-version>3.22</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[getHostLaunchQuery](/document/uYjL24iN/ugzM4UjL4MDO14COzgTN)</md-td>
      <md-td>获取启动时传入的参数</md-td>
      <md-td><md-version>3.1</md-version></md-td>
      <md-td><md-version>3.1</md-version></md-td>
      <md-td><md-version>3.1</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 安全密码验证

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[startPasswordVerify](/document/uYjL24iN/ugTO3IjL4kzNy4CO5cjM)</md-td>
      <md-td>调起二次验证Lark安全密码的输入界面</md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 系统认证

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[startDeviceCredential](/document/uYjL24iN/uIDN14iM0UjLyQTN)</md-td>
      <md-td>打开系统解锁界面</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 水印

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[checkWatermark](/document/uYjL24iN/ukTM1EjL5ETNx4SOxUTM)</md-td>
      <md-td>查看宿主是否显示了全局水印</md-td>
      <md-td><md-version>2.7.0</md-version></md-td>
      <md-td><md-version>2.7.0</md-version></md-td>
      <md-td><md-version>2.7.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 邮件

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[mailto](/document/uYjL24iN/uAjNwEjLwYDMx4CM2ATM)</md-td>
      <md-td>调用系统发送邮件</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 界面
#### 交互反馈

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[showActionSheet](/document/uYjL24iN/ukDNy4SO0IjL5QjM)</md-td>
      <md-td>显示操作菜单</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[showModal](/document/uYjL24iN/ugDNy4CO0IjL4QjM)</md-td>
      <md-td>显示模态弹窗</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[showPrompt](/document/uYjL24iN/uYTO4UjL2kDO14iN5gTN)</md-td>
      <md-td>展示可输入内容的弹窗</md-td>
      <md-td><md-version>3.17.0</md-version></md-td>
      <md-td><md-version>3.17.0</md-version></md-td>
      <md-td><md-version>3.17.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[showLoading](/document/uYjL24iN/uMDNy4yM0IjLzQjM)</md-td>
      <md-td>显示灰色背景的 loading 提示框</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[hideLoading](/document/uYjL24iN/uYDNy4iN0IjL2QjM)</md-td>
      <md-td>隐藏 loading 提示框</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[showToast](/document/uYjL24iN/ugzMy4COzIjL4MjM)</md-td>
      <md-td>显示灰色背景的消息提示框</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[hideToast](/document/uYjL24iN/ukzMy4SOzIjL5MjM)</md-td>
      <md-td>隐藏灰色背景的消息提示框</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### Tab Bar

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[showTabBar](/document/uYjL24iN/uATN04CM1QjLwUDN)</md-td>
      <md-td>显示 tabBar</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[hideTabBar](/document/uYjL24iN/ukDN04SO0QjL5QDN)</md-td>
      <md-td>隐藏 tabBar</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[setTabBarItem](/document/uYjL24iN/uETN04SM1QjLxUDN)</md-td>
      <md-td>动态设置 tabBar 某一项的内容</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[addTabBarItem](/document/uYjL24iN/uQjM04CNyQjL0IDN/addtabbaritem)</md-td>
      <md-td>当前小程序的tab bar数量进行增加调整</md-td>
      <md-td><md-version>5.1.0</md-version></md-td>
      <md-td><md-version>5.1.0</md-version></md-td>
      <md-td><md-version>5.1.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[removeTabBarItem](/document/uYjL24iN/uQjM04CNyQjL0IDN/removetabbaritem)</md-td>
      <md-td>删除tab bar的目标item</md-td>
      <md-td><md-version>4.2.0</md-version></md-td>
      <md-td><md-version>4.2.0</md-version></md-td>
      <md-td><md-version>4.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[setTabBarStyle](/document/uYjL24iN/uITN04iM1QjLyUDN)</md-td>
      <md-td>动态设置 tabBar 的整体样式</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[setTabBarBadge](/document/uYjL24iN/uUjM04SNyQjL1IDN)</md-td>
      <md-td>为 tabBar 某一项的右上角添加文本</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[removeTabBarBadge](/document/uYjL24iN/ucjM04yNyQjL3IDN)</md-td>
      <md-td>移除 tabBar 某一项右上角的文本</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[showTabBarRedDot](/document/uYjL24iN/uYjM04iNyQjL2IDN)</md-td>
      <md-td>显示 tabBar 某一项的右上角的红点，可以使用底部标签栏红点给予用户提示</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[hideTabBarRedDot](/document/uYjL24iN/ugjM04COyQjL4IDN)</md-td>
      <md-td>隐藏 tabBar 某一项的右上角的红点</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 导航栏

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[setNavigationBarTitle](/document/uYjL24iN/uATNy4CM1IjLwUjM)</md-td>
      <md-td>设置导航栏标题</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 窗口

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[setWindowSize](/document/uYjL24iN/uEDO3UjLxgzN14SM4cTN/setwindowsize)</md-td>
      <md-td>小程序在 window 和 window-semi 模式下调整独立窗口的大小和位置</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[onWindowResize](/document/uYjL24iN/uADO3UjLwgzN14CM4cTN)</md-td>
      <md-td>监听窗口尺寸变化事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-version>3.13.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[offWindowResize](/document/uYjL24iN/uIDO3UjLygzN14iM4cTN)</md-td>
      <md-td>取消监听窗口尺寸变化事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td><md-version>3.13.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 下拉刷新

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[Page.onPullDownRefresh](/document/uYjL24iN/uQTNy4CN1IjL0UjM)</md-td>
      <md-td>在 Page 中注册下拉刷新的监听方法，当用户触发下拉刷新时会调用</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[startPullDownRefresh](/document/uYjL24iN/uYTNy4iN1IjL2UjM)</md-td>
      <md-td>下拉刷新</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[stopPullDownRefresh](/document/uYjL24iN/ugTNy4CO1IjL4UjM)</md-td>
      <md-td>停止当前页面下拉刷新</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 页面位置

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[pageScrollTo](/document/uYjL24iN/uITNy4iM1IjLyUjM)</md-td>
      <md-td>滚动页面到目标位置</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### Canvas绘图

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[createCanvasContext](/document/uYjL24iN/uMTNy4yM1IjLzUjM)</md-td>
      <md-td>创建并返回对应 canvasId 的绘图上下文</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[canvasToTempFilePath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvastotempfilepath)</md-td>
      <md-td>导出当前画布指定区域，生成图片并返回文件路径</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[canvasPutImageData](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvasputimagedata)</md-td>
      <md-td>更新画布像素数据</md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[canvasGetImageData](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvasgetimagedata)</md-td>
      <md-td>获取画布像素数据</md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.createPattern](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createPattern)</md-td>
      <md-td>创建径向渐变管理对象</md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
      <md-td><md-version>3.45.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.createCircularGradient](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createCircularGradient)</md-td>
      <md-td>创建圆形渐变管理对象</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.translate](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-translate)</md-td>
      <md-td>平移坐标矩阵</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.transform](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-transform)</md-td>
      <md-td>坐标转换矩阵叠加，每一次调用会在乘以前一次的变换矩阵</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.strokeText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeText)</md-td>
      <md-td>绘制文字路径</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.strokeRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-strokeRect)</md-td>
      <md-td>绘制矩形路径，不添加到当前路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.stroke](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-stroke)</md-td>
      <md-td>绘制当前路径</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setTransform](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTransform)</md-td>
      <md-td>设置坐标转换矩阵</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setTextBaseline](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextBaseline)</md-td>
      <md-td>设置字体的对齐基线</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setTextAlign](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setTextAlign)</md-td>
      <md-td>设置字体对齐方式</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setStrokeStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setStrokeStyle)</md-td>
      <md-td>设置绘制线样式</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setShadow](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setShadow)</md-td>
      <md-td>设置阴影</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setMiterLimit](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setMiterLimit)</md-td>
      <md-td>设置线连接点渲染的斜面倾斜程度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setLineWidth](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineWidth)</md-td>
      <md-td>设置线宽</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setLineJoin](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineJoin)</md-td>
      <md-td>设置线连接点样式</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setLineDash](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineDash)</md-td>
      <md-td>设置间断线</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setLineCap](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setLineCap)</md-td>
      <md-td>设置线端点样式</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setGlobalAlpha](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setGlobalAlpha)</md-td>
      <md-td>设置全局不透明度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.setFillStyle](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-setFillStyle)</md-td>
      <md-td>设置填充样式</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.scale](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-scale)</md-td>
      <md-td>缩放坐标点</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.save](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-save)</md-td>
      <md-td>创建新的绘图上下文，并将之前的上下文保存在栈中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.rotate](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rotate)</md-td>
      <md-td>旋转坐标点</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.restore](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-restore)</md-td>
      <md-td>恢复栈中存储的上下文</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.rect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-rect)</md-td>
      <md-td>添加矩形到当前路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.quadraticCurveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-quadraticCurveTo)</md-td>
      <md-td>添加二次贝塞尔曲线到路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.moveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-moveTo)</md-td>
      <md-td>移动绘制点</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.measureText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-measureText)</md-td>
      <md-td>测量文字宽度</md-td>
      <md-td><md-version>3.9.0</md-version></md-td>
      <md-td><md-version>3.9.0</md-version></md-td>
      <md-td><md-version>3.9.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.lineTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-lineTo)</md-td>
      <md-td>移动并添加线段到路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.fillText](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillText)</md-td>
      <md-td>填充文字</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.fillRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fillRect)</md-td>
      <md-td>填充矩形，不添加到当前路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.fill](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-fill)</md-td>
      <md-td>填充当前路径</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.drawImage](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-drawImage)</md-td>
      <md-td>绘制 Image</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.createLinearGradient](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-createLinearGradient)</md-td>
      <md-td>创建线性渐变对象</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.closePath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-closePath)</md-td>
      <md-td>闭合当前路径</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.clip](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clip)</md-td>
      <md-td>剪切当前路径，限制后续的渲染范围</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.clearRect](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-clearRect)</md-td>
      <md-td>清空画布矩形区域</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.bezierCurveTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-bezierCurveTo)</md-td>
      <md-td>添加三次贝塞尔曲线到路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.beginPath](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-beginPath)</md-td>
      <md-td>创建新的子路径</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.arcTo](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arcTo)</md-td>
      <md-td>移动并添加弧线到当前路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.arc](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-arc)</md-td>
      <md-td>添加圆弧到当前路径中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CanvasContext.draw](/document/uYjL24iN/ukjMy4SOyIjL5IjM/canvascontext/canvascontext-draw)</md-td>
      <md-td>将所有的操作绘制到 Canvas 中</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 动画

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[createAnimation](/document/uYjL24iN/uETNy4SM1IjLxUjM)</md-td>
      <md-td>创建一个动画实例 animation。调用实例的方法来描述动画。最后通过动画实例的 export 方法导出动画数据传递给组件的 animation 属性</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.backgroundColor](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_backgroundcolor)</md-td>
      <md-td>设置背景色</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.bottom](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_bottom)</md-td>
      <md-td>设置 bottom 值</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.export](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_export)</md-td>
      <md-td>导出动画队列。export 方法每次调用后会清掉之前的动画操作</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.height](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_height)</md-td>
      <md-td>设置高度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.left](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_left)</md-td>
      <md-td>设置 left 值</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.matrix](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_matrix)</md-td>
      <md-td>同 transform-function matrix</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.matrix3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_matrix3d)</md-td>
      <md-td>同 transform-function matrix3d</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.opacity](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_opacity)</md-td>
      <md-td>设置透明度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.right](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_right)</md-td>
      <md-td>设置 right 值</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.rotate](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotate)</md-td>
      <md-td>从原点顺时针旋转一个角度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.rotate3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotate3d)</md-td>
      <md-td>从 固定 轴顺时针旋转一个角度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.rotateX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotatex)</md-td>
      <md-td>从 X 轴顺时针旋转一个角度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.rotateY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotateY)</md-td>
      <md-td>从 Y 轴顺时针旋转一个角度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.rotateZ](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_rotatez)</md-td>
      <md-td>从 Z 轴顺时针旋转一个角度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.scale](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scale)</md-td>
      <md-td>缩放</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.scale3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scale3d)</md-td>
      <md-td>缩放</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.scaleX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scalex)</md-td>
      <md-td>缩放 X 轴</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.scaleY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scaley)</md-td>
      <md-td>缩放 Y 轴</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.scaleZ](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_scalez)</md-td>
      <md-td>缩放 Z 轴</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.skew](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skew)</md-td>
      <md-td>对 X、Y 轴坐标进行倾斜</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.skewX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skewx)</md-td>
      <md-td>对 X 轴坐标进行倾斜</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.skewY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_skewy)</md-td>
      <md-td>对 Y 轴坐标进行倾斜</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.step](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_step)</md-td>
      <md-td>表示一组动画完成。可以在一组动画中调用任意多个动画方法，一组动画中的所有动画会同时开始，一组动画完成后才会进行下一组动画</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.top](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_top)</md-td>
      <md-td>设置 top 值</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.translate](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translate)</md-td>
      <md-td>平移变换</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.translate3d](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translate3d)</md-td>
      <md-td>对 xyz 坐标进行平移变换</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.translateX](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translatex)</md-td>
      <md-td>对 X 轴平移</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.translateY](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translatey)</md-td>
      <md-td>对 Y 轴平移</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.translateZ](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_translatez)</md-td>
      <md-td>对 Z 轴平移</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Animation.width](/document/uYjL24iN/ucjMy4yNyIjL3IjM/animation/animation_width)</md-td>
      <md-td>设置宽度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### Customized Input

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getCustomizedInput](/document/uYjL24iN/uEDN1EjLxQTNx4SM0UTM)</md-td>
      <md-td>获取全局唯一的customizedInput实例。通过customizedInput显示一个 可定制化的富文本输入框，支持@联系人、插入图片、插入表情、显示用户头像、切换用户头像状态</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CustomizedInput.show](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/show)</md-td>
      <md-td>显示输入框</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CustomizedInput.update](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/update)</md-td>
      <md-td>更新输入框中显示的内容</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CustomizedInput.hide](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/hide)</md-td>
      <md-td>隐藏输入框</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CustomizedInput.onPicSelect](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpicselect)</md-td>
      <md-td>监听连接成功的事件回调</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CustomizedInput.onModelSelect](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onmodelselect)</md-td>
      <md-td>选择pickerView之后触发的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CustomizedInput.onPublish](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onpublish)</md-td>
      <md-td>点击发送按钮触发的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[CustomizedInput.onHide](/document/uYjL24iN/uADN1EjLwQTNx4CM0UTM/customizedinput/onhide)</md-td>
      <md-td>隐藏输入框之后触发的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### Pad

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[togglePadFullScreen](/document/uYjL24iN/uUTOuUTOuUTO/pad/togglepadfullscreen)</md-td>
      <md-td>在当前Pad小程序窗口可以全屏缩放的前提下， 进行全屏缩放状态的切换</md-td>
      <md-td><md-version>4.10.0</md-version></md-td>
      <md-td><md-version>4.10.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getPadDisplayScaleMode](/document/uYjL24iN/uUTOuUTOuUTO/pad/getpaddisplayscalemode)</md-td>
      <md-td>获取当前Pad的小程序窗口缩放状态，当前显示状态 能否进行全屏缩放的切换</md-td>
      <md-td><md-version>4.10.0</md-version></md-td>
      <md-td><md-version>4.10.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 设备
#### 系统信息

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getSystemInfo](/document/uYjL24iN/uQjNx4CN2EjL0YTM)</md-td>
      <md-td>获取系统信息</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getSystemInfoSync](/document/uYjL24iN/uUjNx4SN2EjL1YTM)</md-td>
      <md-td>获取系统信息</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### NFC

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getNFCAdapter](/document/uYjL24iN/ukzM4YjL5MDO24SOzgjN)</md-td>
      <md-td>获取 NFC 实例</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NFCAdapter.getNfcA](/document/uYjL24iN/ugzM4YjL4MDO24COzgjN)</md-td>
      <md-td>获取NfcA实例，实例支持NFC-A (ISO 14443-3A)标准的读写</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NFCAdapter.getMifareClassic](/document/uYjL24iN/uEDN4YjLxQDO24SM0gjN)</md-td>
      <md-td>获取MifareClassic实例，实例支持MIFARE Classic标签的读写</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NFCAdapter.startDiscovery](/document/uYjL24iN/uIDN4YjLyQDO24iM0gjN)</md-td>
      <md-td>开始扫描NFC标签</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NFCAdapter.stopDiscovery](/document/uYjL24iN/uMDN4YjLzQDO24yM0gjN)</md-td>
      <md-td>关闭NFC标签扫描</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NFCAdapter.onDiscovered](/document/uYjL24iN/uUDN4YjL1QDO24SN0gjN)</md-td>
      <md-td>监听 NFC Tag</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NFCAdapter.offDiscovered](/document/uYjL24iN/uQDN4YjL0QDO24CN0gjN)</md-td>
      <md-td>取消监听 NFC Tag</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NfcA.connect](/document/uYjL24iN/ucDN4YjL3QDO24yN0gjN)</md-td>
      <md-td>连接NfcA类型的标签</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NfcA.transceive](/document/uYjL24iN/uITN4YjLyUDO24iM1gjN)</md-td>
      <md-td>发送数据给NFCA类型的标签</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NfcA.close](/document/uYjL24iN/uYDN4YjL2QDO24iN0gjN)</md-td>
      <md-td>断开与NFCA标签之间的连接</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NfcA.getAtqa](/document/uYjL24iN/ugDN4YjL4QDO24CO0gjN)</md-td>
      <md-td>获取ATQA信息</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NfcA.getMaxTransceiveLength](/document/uYjL24iN/ukDN4YjL5QDO24SO0gjN)</md-td>
      <md-td>获取最大传输长度</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NfcA.getSak](/document/uYjL24iN/uATN4YjLwUDO24CM1gjN)</md-td>
      <md-td>获取SAK信息</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[NfcA.setTimeout](/document/uYjL24iN/uETN4YjLxUDO24SM1gjN)</md-td>
      <md-td>设置超时时间</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[MifareClassic.connect](/document/uYjL24iN/uQTN4YjL0UDO24CN1gjN)</md-td>
      <md-td>连接MifareClassic类型的标签</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[MifareClassic.transceive](/document/uYjL24iN/ucTN4YjL3UDO24yN1gjN)</md-td>
      <md-td>发送数据给MifareClassic类型的标签</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[MifareClassic.close](/document/uYjL24iN/uMTN4YjLzUDO24yM1gjN)</md-td>
      <md-td>断开与MifareClassic标签之间的连接</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[MifareClassic.getMaxTransceiveLength](/document/uYjL24iN/uUTN4YjL1UDO24SN1gjN)</md-td>
      <md-td>获取最大传输长度</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[MifareClassic.setTimeout](/document/uYjL24iN/uYTN4YjL2UDO24iN1gjN)</md-td>
      <md-td>设置超时时间</md-td>
      <md-td><md-version>3.38.0</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 蓝牙

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[openBluetoothAdapter](/document/uYjL24iN/ugzNxYjL4cTM24CO3EjN)</md-td>
      <md-td>初始化蓝牙模块</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[closeBluetoothAdapter](/document/uYjL24iN/uYDOxYjL2gTM24iN4EjN)</md-td>
      <md-td>关闭蓝牙模块。调用该方法将断开所有已建立的连接并释放系统资源。建议在使用蓝牙流程后，与 tt.openBluetoothAdapter 成对调用</md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getBluetoothAdapterState](/document/uYjL24iN/uUDOxYjL1gTM24SN4EjN)</md-td>
      <md-td>获取本机蓝牙适配器状态</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[startBluetoothDevicesDiscovery](/document/uYjL24iN/uUzNxYjL1cTM24SN3EjN)</md-td>
      <md-td>开始搜寻附近的蓝牙外围设备</md-td>
      <md-td><md-version>3.44</md-version></md-td>
      <md-td><md-version>3.44</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[stopBluetoothDevicesDiscovery](/document/uYjL24iN/uczNxYjL3cTM24yN3EjN)</md-td>
      <md-td>停止搜寻附近的蓝牙外围设备。若已经找到需要的蓝牙设备并不需要继续搜索时，建议调用该接口停止蓝牙搜索</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getConnectedBluetoothDevices](/document/uYjL24iN/uMDOxYjLzgTM24yM4EjN)</md-td>
      <md-td>根据 uuid 获取处于已连接状态的设备</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getBluetoothDevices](/document/uYjL24iN/uQDOxYjL0gTM24CN4EjN)</md-td>
      <md-td>获取在蓝牙模块生效期间所有已发现的蓝牙设备。包括已经和本机处于连接状态的设备</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onBluetoothDeviceFound](/document/uYjL24iN/ukzNxYjL5cTM24SO3EjN)</md-td>
      <md-td>监听寻找到新设备的事件</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offBluetoothDeviceFound](/document/uYjL24iN/uEDOxYjLxgTM24SM4EjN)</md-td>
      <md-td>取消监听寻找到新设备的事件</md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onBluetoothAdapterStateChange](/document/uYjL24iN/uADOxYjLwgTM24CM4EjN)</md-td>
      <md-td>监听蓝牙适配器状态变化事件</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offBluetoothAdapterStateChange](/document/uYjL24iN/uIDOxYjLygTM24iM4EjN)</md-td>
      <md-td>取消监听蓝牙适配器状态变化事件</md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 低功耗蓝牙

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[setBLEMTU](/document/uYjL24iN/uMTMyYjLzEjM24yMxIjN)</md-td>
      <md-td>设置蓝牙最大传输单元</md-td>
      <md-td><md-version>3.26</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[readBLECharacteristicValue](/document/uYjL24iN/uYTOxYjL2kTM24iN5EjN)</md-td>
      <md-td>读取数据</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[writeBLECharacteristicValue](/document/uYjL24iN/ucTOxYjL3kTM24yN5EjN)</md-td>
      <md-td>写入数据</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getBLEDeviceCharacteristics](/document/uYjL24iN/ukDOxYjL5gTM24SO4EjN)</md-td>
      <md-td>获取读写特征</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>32.5</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[connectBLEDevice](/document/uYjL24iN/ucDOxYjL3gTM24yN4EjN)</md-td>
      <md-td>链接外围设备</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[disconnectBLEDevice](/document/uYjL24iN/ugDOxYjL4gTM24CO4EjN)</md-td>
      <md-td>断开设备连接</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getBLEDeviceServices](/document/uYjL24iN/uATOxYjLwkTM24CM5EjN)</md-td>
      <md-td>获取设备服务</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onBLEConnectionStateChange](/document/uYjL24iN/uUTOxYjL1kTM24SN5EjN)</md-td>
      <md-td>蓝牙连接状态变化</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offBLEConnectionStateChange](/document/uYjL24iN/uMTOxYjLzkTM24yM5EjN)</md-td>
      <md-td>取消监听蓝牙低功耗连接状态的改变事件</md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[notifyBLECharacteristicValueChange](/document/uYjL24iN/uETOxYjLxkTM24SM5EjN)</md-td>
      <md-td>监听特征值数据变化</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onBLECharacteristicValueChange](/document/uYjL24iN/uQTOxYjL0kTM24CN5EjN)</md-td>
      <md-td>监听特征值数据变化</md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td><md-version>3.25</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offBLECharacteristicValueChange](/document/uYjL24iN/uITOxYjLykTM24iM5EjN)</md-td>
      <md-td>取消监听蓝牙低功耗设备的特征值变化事件</md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td><md-version>3.25.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### Beacon

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[startBeaconDiscovery](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/startbeacondiscovery)</md-td>
      <md-td>开始搜索附近的 iBeacon 设备</md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[stopBeaconDiscovery](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/stopbeacondiscovery)</md-td>
      <md-td>停止搜索附近的 iBeacon 设备</md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getBeacons](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/getbeacons)</md-td>
      <md-td>获取所有已搜索到的 iBeacon 设备</md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onBeaconUpdate](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/onbeaconupdate)</md-td>
      <md-td>监听 iBeacon 设备更新事件，仅能注册一个监听</md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offBeaconUpdate](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconupdate)</md-td>
      <md-td>取消监听 iBeacon 设备更新事件</md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onBeaconServiceChange](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/onbeaconservicechange)</md-td>
      <md-td>监听蓝牙适配器状态变化事件</md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offBeaconServiceChange](/document/uYjL24iN/uQTOuQTOuQTO/ibeacon/offbeaconservicechange)</md-td>
      <md-td>取消监听 iBeacon 服务状态变化事件</md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td><md-version>4.6.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 扫码

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[scanCode](/document/uYjL24iN/uYzNx4iN3EjL2cTM)</md-td>
      <md-td>扫描二维码并返回扫描结果</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### Wi-Fi

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getConnectedWifi](/document/uYjL24iN/ugjNx4CO2EjL4YTM)</md-td>
      <md-td>获取设备当前所连的 Wifi</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getWifiStatus](/document/uYjL24iN/uYTN4QjL2UDO04iN1gDN)</md-td>
      <md-td>请求获取 Wi-Fi 开关状态</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getWifiList](/document/uYjL24iN/uUDO4UjL1gDO14SN4gTN)</md-td>
      <md-td>请求获取Wifi 列表</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onGetWifiList](/document/uYjL24iN/uYDO4UjL2gDO14iN4gTN)</md-td>
      <md-td>监听获取到 Wi-Fi 列表数据事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offGetWifiList](/document/uYjL24iN/ucDO4UjL3gDO14yN4gTN)</md-td>
      <md-td>取消监听获取到 Wi-Fi 列表数据事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 剪贴板

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[setClipboardData](/document/uYjL24iN/ugzNx4CO3EjL4cTM)</md-td>
      <md-td>设置系统剪贴板内容</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getClipboardData](/document/uYjL24iN/uczNx4yN3EjL3cTM)</md-td>
      <md-td>获取系统粘贴板数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 网络状态

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getNetworkType](/document/uYjL24iN/uYjNx4iN2EjL2YTM)</md-td>
      <md-td>获取设备当前所处的网络类型</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onNetworkStatusChange](/document/uYjL24iN/ucjNx4yN2EjL3YTM)</md-td>
      <md-td>监听网络状态变化</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getNetworkQualityType](/document/uYjL24iN/uUTNx4SN1EjL1UTM/getnetworkqualitytype)</md-td>
      <md-td>网络评级接口，获取当前设备所处的网络状态</md-td>
      <md-td><md-version>4.9.0</md-version></md-td>
      <md-td><md-version>4.9.0</md-version></md-td>
      <md-td><md-version>5.1.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[onNetworkQualityChange](/document/uYjL24iN/uUTNx4SN1EjL1UTM/onnetworkqualitychange)</md-td>
      <md-td>监听网络质量变化</md-td>
      <md-td><md-version>4.9.0</md-version></md-td>
      <md-td><md-version>4.9.0</md-version></md-td>
      <md-td><md-version>5.1.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 屏幕亮度

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[setScreenBrightness](/document/uYjL24iN/uIjNx4iM2EjLyYTM/set-screen-brightness)</md-td>
      <md-td>设置屏幕亮度</md-td>
      <md-td><md-version>3.42.0</md-version></md-td>
      <md-td><md-version>3.42.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getScreenBrightness](/document/uYjL24iN/uIjNx4iM2EjLyYTM/get-screen-brightness)</md-td>
      <md-td>获取屏幕亮度</md-td>
      <md-td><md-version>3.42.0</md-version></md-td>
      <md-td><md-version>3.42.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[setKeepScreenOn](/document/uYjL24iN/ukzNx4SO3EjL5cTM)</md-td>
      <md-td>设置是否保持常亮状态</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 截屏监听

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[onUserCaptureScreen](/document/uYjL24iN/uMjNwEjLzYDMx4yM2ATM)</md-td>
      <md-td>监听用户主动截屏事件。用户使用系统截屏按键截屏时触发</md-td>
      <md-td><md-version>2.4.0</md-version></md-td>
      <md-td><md-version>2.4.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[offUserCaptureScreen](/document/uYjL24iN/uQjNwEjL0YDMx4CN2ATM)</md-td>
      <md-td>取消监听用户主动截屏事件</md-td>
      <md-td><md-version>2.4.0</md-version></md-td>
      <md-td><md-version>2.4.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 加速度计

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[startAccelerometer](/document/uYjL24iN/ukjNx4SO2EjL5YTM)</md-td>
      <md-td>通知客户端开始监听加速度计数据。具体的数据返回通过注册onAccelerometerChange接口回调方法获取</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[stopAccelerometer](/document/uYjL24iN/uAzNx4CM3EjLwcTM)</md-td>
      <md-td>停止监听加速度计数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onAccelerometerChange](/document/uYjL24iN/uEzNx4SM3EjLxcTM)</md-td>
      <md-td>监听加速度计数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 拨打电话

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[makePhoneCall](/document/uYjL24iN/uUzNx4SN3EjL1cTM)</md-td>
      <md-td>拨打电话</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 震动

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[vibrateShort](/document/uYjL24iN/uADOx4CM4EjLwgTM)</md-td>
      <md-td>使手机发生较短时间的振动</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[vibrateLong](/document/uYjL24iN/uEDOx4SM4EjLxgTM)</md-td>
      <md-td>使手机发生较长时间的振动</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 罗盘

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[startCompass](/document/uYjL24iN/uIzNx4iM3EjLycTM)</md-td>
      <md-td>开始监听罗盘数据。</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[stopCompass](/document/uYjL24iN/uMzNx4yM3EjLzcTM)</md-td>
      <md-td>停止监听罗盘数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[onCompassChange](/document/uYjL24iN/uQzNx4CN3EjL0cTM)</md-td>
      <md-td>监听罗盘数据变化事件，频率：5 次/秒，接口调用后会自动开始监听，可使用 tt.stopCompass 停止监听</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::

## 文件

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[saveFile](/document/uYjL24iN/ugDOz4CO4MjL4gzM)</md-td>
      <md-td>保存临时文件到本地永久目录</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[saveFileAs](/document/uYjL24iN/uQjN3UjL0YzN14CN2cTN)</md-td>
      <md-td>保存文件到本地指定目录</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td><md-version>3.9.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[filePicker](/document/uYjL24iN/uETM04SMxQjLxEDN)</md-td>
      <md-td>打开附件选择列表</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[docsPicker](/document/uYjL24iN/ukTN3UjL5UzN14SO1cTN)</md-td>
      <md-td>打开云文档选择列表</md-td>
      <md-td><md-version>3.12.0</md-version></md-td>
      <md-td><md-version>3.12.0</md-version></md-td>
      <md-td><md-version>3.13.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[openDocument](/document/uYjL24iN/ukTN24SO1YjL5UjN)</md-td>
      <md-td>在新页面打开文档</md-td>
      <md-td><md-version>2.6.0</md-version></md-td>
      <md-td><md-version>2.6.0</md-version></md-td>
      <md-td><md-version>2.6.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[getFileSystemManager](/document/uYjL24iN/uETOuETOuETO/tt_get_file_system_manager)</md-td>
      <md-td>获取全局唯一的文件管理器</md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>

		

    <md-tr>
      <md-td>[FileSystemManager.readFile](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file)</md-td>
      <md-td>读取本地文件内容</md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[FileSystemManager.stat](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_stat)</md-td>
      <md-td>获取本地文件 Stats 对象</md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>
		

    <md-tr>
      <md-td>[Stats.isDirectory](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_directory)</md-td>
      <md-td>判断当前文件是否一个目录</md-td>
      <md-td><md-version>4.11.0</md-version></md-td>
      <md-td><md-version>4.11.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[Stats.isFile](/document/uYjL24iN/uETOuETOuETO/stat/stats_is_file)</md-td>
      <md-td>判断当前文件是否一个普通文件</md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td><md-version>4.11</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 媒体
#### 图片

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[saveImageToPhotosAlbum](/document/uYjL24iN/uUTMx4SNxEjL1ETM)</md-td>
      <md-td>保存图片到系统相册</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[chooseImage](/document/uYjL24iN/uMTMx4yMxEjLzETM)</md-td>
      <md-td>从系统相册中选择图片，或使用相机拍摄图片</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[previewImage](/document/uYjL24iN/uMDOx4yM4EjLzgTM)</md-td>
      <md-td>预览一组图片</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[compressImage](/document/uYjL24iN/uMjN24yM2YjLzYjN)</md-td>
      <md-td>压缩图片接口，可选压缩质量</md-td>
      <md-td><md-version>2.0.0</md-version></md-td>
      <md-td><md-version>2.0.0</md-version></md-td>
      <md-td><md-version>2.0.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[getImageInfo](/document/uYjL24iN/ugjNwEjL4YDMx4CO2ATM)</md-td>
      <md-td>获取图片信息</md-td>
      <md-td><md-version>2.4.0</md-version></md-td>
      <md-td><md-version>2.4.0</md-version></md-td>
      <md-td><md-version>2.4.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 视频

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[saveVideoToPhotosAlbum](/document/uYjL24iN/ucDOx4yN4EjL3gTM)</md-td>
      <md-td>保存视频到系统相册</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[chooseVideo](/document/uYjL24iN/uEjMx4SMyEjLxITM)</md-td>
      <md-td>从系统相册中选择视频，或使用相机拍摄视频</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[chooseMedia](/document/uYjL24iN/uITMx4iMxEjLyETM/choosemedia)</md-td>
      <md-td>拍摄或从系统相册中选择图片或视频</md-td>
      <md-td><md-version>4.7.0</md-version></md-td>
      <md-td><md-version>4.7.0</md-version></md-td>
      <md-td><md-version>4.7.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[createVideoContext](/document/uYjL24iN/uITMx4iMxEjLyETM/createvideocontext)</md-td>
      <md-td>创建 VideoContext 实例，通过 id 跟一个 video 组件绑定，操作对应的 video 组件</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[VideoContext.play](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/play)</md-td>
      <md-td>播放视频</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[VideoContext.pause](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/pause)</md-td>
      <md-td>暂停视频</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[VideoContext.stop](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/stop)</md-td>
      <md-td>停止视频</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[VideoContext.seek](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/seek)</md-td>
      <md-td>跳转到指定位置</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[VideoContext.requestFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/requestfullscreen)</md-td>
      <md-td>进入全屏</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[VideoContext.exitFullScreen](/document/uYjL24iN/uITMx4iMxEjLyETM/videocontext/exitfullscreen)</md-td>
      <md-td>退出全屏</md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td><md-version>4.3.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 音频

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[createInnerAudioContext](/document/uYjL24iN/uUDOx4SN4EjL1gTM)</md-td>
      <md-td>创建innerAudioContext实例，通过它能够操作音频播放</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.play](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/play)</md-td>
      <md-td>播放音频</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.pause](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/pause)</md-td>
      <md-td>暂停。暂停后的音频再播放会从暂停处开始播放</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.stop](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/stop)</md-td>
      <md-td>停止。停止后的音频再播放会从头开始播放</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.seek](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/seek)</md-td>
      <md-td>跳转到指定位置</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.destroy](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/destroy)</md-td>
      <md-td>销毁当前实例</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onCanplay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/oncanplay)</md-td>
      <md-td>音频进入可以播放状态时触发回调函数</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offCanplay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offcanplay)</md-td>
      <md-td>取消监听 Canplay 事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onPlay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onPlay)</md-td>
      <md-td>监听音频播放事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offPlay](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offplay)</md-td>
      <md-td>取消监听音频播放事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onPause](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onPause)</md-td>
      <md-td>监听音频暂停事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offPause](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offpause)</md-td>
      <md-td>取消监听音频暂停事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onStop](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onstop)</md-td>
      <md-td>监听音频停止事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offStop](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offstop)</md-td>
      <md-td>取消监听音频停止事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onEnded](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onended)</md-td>
      <md-td>监听音频自然播放至结束的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offEnded](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offEnded)</md-td>
      <md-td>取消监听音频自然播放至结束的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onTimeUpdate](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onTimeUpdate)</md-td>
      <md-td>监听音频播放进度更新事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offTimeUpdate](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offtimeupdate)</md-td>
      <md-td>取消监听音频播放进度更新事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onError](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onerror)</md-td>
      <md-td>监听音频播放错误事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offError](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offerror)</md-td>
      <md-td>取消监听音频播放错误事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onWaiting](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onwaiting)</md-td>
      <md-td>监听音频加载中事件。当音频因为数据不足，需要停下来加载时会触发</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offWaiting](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offwaiting)</md-td>
      <md-td>取消监听音频加载中事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onSeeking](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onseeking)</md-td>
      <md-td>监听音频进行跳转操作的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offSeeking](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeking)</md-td>
      <md-td>取消监听音频进行跳转操作的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.onSeeked](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/onSeeked)</md-td>
      <md-td>监听音频完成跳转操作的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[InnerAudioContext.offSeeked](/document/uYjL24iN/uETMx4SMxEjLxETM/inneraudiocontext/offseeked)</md-td>
      <md-td>取消监听音频完成跳转操作的事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### 录音

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getRecorderManager](/document/uYjL24iN/uQDOx4CN4EjL0gTM)</md-td>
      <md-td>获取全局唯一的recorderManager。通过recorderManager进行录音操作和管理</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.start](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/start)</md-td>
      <md-td>开始录音</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.pause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/pause)</md-td>
      <md-td>暂停录音</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.resume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/resume)</md-td>
      <md-td>继续录音</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.stop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/stop)</md-td>
      <md-td>停止录音</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.onStart](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstart)</md-td>
      <md-td>监听录音开始事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.onPause](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onpause)</md-td>
      <md-td>监听录音暂停事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.onResume](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onresume)</md-td>
      <md-td>监听录音继续事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.onStop](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onstop)</md-td>
      <md-td>监听录音结束事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.onFrameRecorded](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onframerecorded)</md-td>
      <md-td>监听已录制完指定帧大小的文件事件。如果设置了 frameSize，则会回调此事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RecorderManager.onError](/document/uYjL24iN/uATMx4CMxEjLwETM/recordermanager/onerror)</md-td>
      <md-td>监听录音错误事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 导航

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[navigateTo](/document/uYjL24iN/uYTOz4iN5MjL2kzM)</md-td>
      <md-td>跳转到指定页面。跳转后原页面保留</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[navigateBack](/document/uYjL24iN/uADM04CMwQjLwADN)</md-td>
      <md-td>返回上一级页面（或上N级页面）。可通过 getCurrentPages 获取当前的页面栈，决定需要返回几层</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[redirectTo](/document/uYjL24iN/ucTOz4yN5MjL3kzM)</md-td>
      <md-td>关闭当前页面，跳转到指定页面</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[switchTab](/document/uYjL24iN/ukTOz4SO5MjL5kzM)</md-td>
      <md-td>跳转到指定 TabBar 页面，并关闭其他所有非 TabBar 页面</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[reLaunch](/document/uYjL24iN/uEDM04SMwQjLxADN)</md-td>
      <md-td>关闭所有当前页面，打开指定页面</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[exitMiniProgram](/document/uYjL24iN/uATN4IjLwUDOy4CM1gjM)</md-td>
      <md-td>退出当前小程序</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[openSchema](/document/uYjL24iN/ukzN4IjL5cDOy4SO3gjM)</md-td>
      <md-td>跳转到小程序以外的应用（如云文档、网页等）</md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
      <md-td><md-version>3.1.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 网络
#### WebSocket

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[connectSocket](/document/uYjL24iN/ugDMx4COwEjL4ATM)</md-td>
      <md-td>创建一个 WebSocket 连接实例，并通过返回的 socketTask 操作该连接</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[SocketTask.send](/document/uYjL24iN/ugDOugDOugDO/sockettask/send)</md-td>
      <md-td>通过 WebSocket 连接发送数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[SocketTask.close](/document/uYjL24iN/ugDOugDOugDO/sockettask/close)</md-td>
      <md-td>关闭 WebSocket 连接</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[SocketTask.onOpen](/document/uYjL24iN/ugDOugDOugDO/sockettask/onopen)</md-td>
      <md-td>监听 WebSocket 连接打开事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[SocketTask.onClose](/document/uYjL24iN/ugDOugDOugDO/sockettask/onclose)</md-td>
      <md-td>监听 WebSocket 连接关闭事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[SocketTask.onMessage](/document/uYjL24iN/ugDOugDOugDO/sockettask/onmessage)</md-td>
      <md-td>监听 WebSocket 接受到服务器的消息事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
#### HTTP

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[request](/document/uYjL24iN/uIDMx4iMwEjLyATM)</md-td>
      <md-td>发起一个 HTTP 请求</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[RequestTask.abort](/document/uYjL24iN/ugDNugDNugDN/requesttask/abort)</md-td>
      <md-td>中断请求任务</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[downloadFile](/document/uYjL24iN/ucDMx4yNwEjL3ATM)</md-td>
      <md-td>下载网络文件到本地临时目录</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[DownloadTask.onProgressUpdate](/document/uYjL24iN/ugDNugDNugDN/downloadfile/onprogressupdate)</md-td>
      <md-td>downloadFile的调用结果在通过回调传递的同时会返回一个downloadTask对象，通过onProgressUpdate方法监听下载进度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[DownloadTask.abort](/document/uYjL24iN/ugDNugDNugDN/downloadfile/abort)</md-td>
      <md-td>调用downloadFile时，会返回一个downloadTask对象，可以通过该对象的abort方法中断请求任务</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[uploadFile](/document/uYjL24iN/uYDMx4iNwEjL2ATM)</md-td>
      <md-td>将本地文件上传到网络</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[UploadTask.onProgressUpdate](/document/uYjL24iN/ugDNugDNugDN/uploadtask/onprogressupdate)</md-td>
      <md-td>调用uploadFile时，会返回一个uploadTask对象，通过onProgressUpdate方法监听下载进度</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[UploadTask.abort](/document/uYjL24iN/ugDNugDNugDN/uploadtask/abort)</md-td>
      <md-td>调用uploadFile时，会返回一个uploadTask对象，可以通过该对象的abort方法中断请求任务</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 地理位置

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[openLocation](/document/uYjL24iN/uQTOz4CN5MjL0kzM)</md-td>
      <md-td>使用客户端内置地图查看位置</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[chooseLocation](/document/uYjL24iN/uUDN1EjL1QTNx4SN0UTM)</md-td>
      <md-td>打开地图选择位置</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getLocation](/document/uYjL24iN/uUTOz4SN5MjL1kzM)</md-td>
      <md-td>获取设备当前的地理位置</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 数据缓存

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[setStorage](/document/uYjL24iN/uETOx4SM5EjLxkTM)</md-td>
      <md-td>以「键值对」的形式设置本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[setStorageSync](/document/uYjL24iN/uITOx4iM5EjLykTM)</md-td>
      <md-td>以「键值对」的形式设置本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getStorage](/document/uYjL24iN/ukDOx4SO4EjL5gTM)</md-td>
      <md-td>获取本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getStorageSync](/document/uYjL24iN/uATOx4CM5EjLwkTM)</md-td>
      <md-td>获取本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[removeStorage](/document/uYjL24iN/uMTOx4yM5EjLzkTM)</md-td>
      <md-td>删除本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[removeStorageSync](/document/uYjL24iN/uQTOx4CN5EjL0kTM)</md-td>
      <md-td>删除本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[clearStorage](/document/uYjL24iN/uUTOx4SN5EjL1kTM)</md-td>
      <md-td>清理全部本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[clearStorageSync](/document/uYjL24iN/uYTOx4iN5EjL2kTM)</md-td>
      <md-td>清理全部本地缓存数据</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getStorageInfo](/document/uYjL24iN/ucTOx4yN5EjL3kTM)</md-td>
      <md-td>获取本地缓存数据的相关信息</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[getStorageInfoSync](/document/uYjL24iN/ugTOx4CO5EjL4kTM)</md-td>
      <md-td>获取本地缓存数据的相关信息</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## TTML

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[createSelectorQuery](/document/uYjL24iN/uYjN24iN2YjL2YjN)</md-td>
      <md-td>获取一个 SelectorQuery 对象实例</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>

		

    <md-tr>
      <md-td>[SelectorQuery.in](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/in)</md-td>
      <md-td>将选择器的选取范围更改为自定义组件 component 内（初始时，选择器仅选取页面范围的节点，不会选取任何自定义组件中的节点）</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[SelectorQuery.select](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/select)</md-td>
      <md-td>在当前页面下选择第一个匹配选择器 selector 的节点，返回一个 NodesRef 对象实例，可以用于获取节点信息。  selector 类似于 CSS 的选择器，其中移动端只支持 ID 选择器</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[SelectorQuery.selectAll](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectall)</md-td>
      <md-td>在当前页面下选择匹配选择器 selector 的所有节点，返回一个 NodesRef 对象实例，可以用于获取节点信息。  selector 类似于 CSS 的选择器，同 select</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[SelectorQuery.selectViewport](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/selectviewport)</md-td>
      <md-td>选择显示区域。可用于获取显示区域的尺寸、滚动位置等信息</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td>**X**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[SelectorQuery.exec](/document/uYjL24iN/uUjN24SN2YjL1YjN/selectorquery/exec)</md-td>
      <md-td>执行所有的请求。请求结果按请求次序构成数组，在callback的第一个参数中返回</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>

		

    <md-tr>
      <md-td>[NodesRef.boundingClientRect](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/boundingclientrect)</md-td>
      <md-td>添加节点的布局位置的查询请求</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[NodesRef.scrollOffset](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/scrolloffset)</md-td>
      <md-td>添加节点的滚动位置查询请求</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[NodesRef.fields](/document/uYjL24iN/uUjN24SN2YjL1YjN/nodesref/fields)</md-td>
      <md-td>获取节点的相关信息</md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
      <md-td><md-version>2.2.0</md-version></md-td>
    </md-tr>

		

    <md-tr>
      <md-td>[createIntersectionObserver](/document/uYjL24iN/ucTNwEjL3UDMx4yN1ATM)</md-td>
      <md-td>创建并返回一个 IntersectionObserver 对象实例</md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
    </md-tr>

		

    <md-tr>
      <md-td>[IntersectionObserver.observe](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/observe)</md-td>
      <md-td>指定目标节点并开始监听相交状态变化情况</md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[IntersectionObserver.relativeTo](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativeto)</md-td>
      <md-td>使用选择器指定一个节点，作为参照区域之一</md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[IntersectionObserver.relativeToViewport](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/relativetoviewport)</md-td>
      <md-td>指定页面显示区域作为参照区域之一</md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
    </md-tr>




    <md-tr>
      <md-td>[IntersectionObserver.disconnect](/document/uYjL24iN/uUjN24SN2YjL1YjN/intersectionobserver/disconnect)</md-td>
      <md-td>停止监听，回调函数将不再触发</md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
      <md-td><md-version>2.3.0</md-version></md-td>
    </md-tr>


</md-tbody>
</md-table>
:::
## 更新

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">API名称</md-th>
      <md-th>说明</md-th>
      <md-th style="width: 18%;">Android</md-th>
       <md-th style="width: 18%;">iOS</md-th>
      <md-th style="width: 18%;">PC</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
		

    <md-tr>
      <md-td>[getUpdateManager](/document/uYjL24iN/uEzM04SMzQjLxMDN)</md-td>
      <md-td>获取全局唯一的版本更新管理器，返回 updateManager 对象，用于管理小程序更新</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>
		

    <md-tr>
      <md-td>[UpdateManager.applyUpdate](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/applyupdate)</md-td>
      <md-td>强制小程序重启并使用新版本。在小程序新版本下载完成后（即收到 onUpdateReady 回调）调用</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[UpdateManager.onCheckForUpdate](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/oncheckforupdate)</md-td>
      <md-td>监听向后台请求检查更新结果事件。客户端在小程序冷启动时自动检查更新，不需由开发者主动触发。线上环境在有更新内容时会触发callback回调</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[UpdateManager.onUpdateFailed](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdatefailed)</md-td>
      <md-td>监听小程序更新失败事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[UpdateManager.onUpdateReady](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/onupdateready)</md-td>
      <md-td>监听小程序有版本更新事件</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>




    <md-tr>
      <md-td>[UpdateManager.triggerCheckUpdate](/document/uYjL24iN/uAzM04CMzQjLwMDN/getUpdateManager/triggercheckupdate)</md-td>
      <md-td>主动触发更新小程序</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
    </md-tr>


</md-tbody>
</md-table>
:::

