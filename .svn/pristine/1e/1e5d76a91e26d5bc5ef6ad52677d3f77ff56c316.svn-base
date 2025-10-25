<?php
/**
 * Created by PhpStorm.
 * User: 12258
 * Date: 2022/10/24
 * Time: 15:52
 */

namespace App\Controller;
use App\Controller\BaseController;
use App\Services\WxMiniService;
use Hyperf\Di\Annotation\Inject;

class WxMiniController extends BaseController
{
    /**
     * @Inject()
     * @var WxMiniService
     */
    public $wxMiniService;
    /**
     * 获取accesstoken
     * @return \Psr\Http\Message\ResponseInterface
     */
    public function getAccessToken()
    {
        $requestData = $this->request->all();
        $result = $this->wxMiniService->getAccessToken($requestData);
        if ($result['status'] !== 'success') {
            return $this->failed($result['message'],$result['data']);
        }else{
            return $this->success($result['data'],$result['message']);
        }
    }
}
