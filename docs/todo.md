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
            - [ ] Extract
            - [ ] Range
            - [ ] If-Match
            - [ ] LifecycleS
            - [ ] Replication
            - [ ] BucketObjectLock
            - [ ] LegalHold,Retention
            - [ ] Checksum
            - [ ] Do GetObject
                - [ ] NsLock
                - [ ] Remote
            - [x] EventNotifier SendEvent 
        - [ ] CopyObject
        - [ ] PutObjectRetention
        - [ ] PutObjectExtract
        - [ ] PutObject
            - [ ] Check BucketQuota
            - [ ] Content-Md5
            - [ ] IfMatch
            - [ ] Check ObjectLock, LegalHold, Retention
            - [x] Compression
            - [ ] Encryption
            - [ ] Do PutObject
                - [ ] Set Parity By StorageClass, availability optimized
                - [x] Filemeta
                - [x] Erasure
                - [x] InlineData
                - [x] Bitrot
                - [ ] NsLock
                - [ ] MRF
            - [ ] Auto Extract
            - [ ] Replicat
            - [ ] EventNotifier SendEvent 
            - [ ] Tier Sweep
        - [ ] DeleteObject
            - [ ] BucketReplication
            - [ ] BucketObjectLock
            - [ ] Retention
            - [ ] Tier Sweep SetTransitionState
            - [ ] Do DeleteObject
                - [ ] Lifecycle
                - [ ] BucketObjectLock
                - [ ] Replication
                - [x] DeleteObjectVersion
            - [ ] NotFound Event
            - [x] EventNotifier SendEvent 
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
            - [x] Do MakeBucket
            - [ ] ReplicationSys MakeBucketHook
            - [ ] NotificationSys LoadBucketMetadata
            - [x] EventNotifier Send BucketCreated
        - [ ] GetBucketLocation
            - [x] default
            - [ ] region from config
        - [x] HeadBucket
        - [ ] DeleteBucket
            - [ ] ForceDelete 
            - [ ] Checkk bucket exist
            - [ ] NsLock
            - [x] Do DeleteBucket
            - [ ] Notification DeleteBucketMetadata
            - [ ] Replication DeleteResyncMetadata, DeleteBucketHook
            - [x] EventNotifier SendEvent BucketRemoved
        - [ ] DeleteMultipleObjects
        - [ ] ListBuckets
            - [x] Do ListBuckets
            - [x] Filter my buckets
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