---
document_id: '6967331158356393990'
directory_id: '7122028361538945030'
title: 关联外部选项说明
full_path: /ukTMukTMukTM/uADM4QjLwADO04CMwgDN
breadcrumb:
- Server API
- Approval
- Approval definition
- Associate external options
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:35Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uADM4QjLwADO04CMwgDN
---

# 关联外部选项说明

## 使用说明

### 你在什么情况下能用到？

公司同时使用多个系统（Lark审批、人事系统、销售管理系统），需要将其他系统数据同步到审批表单中作为选项，此时通过配置外部数据源为单选/多选控件的选项，就不需要在多个系统维护同一份数据。

例如：Lark审批发起一个涉及销售的审批，销售同学提交审批时需要填写外部客户名单，名单已经维护在销售管理系统中且经常变动，这时就可以通过配置外部数据为单选/多选的选项，销售同学在提交审批时只需要选择自己跟进的客户，且当销售管理系统中的数据更新时还能同步更新到审批系统中，无需反复维护。

### 可以帮你解决什么问题？
- 一份相同的数据不需要在多个系统重复进行更新、修改，避免重复劳动；
- 不管选项数量有多少，通过配置后，让每个员工只需选择和自己相关的选项。

### 如何三个步骤完成你的配置？
- 需要公司开发同学根据「开发者文档」中的要求开发相应数据接口；
- 审批管理员将开发同学开发的接口填入数据源接口中，校验通过后，审批发起者在发起审批时就可以直接选择接口同步过来的选项；
- 为了确保数据安全，开发者可以设置接口的 token 和 key，管理员配置时填入，确保数据不被外泄（ key 为非必需）。

## 开发文档
编写 http 或 https 接口，同时设置好token和key（token 与 key 格式不限），接口的实现不限语言，token 用于校验请求来源，key 用来加密解密（加密key为非必填），不填则不进行加密。调用方式、返回参数格式及加密解密方式见下文。

### 1.调用方式

如果表单处于编辑状态下，当数据源来源于外部系统的控件时，点击校验数据或用户发起请求时，审批系统将对用户配置的外部数据源接口地址发起 **http 或 https** 请求。 

**请求地址** ：用户配置的请求地址<br>
**请求方式** ：POST<br>
**请求超时时间** ：3秒<br>
**请求 Header** ：<br>
key|value
--|--
Content-Type|application/json
目前审批支持通过 **userid，employeeid以及表单中关联的extra字段** 来请求不同的数据填入到单选、多选控件（当 user_id 和 employee_id 均为空时，返回所有选项）。不同的控件 http 请求入参统一入参格式为：

```js 
{
        "user_id": "123",
        "employee_id": "abc",
        "token":"1e8e999f580e7a202dbe1e5103c5e4c58ecc757e",
        "linkage_params":{
          "key1":"value1", // key1 为联动字段的字段代码，value1为被联动控件值
          "key2":"value2" // key2 为联动字段的字段代码，value2为被联动控件值
        }
} 
```
**请求参数说明** :  
|参数|类型|必须|说明|
|-|-|-|-|
|user_id|String|是|用户 user_id|
|employee_id|String|是|员工 employee_id|
|token|String|是|校验请求是否为合法来源的 token|
|linkage_params|Map|否|联动参数（不带linkage_params时，请返回所有的options）|

注：目前开放平台的概念是user id 等同于 employee id

### 2.返回参数格式

出参加密前的格式:

```js 
{
    "code":0,
    "msg":"success!",
    "data":{
        "result":{
            "options":[
                {
                    "id":"id1",
                    "value":"name1",
                    "isDefault":true
                },
                {
                    "id":"id2",
                    "value":"name2"
                },
                {
                    "id":"id3",
                    "value":"name3"
                }
            ],
            "i18nResources":[
                {
                    "locale":"zh_cn",
                    "isDefault":true,
                    "texts":{
                        "name1":"值1",
                        "name2":"值2",
                        "name3":"值3"
                    }
                },
                {
                    "locale":"en_us",
                    "isDefault":false,
                    "texts":{
                        "name1":"value1",
                        "name2":"value2",
                        "name3":"value3"
                    }
                }
            ]
        }
    }
} 
```

**返回参数说明** :（i18nResources必传） 
|参数|类型|说明|
|-|-|-|
|code|int|错误码，非0表示失败|
|msg|string|返回码的描述|
|data|object|返回业务信息|
|&emsp;∟result|object|请求结果的内容|
|&emsp;&emsp;∟options|list<externalData>|选项列表|
|&emsp;&emsp;∟i18nResources|list<i18nResource>|国际化文案|

**externalData 结构** : 
|参数|类型|说明|
|-|-|-|
|id|string|选项唯一ID|
|value|string|选项（需保证唯一）|
|isDefault|bool|是否为默认选项|
  **i18nResource结构** : 
|参数|类型|说明|
|-|-|-|
|locale|string|语言（ zh_cn 为中文，en_us 为英文，ja_jp为日文 ）
|isDefault|bool|是否为默认选项
|texts|map[string]string|国际化文案 map，key 为国际化选项的唯一值，不同语言环境下，此值是相同的值，value为此语言环境下的文案

<br>
  
出参加密后的格式为（将 result 内容加密并转为 base64 输出，未配置key直接明文返回即可）

