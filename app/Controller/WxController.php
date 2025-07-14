<?php

namespace App\Controller;
use App\Services\WxService;
use Hyperf\Di\Annotation\Inject;

class WxController extends BaseController
{
    /**
     * @Inject()
     * @var WxService
     */
    public $wxService;
    /**
     * 获取accesstoken
     * @return \Psr\Http\Message\ResponseInterface
     */
    public function getAccessToken()
    {
        $requestData = $this->request->all();
        $result = $this->wxService->getAccessToken($requestData);
        if (empty($result['status'])|| $result['status'] !== 'success') {
            return $this->failed($result['message'],$result['data']);
        }else{
            return $this->success($result['data'],$result['message']);
        }
    }
    /**
     * 获取accesstoken
     * @return \Psr\Http\Message\ResponseInterface
     */
    public function getJsApiTicket()
    {
        $requestData = $this->request->all();
        $result = $this->wxService->getJsApiTicket($requestData);
        if (empty($result['status'])|| $result['status'] !== 'success') {
            return $this->failed($result['message'],$result['data']);
        }else{
            return $this->success($result['data'],$result['message']);
        }
    }
}
