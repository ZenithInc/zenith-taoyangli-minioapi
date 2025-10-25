<?php
/**
 * Created by PhpStorm.
 * User: 12258
 * Date: 2022/10/24
 * Time: 15:52
 */

namespace App\Controller;
use App\Controller\BaseController;
use App\Services\MinioService;
use Hyperf\Di\Annotation\Inject;

class MinioController extends BaseController
{
    /**
     * @Inject()
     * @var MinioService
     */
    public $minioService;
    /**
     * 上传
     * @return \Psr\Http\Message\ResponseInterface
     */
    public function upload()
    {
        $result = $this->minioService->upload();
        if ($result['status'] !== 'success') {
            return $this->failed($result['message'],$result['data']);
        }else{
            return $this->success($result['data'],$result['message']);
        }
    }
}
