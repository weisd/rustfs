# RustFS 



- [ ] 存储系统
    - [ ] 标准S3 api
        - [ ] HeadObject
        - [ ] GetObjectAttributes
        - [ ] CopyObjectPart
        - [ ] PutObjectPart
        - [ ] ListObjectParts
        - [ ] CompleteMultipartUpload
        - [ ] NewMultipartUpload
        - [ ] AbortMultipartUpload
        - [ ] GetObjectACL
        - [ ] PutObjectACL
        - [ ] GetObjectTagging
        - [ ] PutObjectTagging
        - [ ] DeleteObjectTagging 
        - [ ] SelectObjectContent
        - [ ] GetObjectRetention
        - [ ] GetObjectLegalHold
        - [ ] PutObjectLegalHold
        - [ ] GetObjectLambda
        - [ ] GetObject
        - [ ] CopyObject
        - [ ] PutObjectRetention
        - [ ] PutObjectExtract
        - [ ] PutObject
        - [ ] DeleteObject
        - [ ] PostRestoreObject
        - [ ] GetBucketPolicy
        - [ ] GetBucketLifecycle
        - [ ] DeleteBucketLifecycle
        - [ ] GetBucketEncryption
        - [ ] DeleteBucketEncryption
        - [ ] GetBucketObjectLockConfig
        - [ ] GetBucketReplicationConfig
        - [ ] GetBucketVersioning
        - [ ] PutBucketVersioning
        - [ ] GetBucketNotification
        - [ ] PutBucketNotification
        - [ ] ListenNotification
        - [ ] GetBucketTagging
        - [ ] PutBucketTagging
        - [ ] DeleteBucketTagging
        - [ ] ListMultipartUploads
        - [ ] ListObjectsV2
        - [ ] ListObjectVersions/ListObjectVersionsMetadata
        - [ ] GetBucketPolicyStatus
        - [ ] PutBucketLifecycle
        - [ ] PutBucketReplicationConfig
        - [ ] DeleteBucketReplicationConfig
        - [ ] PutBucketPolicy
        - [ ] DeleteBucketPolicy
        - [ ] PostPolicyBucket
        - [ ] PutBucketObjectLockConfig
        - [ ] PutBucket
            - [ ] CheckValidName
            - [ ] Check Region: request vs global
            - [ ] ObjectLockEnable
            - [ ] ForceCreate
            - [ ] ReplicationSys MakeBucketHook
            - [ ] NotificationSys LoadBucketMetadata
            - [ ] EventNotifier Send BucketCreated
        - [ ] GetBucketLocation
            - [x] default
            - [ ] region from config
        - [ ] HeadBucket
        - [ ] DeleteBucket
        - [ ] DeleteMultipleObjects
        - [ ] ListBuckets
        - [ ] ListenNotification

    - [ ] 

- [ ] Config 配置系统
    - [ ] 默认值, 从ENV, config文件加载, 动态加载
    - [x] 从存储系统更新/读取
    - [ ] admin管理接口
    - [ ] console界面

- [ ] Notification 远程通知

- [ ] EventNotifier 事件通知

- [ ] BucketMetadata 桶元数据

- [ ] IAM

- [ ] Policy

- [ ] Lifecycle 

- [ ] BucketSSEConfig

- [ ] BucketObjectLock

- [ ] BucketQuota
    
- [ ] BucketVersioning

- [ ] BucketTarget replication

- [ ] Tier

- [ ] Metrics