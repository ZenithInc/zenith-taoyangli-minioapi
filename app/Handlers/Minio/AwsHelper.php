<?php

namespace App\Handlers\Minio;

use Aws\S3\S3Client;
use Aws\S3\MultipartUploader;

class AwsHelper
{
    public $endpoint; //此处注意端口不要写错为9090（若你是按照官方文档安装的minio）
    public $key; //登录http://127.0.0.1:9090/browser 时的账户名
    public $secret; //密码
    public $bucket; //桶名称

    public function __construct()
    {
        $config = config('minio');
        $this->endpoint = $config['endpoint'] ?? '';
        $this->key = $config['key'] ?? '';
        $this->secret = $config['secret'] ?? '';
        $this->bucket = $config['bucket'] ?? '';
        $this->s3Client = new S3Client([
            'version' => 'latest',
            'region'  => 'cn-north-1', //China (Beijing)
            'endpoint' => $this->endpoint,
            'use_path_style_endpoint' => true,
            'credentials' => [
                'key'    => $this->key,
                'secret' => $this->secret,
            ],
        ]);
    }

    /**
     * 判断 bucket是否存在
     * @param $bucket
     * @return bool
     * @author zsh
     */
    public function bucketExist($bucket = null)
    {
        return $this->s3Client->doesBucketExist($bucket);
    }

    /**
     * 判断bucket是否存在的另一种写法
     * @param $bucket
     * @return bool
     */
    public function bucketExists($bucket = null)
    {
        $result = $this->s3Client->listBuckets();
        $names = $result->search('Buckets[].Name');

        if($bucket){
            if(!in_array($bucket, $names)){
                return false;
            }
        }else{
            if(!in_array($this->bucket, $names)){
                return false;
            }
        }

        return true;
    }

    /**
     * 判断 查询对象是否存在
     * @param $bucket
     * @return bool
     */
    public function objectExist($object)
    {
        return $this->s3Client->doesObjectExist($this->bucket, $object);

    }

    /**
     * 上传
     * @param $objectPath
     * @param $objectName
     * @return bool|array
     */
    public function upLoadObject($objectPath, $objectName = null)
    {
        if(!$objectName){
            $objectName = basename($objectPath);
        }

        $uploader = new MultipartUploader($this->s3Client, $objectPath, [
            'bucket' => $this->bucket,
            'key' => $objectName,
        ]);

        $result=$uploader->upload();

        if(isset($result["@metadata"]["statusCode"]) && $result["@metadata"]["statusCode"] == 200){
            return [
                'bucket' => $this->bucket,
                'name' => $objectName,
                'path' => $this->bucket . '/' . $objectName,
                'url'  => $result['ObjectURL'],
            ];
        }else{
            return false;
        }
    }

    /**
     * 上传文件到某文件夹下（上传函数upLoadObject也能实现此功能，可自己研究下）
     * @param $objectPath : 本地文件路径 li:/tmp/test.txt
     * @param $folderPath : minio中桶下的文件夹路径 li:folder/test
     * @param $objectName : 对象名称（重命名用） li: abc.txt
     * @return bool|array
     */
    public function upLoadObjectToFolder($objectPath, $folderPath, $objectName = null)
    {
        if(!$objectName){
            $objectName = basename($objectPath);
        }

        $key = $folderPath . '/' . $objectName;

        $result = $this->s3Client->putObject([
            'Bucket' => $this->bucket,
            'Key' => $key,
            'Body' => file_get_contents($objectPath) //要上传的文件
        ]);

        if(isset($result["@metadata"]["statusCode"]) && $result["@metadata"]["statusCode"] == 200){
            return [
                'bucket' => $this->bucket,
                'name' => $objectName,
                'path' => $this->bucket . '/' . $key
            ];
        }else{
            return false;
        }
    }

    /**
     * 多文件上传
     * @param $objectPathArr
     * @return array
     */
    public function batchUpload($objectPathArr)
    {
        //路径数组
        $pathArr = array();
        $s3 = $this->s3Client;

        foreach ($objectPathArr as $object) {

            if(file_exists($object)){
                //文件扩展名
                $extend = substr(strrchr($object,'.'),1);

                //文件名
                $fileName = date('Ymd') . '-' . uniqid() . '.' . $extend;

                $return = $s3->putObject([
                    'Bucket' => $this->bucket, //存储桶名称
                    'Key' => $fileName, //文件名（包括后缀名）
                    'Body' => file_get_contents($object) //要上传的文件
                ]);

                if (isset($return['@metadata']['statusCode']) && $return['@metadata']['statusCode'] == 200) {

                    $pathArr[] = [
                        'bucket' => $this->bucket,
                        'name' => $fileName,
                        'path' => $this->bucket . '/' . $fileName
                    ];

                } else {
                    //此处可增加日志记录
                    continue;
                }
            }
        }

        return $pathArr;
    }

