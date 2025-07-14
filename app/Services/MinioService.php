<?php

namespace App\Services;
use App\Handlers\Minio\AwsHelper;
use App\Services\BaseService;
use Hyperf\DbConnection\Db;
use Hyperf\Di\Annotation\Inject;
use Aws\S3\Exception\S3Exception;
use Hyperf\HttpServer\Contract\RequestInterface;

class MinioService extends BaseService
{
    /**
     * @Inject()
     * @var RequestInterface
     */
    public $request;

    /**
     * 上传
     * @return array
     */
    public function upload(){
        $file = $this->request->file('upload_file');
        $extension = $file->getExtension(); //后缀名
        $size = $file->getSize(); //文件大小
        $name= $file->getClientFilename(); //文件原名称
        $infoArr = $file->getFileInfo();
        $newFileName = time().mt_rand(1,10000).'_'.$name;
        $path = 'public/'.$newFileName;
        $file->moveTo($path);
        try {
            $aws = new AwsHelper();
            $result = $aws->upLoadObject($path,$newFileName);
            unlink($path);
            if(!$result){
                return $this->baseFailed('上传失败',600);
            }
            return $this->baseSucceed('上传成功',['url'=>str_replace("http://","https://",$result['url']),'size'=>$size]);
        } catch (S3Exception $e) {
            unlink($path);
            return $this->baseFailed('上传失败：'.$e->getAwsErrorMessage(),600);
        }
        //5.计算样本文件的md5值返回给前端
//        $sampleApkMd5=md5_file($fullFileName);

//        $frontRes=["sample_hash"=>$sampleApkMd5,"size"=>$infoArr['size']];

//        var_dump($frontRes);die;
    }
}
