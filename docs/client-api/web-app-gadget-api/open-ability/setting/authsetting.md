---
document_id: '6965379541104672773'
directory_id: '6907567269107630082'
title: AuthSetting
full_path: /uYjL24iN/uYzMx4iNzEjL2MTM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Setting
- AuthSetting
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYzMx4iNzEjL2MTM
---

# AuthSetting


名称 | 类型 | 描述
--|--|--
 scope.userInfo | boolean | 是否授权用户信息，对应接口 tt.getUserInfo |
 scope.userLocation | boolean | 是否授权地理位置，对应接口 tt.getLocation, tt.chooseLocation | 
 scope.record | boolean | 是否授权麦克风功能，对应接口 tt.getRecorderManager.start | 
 scope.writePhotosAlbum | boolean | 是否授权保存到相册,  对应接口 tt.saveImageToPhotosAlbum, tt.saveVideoToPhotosAlbum **(PC端无此权限)**| 
 scope.clipboard | boolean | 是否授权剪贴板权限，对应接口 tt.setClipboardData,tt.getClipboardData |