    /**
     * 复制（重命名文件）
     * @param $sourceObject
     * @param $objectName
     * @return bool|array
     */
    public function copyObject($sourceObject, $objectName = null)
    {
        if(!$objectName){
            $extend = substr(strrchr($sourceObject,'.'),1);
            $objectName = date('Ymd') . '-' . uniqid() . '.' .$extend;
        }

        //源对象需包含桶+key
        $source = '/' . $this->bucket . '/' . $sourceObject;

        $result = $this->s3Client->copyObject([
            'Bucket' => $this->bucket, //存储桶名称
            'CopySource' => $source,
            'Key' => $objectName,
        ]);

        if(isset($result["@metadata"]["statusCode"]) && $result["@metadata"]["statusCode"] == 200){
            return [
                'bucket' => $this->bucket,
                'name' => $objectName,
                'path' => $this->bucket . '/' . $objectName
            ];
        }else{
            return false;
        }
    }

    /**
     * 获取单个对象信息
     * @param $object
     * @return array|mixed|null
     */
    public function getMetaData($object){
        $retrive = $this->s3Client->getObject([
            'Bucket' => $this->bucket,
            'Key'    => $object,
        ]);

        if(!isset($retrive['@metadata'])){
            return [];
        }

        return $retrive['@metadata'];
    }

    /**
     * 获取对象连接
     * @param $object : 对象路径 例：若在文件夹内 aaa/bbb/ccc.png 桶根目录下 ddd.png
     * @param $expires : 有效期
     * @return string
     */
    public function getUrl($object, $expires = null){
        $cmd = $this->s3Client->getCommand('GetObject', [
            'Bucket' => $this->bucket,
            'Key' => $object
        ]);

        if(!$expires){
            $expires = '+1 days';
        }

        $request=$this->s3Client->createPresignedRequest($cmd,$expires);
        $presignedUrl = (string)$request->getUri();

        return $presignedUrl;

        //测试-图片
        //return "<img src='{$presignedUrl}'/>";
    }

    /**
     * 获取多个对象信息
     * @param $data
     * @return array|mixed
     */
    public function getAll($data){
        $param = [
            'Bucket' => $this->bucket
        ];

        if(isset($data['num'])){
            $param['MaxKeys'] = $data['num'];
        }

        if(isset($data['prefix'])){
            $param['Prefix'] = $data['prefix'];
        }

        $retrive = $this->s3Client->listObjects($param);

        if(isset($retrive['Contents'])){
            return $retrive['Contents'];
        }else{
            return [];
        }
    }

    /**
     * 删除单个文件
     * @param $object
     * @return bool
     */
    public function deleteObject($object){

        $result = $this->s3Client->deleteObject([
            'Bucket' => $this->bucket,
            'Key' => $object,
        ]);

        if(isset($result['@metadata']['statusCode']) && $result['@metadata']['statusCode'] == 204){
            return true;
        }else{
            return false;
        }
    }

    /**
     * 删除多个文件
     * @param $objectArr
     * @return bool
     */
    public function batchDeleteObject($objectArr)
    {
        $keys = array();

        foreach ($objectArr as $item) {
            $keys[] = array('Key' => $item);
        }

        $s3 = $this->s3Client;

        $result = $s3->deleteObjects([
            'Bucket' => $this->bucket,
            'Delete' => ['Objects' => $keys]
        ]);

        if(isset($result["@metadata"]["statusCode"]) && $result["@metadata"]["statusCode"] == 200){
            return true;
        }else{
            return false;
        }
    }

    /**
     * 上传文件夹
     * @param $folderPath : 文件夹路径
     * @param $folderName : 指定文件夹名称
     * @return bool
     */
    public function uploadFolder($folderPath, $folderName = null)
    {
        if(!$folderName) {
            $folderName = basename($folderPath);
        }

        $keyPrefix = $folderName . '/';

        $options['params']['ACL'] = 'public-read';

        $this->s3Client->uploadDirectory($folderPath, $this->bucket, $keyPrefix , $options);

        return true;
    }

    /**
     * 删除文件夹
     * @param $folderPath
     * @return bool
     */
    public function deleteFolder($folderPath)
    {
        $folderName = basename($folderPath);

        $keyPrefix = $folderName . '/';

        $this->s3Client->deleteMatchingObjects($this->bucket, $keyPrefix);

        return true;
    }

}