```js 
{
    "code":0,
    "msg":"success!",
    "data":{
        "result":"tKqgkBNFEzakJAeS/ySKS7j7YoX2rKVuzLJbG44xHsz0eHaqLx6ZLsAQ/ljfK9mDi0F/32UVXM3gUQaczHbR2upD/EStb+O26FApdvNKm0yvKG0WrhFIe7UCMkrxPnegBqqgqcMHLCZQZ2uh/2k5dDlhReT6fxm/bAR4ZwgyvvshqudakKigshSK0Aq25IQ0H65PS/5iRHgk2b06sahZuvH6b9yrfBXJqHdhztvPkPW2FkipbvLMrzQdXz+deBm2DTJ5W53f2QKOxk7szaXKOr1+u1MyCIkjldPcAHqPYRiOzx6iXQPJ6hMj7MHex08amm44d5T3Z2jzCoinkGSrhpusTcmhHmQnjDjl51a2LqBlty1L9yHuMaED+al2lTUhlzGHqhITCQBJLZraOkXYcR6oOXAV3gP4towZw5G/zeeEtXYZvWUvTZ9F3UAXM4jP"
    }
} 
```

**返回参数说明** : 
|参数|类型|说明|
|-|-|-|
|code|int|错误码，非0表示失败|
|msg|string|返回码的描述|
|data|string|返回业务信息|
|&emsp;∟result|string|请求结果加密后转为 base64 的内容|
<br>
### 3.加密解密方式

以下为Golang加密代码

```js 
//AES CBC 加密
func CBCEncrypter(buf []byte, keyStr string) ([]byte, error) {
	key := sha256.Sum256([]byte(keyStr))
	plaintext := standardizeDataEn(buf)

	if len(plaintext)%aes.BlockSize != 0 {
		return nil, errors.New("plaintext is not a multiple of the block size")
	}

	block, err := aes.NewCipher(key[:sha256.Size])
	if err != nil {
		return nil, err
	}

	ciphertext := make([]byte, aes.BlockSize+len(plaintext))
	iv := ciphertext[:aes.BlockSize]
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		return nil, err
	}

	mode := cipher.NewCBCEncrypter(block, iv)
	mode.CryptBlocks(ciphertext[aes.BlockSize:], plaintext)

	return ciphertext, nil
}

func standardizeDataEn(data []byte) []byte {
	appendingLen := aes.BlockSize - (len(data) % aes.BlockSize)
	sd := make([]byte, len(data)+appendingLen)
	copy(sd, data)
	for i := 0; i < appendingLen; i++ {
		sd[i+len(data)] = byte(appendingLen)
	}
	return sd
}
```

<br>
 以下为Java加密代码示例：
  
 
```js 
 public String CBCEncrypter(String key, String source){
        try {
            MessageDigest messageDigest = MessageDigest.getInstance("SHA-256");
            messageDigest.reset();
            messageDigest.update(key.getBytes());

            SecretKeySpec skeySpec = new SecretKeySpec(messageDigest.digest(), "AES");
            Cipher cipher = Cipher.getInstance("AES/CBC/PKCS5Padding");//"算法/模式/补码方式"
            byte[] sSrcBytes = source.getBytes();
            byte[] newSrc =  new byte[sSrcBytes.length + 16];
            byte[] cSrc = new byte[16];
            System.arraycopy(cSrc, 0, newSrc, 0, cSrc.length);
            System.arraycopy(sSrcBytes, 0, newSrc, 16, sSrcBytes.length);
            IvParameterSpec iv = new IvParameterSpec(cSrc);//使用CBC模式，需要一个向量iv，可增加加密算法的强度
            cipher.init(Cipher.ENCRYPT_MODE, skeySpec, iv);
            byte[] encrypted = cipher.doFinal(newSrc);
            return Base64.getEncoder().encodeToString(encrypted);//此处使用BASE64做转码功能，同时能起到2次加密的作用。
        } catch (Exception e) {
            //handle Exception
        }
        return null;
    } 
```


<br>
以下为解密代码


```js 
//AES CBC解密
func CBCDecrypter(buf []byte, keyStr string) ([]byte, error) {
	key := sha256.Sum256([]byte(keyStr))
	if len(buf)%aes.BlockSize != 0 {
		return nil, errors.New("plaintext is not a multiple of the block size")
	}
	block, err := aes.NewCipher(key[:sha256.Size])
	if err != nil {
		return nil, err
	}
	ciphertext := make([]byte, aes.BlockSize+len(buf))
	iv := ciphertext[:aes.BlockSize]
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		return nil, err
	}
	mode := cipher.NewCBCDecrypter(block, iv)
	mode.CryptBlocks(ciphertext[aes.BlockSize:], buf)
	ciphertext = ciphertext[32:]

	plain := standardizeDataDe(ciphertext)
	return plain, nil
}

func standardizeDataDe(origData []byte) []byte {
	length := len(origData)
	unpadding := int(origData[length-1])
	if unpadding > length {
		return nil
	}
	return origData[:(length - unpadding)]
}

func RandKey256() (string, error) {
	key := make([]byte, 32)

	if _, err := rand.Read(key); err != nil {
		return "", err
	} else {
		return  string(key), nil
	}
} 
```


